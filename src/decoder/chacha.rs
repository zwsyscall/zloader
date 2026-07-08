use chacha20poly1305::{ChaCha20Poly1305, KeyInit, aead::Aead};

use super::Decoder;

pub struct ChaCha20 {
    pub key: [u8; 32],
    pub nonce: [u8; 12],
}

impl Decoder for ChaCha20 {
    type Error = chacha20poly1305::aead::Error;

    fn decode(&mut self, data: &[u8]) -> Result<Vec<u8>, Self::Error> {
        let cipher = ChaCha20Poly1305::new(&self.key.into());
        cipher.decrypt(&self.nonce.into(), data)
    }
}
