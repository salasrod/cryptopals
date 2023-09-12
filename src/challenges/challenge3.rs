// ETAOIN SHRDLU

pub fn brute_force_single_character_xor(xor_hex: &[u8]) -> (String, i32, u8) {
    const COMMON: &str = "ETAOIN SHRDLU";
    let mut score = 0;
    let mut plaintext = String::new();
    let mut key: u8 = 0;

    for hex_digit in u8::MIN..u8::MAX {
        let new_plaintext = match String::from_utf8(xor_hex.iter().map(|b| b ^ hex_digit).collect())
        {
            Ok(s) => s,
            Err(_) => continue,
        };

        let mut new_score = 0;
        for c in new_plaintext.to_uppercase().chars() {
            if COMMON.contains(c) {
                new_score += 1;
            }
        }

        if score < new_score {
            score = new_score;
            plaintext = new_plaintext;
            key = hex_digit;
        }
    }

    (plaintext, score, key)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::challenges::challenge1::hex_decode;

    #[test]
    fn it_can_detect_english_from_a_bruteforced_xor() {
        let encoded_ciphertext =
            "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
        let xor_hex = hex_decode(encoded_ciphertext).unwrap();

        let (detected_string, detected_score, _) = brute_force_single_character_xor(&xor_hex);
        assert_eq!(detected_string, "Cooking MC's like a pound of bacon");
        assert_eq!(detected_score, 23);
    }
}
