use crate::errors::CryptoPalsError;

pub fn repeat_xor_encrypt(plaintext: &str, key: &str) -> Result<Vec<u8>, CryptoPalsError> {
    let decoded_plaintext = plaintext.as_bytes();
    let decoded_key = key.as_bytes();
    let cycling_decoded_key = decoded_key.iter().cycle();

    Ok(decoded_plaintext
        .iter()
        .zip(cycling_decoded_key)
        .map(|(c1, c2)| c1 ^ c2)
        .collect())
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::challenges::challenge1::hex_decode;

    #[test]
    fn it_can_do_repeating_xor_encryption() {
        let plaintext = "Burning 'em, if you ain't quick and nimble
I go crazy when I hear a cymbal";
        let ciphertext =
            "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272
a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f";

        let ciphertext = ciphertext.chars().filter(|c| !c.is_whitespace()).collect::<String>();
        let key = "ICE";

        let result = repeat_xor_encrypt(plaintext, key).unwrap();

        assert_eq!(result, hex_decode(&ciphertext).unwrap());
    }
}
