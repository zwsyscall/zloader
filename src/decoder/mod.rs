mod aes;
mod base32;
mod chacha;

pub use aes::*;
pub use base32::*;
pub use chacha::*;

pub trait Decoder {
    type Error: std::fmt::Debug;
    fn decode(&mut self, data: &[u8]) -> Result<Vec<u8>, Self::Error>;
}
