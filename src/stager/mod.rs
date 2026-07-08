mod http;
pub use http::HttpStager;
mod meterpreter;
pub use meterpreter::MeterpreterStager;

// Supports multiple backends, some with encryption, some without, some HTTP(s), some DNS etc.
pub trait Stager {
    type Error: std::fmt::Debug;
    fn get(&mut self) -> Result<Vec<u8>, Self::Error>;
}
