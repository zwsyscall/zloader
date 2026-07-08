use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;
use uuid::Uuid;

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("build_uuid.rs");
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Machine time is not reliable")
        .as_secs()
        .to_string();
    let username = env::var("USER")
        .or_else(|_| env::var("USERNAME"))
        .unwrap_or_else(|_| "unknown_user".to_string());
    // lazy but I don't really care
    let hostname = Command::new("hostname")
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_else(|_| {
            env::var("HOSTNAME")
                .or_else(|_| env::var("COMPUTERNAME"))
                .unwrap_or_else(|_| "unknown_host".to_string())
        });

    let build_id = Uuid::new_v4();

    fs::write(
        &dest_path,
        format!(
            "#[used]\nstatic BUILD_UUID: &str = \"{}\";\n#[used]\nstatic BUILD_WATERMARK: &str = \"{}@{}-{}\";\n#[used]\nstatic REPO_PATH: &str = \"https://github.com/zwsyscall/zloader\";",
            build_id,
            username,
            hostname,
            timestamp
        ),
    )
    .unwrap();

    println!("cargo:rerun-if-changed=build.rs");
}
