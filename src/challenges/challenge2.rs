use std::iter::zip;

use crate::errors::CryptoPalsError;

pub fn xor_buffer(buf1: &[u8], buf2: &[u8]) -> Result<Vec<u8>, CryptoPalsError> {
    if buf1.len() != buf2.len() {
        return Err(CryptoPalsError::DifferentSizedBuffers);
    }

    Ok(zip(buf1, buf2).map(|(b1, b2)| b1 ^ b2).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_can_xor_two_same_length_buffers() {
        use crate::challenges::challenge1::hex_decode;

        let buf1 = hex_decode("1c0111001f010100061a024b53535009181c").unwrap();
        let buf2 = hex_decode("686974207468652062756c6c277320657965").unwrap();
        let result = hex_decode("746865206b696420646f6e277420706c6179").unwrap();

        let buf3 = xor_buffer(&buf1, &buf2).unwrap();

        assert_eq!(buf3, result);
    }
}
