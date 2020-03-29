use std::collections::HashMap;

const ALPHA_LWR: &str = "abcdefghijklmnopqrstuvwxyz";
const ALPHA_UPR: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const ALPHA_LEN: i8 = 26;

pub fn rotate(input: &str, key: i8) -> String {
    let cipher = construct_cipher(key);

    input
        .chars()
        .map(|letter| match cipher.get(&letter) {
            Some(rot_let) => *rot_let,
            None => letter,
        })
        .collect()
}

fn construct_cipher(key: i8) -> HashMap<char, char> {
    let mut cipher = HashMap::new();
    for (index, letter) in ALPHA_LWR.chars().enumerate() {
        let rot_ind = ((index as i8) + key).wrapping_rem_euclid(ALPHA_LEN) as usize;
        let rot_letter = ALPHA_LWR.chars().nth(rot_ind).unwrap();
        cipher.insert(letter, rot_letter);
    }
    for (index, letter) in ALPHA_UPR.chars().enumerate() {
        let rot_ind = ((index as i8) + key).wrapping_rem_euclid(ALPHA_LEN) as usize;
        let rot_letter = ALPHA_UPR.chars().nth(rot_ind).unwrap();
        cipher.insert(letter, rot_letter);
    }

    cipher
}
