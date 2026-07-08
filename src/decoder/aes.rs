use aes_gcm::{
    Aes128Gcm, Aes256Gcm, Key, Nonce,
    aead::{Aead, AeadCore, KeyInit, OsRng},
};

use super::Decoder;

pub struct Aes256 {
    key: [u8; 32],
    nonce: [u8; 12],
}

impl Decoder for Aes256 {
    type Error = aes_gcm::aead::Error;
    fn decode(&mut self, data: &[u8]) -> Result<Vec<u8>, Self::Error> {
        let cipher = Aes256Gcm::new(&self.key.into());
        cipher.decrypt(&self.nonce.into(), data)
    }
}

pub struct Aes128 {
    key: [u8; 16],
    nonce: [u8; 12],
}

impl Decoder for Aes128 {
    type Error = aes_gcm::aead::Error;
    fn decode(&mut self, data: &[u8]) -> Result<Vec<u8>, Self::Error> {
        let cipher = Aes128Gcm::new(&self.key.into());
        cipher.decrypt(&self.nonce.into(), data)
    }
}
