use crate::errors::CryptoPalsError;

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

    for keysize_guess in 2..40 {
        let mut total_hamming_score = 0.0f64;
        let mut total_steps = 0;

        for i in (keysize_guess..data.len()).step_by(keysize_guess) {
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
            get_hamming_distance(s1.as_bytes(), s2.as_bytes()).unwrap(),
            37
        );

        let mut f = std::fs::File::open("challenge6.txt").unwrap();
        let mut data = String::new();

        f.read_to_string(&mut data).unwrap();
        let data = base64_decode(&data).unwrap();
        let keysize_guess = find_vinegere_keysize(&data).unwrap();
        assert_eq!(keysize_guess, 29);
    }
}
