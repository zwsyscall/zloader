use super::Decoder;

pub struct Base32 {}

impl Decoder for Base32 {
    type Error = ();

    fn decode(&mut self, data: &[u8]) -> Result<Vec<u8>, Self::Error> {
        let string_form = String::from_utf8_lossy(data);
        base32::decode(base32::Alphabet::Crockford, &string_form.to_string()).ok_or(())
    }
}
