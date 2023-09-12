use crate::challenges::challenge3::brute_force_single_character_xor;

pub fn find_encoded_string(data: &[Vec<u8>]) -> String {
    let mut score = 0;
    let mut text = String::new();

    for line in data {
        let (new_text, new_score, _) = brute_force_single_character_xor(line);

        if new_score > score {
            text = new_text;
            score = new_score;
        }
    }

    text
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::challenges::challenge1::hex_decode;
    use std::io::Read;

    #[test]
    fn can_find_single_character_xors_in_vecs() {
        let mut f = std::fs::File::open("challenge4.txt").unwrap();
        let mut data = String::new();

        f.read_to_string(&mut data).unwrap();

        let data: Vec<Vec<u8>> = data
            .split('\n')
            .map(|line| {
                line.strip_suffix('\r').unwrap_or(line);
                hex_decode(line).unwrap()
            })
            .collect();
        assert_eq!(
            find_encoded_string(&data),
            "Now that the party is jumping\n"
        );
    }
}
