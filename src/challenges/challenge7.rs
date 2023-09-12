use crate::errors::CryptoPalsError;
use openssl::symm::{decrypt, Cipher};

pub fn decrypt_using_aes(data: &[u8], priv_key: &[u8]) -> Result<Vec<u8>, CryptoPalsError> {
    let cipher = Cipher::aes_128_cbc();
    let iv = b"\x00\x01\x02\x03\x04\x05\x06\x07\x00\x01\x02\x03\x04\x05\x06\x07";

    println!("{:?}", priv_key);
    println!("{:?}", Some(iv));
    println!("{:?}", data);

    let ciphertext = decrypt(cipher, priv_key, Some(iv), data).unwrap();

    Ok(ciphertext)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::challenges::challenge1::base64_decode;
    use std::io::Read;

    fn read_challenge7_data() -> Vec<u8> {
        let mut f = std::fs::File::open("challenge7.txt").expect("could not open challenge file");
        let mut data = String::new();

        f.read_to_string(&mut data).unwrap();
        base64_decode(&data).unwrap()
    }

    #[test]
    fn it_can_decrypt_messages() {
        let data = read_challenge7_data();
        let aes_key: Vec<u8> = "YELLOW SUBMARINE".bytes().collect();

        assert_eq!(aes_key.len(), 16);
        assert_eq!(data.len() % 16, 0);
        assert_eq!(data.len(), 2880);

        let decrypted_value = decrypt_using_aes(&data, &aes_key).unwrap();
        let decrypted_value: String = std::str::from_utf8(&decrypted_value).unwrap().to_string();
        assert_eq!(decrypted_value, "Hello world");
    }
}
