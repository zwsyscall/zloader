import os
import sys
import yaml
import base64
from cryptography.hazmat.primitives.ciphers.aead import ChaCha20Poly1305, AESGCM
from cryptography.hazmat.primitives.asymmetric import rsa, padding
from cryptography.hazmat.primitives import hashes, serialization

config_path = "config.yaml"
STD_B32 = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ234567"
CROCK_B32 = b"0123456789ABCDEFGHJKMNPQRSTVWXYZ"
B32_TRANS = bytes.maketrans(STD_B32, CROCK_B32)

def encode_crockford(data: bytes) -> bytes:
    encoded = base64.b32encode(data)
    return encoded.translate(B32_TRANS).replace(b"=", b"")

def generate_missing(layer, layer_type, key_size, nonce_size=12):
    missing = False
    if 'key' not in layer and key_size > 0:
        layer['key'] = os.urandom(key_size).hex()
        print(f"[{layer_type}] Generated missing key:   {layer['key']}")
        missing = True
    if 'nonce' not in layer and nonce_size > 0:
        layer['nonce'] = os.urandom(nonce_size).hex()
        print(f"[{layer_type}] Generated missing nonce: {layer['nonce']}")
        missing = True
    
    if missing:
        print(f"--> Please update your YAML for {layer_type} to reuse these.\n")
    return layer

def main():
    input_path = "exempt\\payload.bin" 
    output_path = "exempt\\output.bin"

    if len(sys.argv) > 1:
        input_path = sys.argv[1]
    if len(sys.argv) > 2:
        output_path = sys.argv[2]

    print(f"Using input:  {input_path}")
    print(f"Using output: {output_path}")

    with open(config_path, 'r') as f:
        config = yaml.safe_load(f)

    with open(input_path, 'rb') as f:
        data = f.read()

    decoders = config.get('decoders', [])
    encoders = decoders[::-1] 

    print(f"Starting encoding pipeline. Total layers: {len(encoders)}")

    for layer in encoders:
        algo_type = layer.get('type')
        print(f"Applying layer: {algo_type}")

        if algo_type == "Base32":
            # Crockford base32
            data = encode_crockford(data)

        elif algo_type == "ChaCha20":
            # ChaCha20-Poly1305 (Requires 32-byte key, 12-byte nonce)
            layer = generate_missing(layer, algo_type, 32, 12)
            key = bytes.fromhex(layer['key'])
            nonce = bytes.fromhex(layer['nonce'])
            
            cipher = ChaCha20Poly1305(key)
            data = cipher.encrypt(nonce, data, associated_data=None)

        elif algo_type == "Aes256":
            layer = generate_missing(layer, algo_type, 32, 12)
            key = bytes.fromhex(layer['key'])
            nonce = bytes.fromhex(layer['nonce'])
            
            cipher = AESGCM(key)
            data = cipher.encrypt(nonce, data, associated_data=None)

        elif algo_type == "Aes128":
            layer = generate_missing(layer, algo_type, 16, 12)
            key = bytes.fromhex(layer['key'])
            nonce = bytes.fromhex(layer['nonce'])
            
            cipher = AESGCM(key)
            data = cipher.encrypt(nonce, data, associated_data=None)

        else:
            print(f"Unknown algorithm type: {algo_type}")
            sys.exit(1)

    with open(output_path, 'wb') as f:
        f.write(data)
    
    print(f"\nSuccess! Encoded payload written to {output_path}")

if __name__ == "__main__":
    main()