extern crate base64;
use crate::errors::CryptoPalsError;

pub fn hex_decode(hextext: &str) -> Result<Vec<u8>, CryptoPalsError> {
    if hextext.len() % 2 != 0 {
        return Err(CryptoPalsError::CannotDecodeHex(
            "String length is not divisible by 2".to_owned(),
        ));
    }

    match (0..hextext.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&hextext[i..i + 2], 16))
        .collect::<Result<Vec<_>, _>>()
    {
        Ok(r) => Ok(r),
        Err(e) => Err(CryptoPalsError::CannotDecodeHex(e.to_string())),
    }
}

pub fn base64_encode(bytes: &[u8]) -> String {
    use base64::{engine::general_purpose, Engine as _};
    general_purpose::STANDARD_NO_PAD.encode(bytes)
}

pub fn base64_decode(data: &str) -> Result<Vec<u8>, CryptoPalsError> {
    use base64::{engine::general_purpose, Engine as _};
    let data = data
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>(); // Remove Whitespace
    let bytes = general_purpose::STANDARD.decode(data)?;

    Ok(bytes)
}

pub fn convert_hex_to_base64(hextext: &str) -> Result<String, CryptoPalsError> {
    let bytes = hex_decode(hextext)?;
    Ok(base64_encode(&bytes))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_can_convert_from_hex_to_base64() {
        let hex = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        let base64 = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";

        let parsed_base64 = convert_hex_to_base64(hex).unwrap();
        assert_eq!(parsed_base64, base64)
    }
}
