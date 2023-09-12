use crate::errors::CryptoPalsError;

use super::challenge3::brute_force_single_character_xor;

pub fn get_hamming_distance(s1: &[u8], s2: &[u8]) -> Result<u32, CryptoPalsError> {
    if s1.len() != s2.len() {
        return Err(CryptoPalsError::DifferentSizedBuffers);
    }

    Ok(s1
        .iter()
        .zip(s2)
        .map(|(c1, c2)| (c1 ^ c2).count_ones())
        .sum())
}

pub fn find_vinegere_keysize(data: &[u8]) -> Result<usize, CryptoPalsError> {
    let mut keysize = 0;
    let mut edit_distance = f64::MAX;

    for keysize_guess in 2..=40 {
        let mut total_hamming_score = 0.0f64;
        let mut total_steps = 0;

        for i in (keysize_guess..data.len() - 1).step_by(keysize_guess) {
            if i + keysize_guess < data.len() {
                let prev = &data[i - keysize_guess..i];
                let curr = &data[i..i + keysize_guess];

                let hamming_score = get_hamming_distance(prev, curr)?;

                // Assume we are running 64 bits.
                let hamming_score_usize = usize::try_from(hamming_score).unwrap();
                total_hamming_score += hamming_score_usize as f64 / keysize_guess as f64;
                total_steps += 1;
            }
        }

        let normalized_edit_distance = total_hamming_score / total_steps as f64;
        if normalized_edit_distance < edit_distance {
            keysize = keysize_guess;
            edit_distance = normalized_edit_distance;
        }
    }
    Ok(keysize)
}

pub fn bruteforce_vinegere(ciphertext: &[u8]) -> Result<String, CryptoPalsError> {
    let keysize = find_vinegere_keysize(ciphertext)?;
    let mut key_bytes: Vec<u8> = vec![];

    let mut idx;
    let mut ith_bytes: Vec<u8> = vec![];

    for i in 0..keysize {
        idx = i;
        ith_bytes.clear();
        while idx < ciphertext.len() {
            ith_bytes.push(ciphertext[idx]);
            idx += keysize;
        }

        let (_decrypted, _score, key_i) = brute_force_single_character_xor(&ith_bytes);
        key_bytes.push(key_i);
    }

    let key: String = key_bytes.iter().map(|&b| b as char).collect();

    Ok(key)
}

#[cfg(test)]
mod tests {
    use std::io::Read;

    use crate::challenges::challenge1::base64_decode;

    use super::*;

    #[test]
    fn it_correctly_assesses_hamming_distance() {
        let s1 = "this is a test";
        let s2 = "wokka wokka!!!";

        assert_eq!(
            get_hamming_distance(s1.as_bytes(), s2.as_bytes())
                .expect("could not get hamming distance"),
            37
        );
    }

    fn read_challenge6_data() -> Vec<u8> {
        let mut f = std::fs::File::open("challenge6.txt").expect("could not open challenge file");
        let mut data = String::new();

        f.read_to_string(&mut data).unwrap();
        base64_decode(&data).unwrap()
    }

    #[test]
    fn it_correctly_finds_the_vinegere_keysize() {
        let data = read_challenge6_data();

        let keysize_guess =
            find_vinegere_keysize(&data).expect("could not find the Vinegere keysize");
        assert_eq!(keysize_guess, 29);
    }

    #[test]
    fn it_correctly_cracks_vinegere_ciphers() {
        let data = read_challenge6_data();

        let decrypted_text =
            bruteforce_vinegere(&data).expect("could not brute force the vinegere");

        assert_eq!(decrypted_text, "Terminator X: Bring the noise");
    }
}
