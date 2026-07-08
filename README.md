## zloader
This is a modular and easily extendable stager written in Rust with the core idea of being made up of building blocks so you can tailor your implant stagers to your needs without a bloated build system.

Out of the box, it supports multiple decoders, mappers and stagers. Take a look at the `config.yaml` file to get a hand on what's supported.

I included an MVP encoder, `scripts/encoder.py` but I highly recommend you make your own encoder that's more robust. 

As this is a public release, I've toned down some of the behaviour and removed some modules that are more effective. This is a neutered release mainly meant as a 'framework' for further development.

## Custom modules
To create new modules, whether it be allocators, decoders, mappers or stagers, you need to do the following:
 - Add the required dependencies to the `cargo.toml` file
 - Create a class in the correct folder under `src/`
 - Create a new template for setting up and running the module in the appropriate folder under `templates/` 
 - In the case of decoders, create an encoder and add it to your payload encoding pipeline

### Usage
To use this out of the box, modfy the `config.yaml`, run the `scripts/encoder.py` and finally, `scripts/build.py` to create your final binary.

**I will not provide any support for usage aside from this Readme.**

## Notice
⚠️  This repository contains source code for a malicious stager. This code is provided strictly for cybersecurity research, reverse engineering, malware analysis, and detection development purposes only. Do not use this code to attack any real devices. Unauthorized use is illegal and violates GitHub policy