use std::collections::HashMap;

#[macro_use]
extern crate unic_char_range;

#[macro_use]
extern crate lazy_static;

static ALPHA_SIZE: usize = 26;

lazy_static! {
    static ref ALPHA_2_NUM: HashMap<char, usize> =
        chars!('a'..='z')
            .iter()
            .enumerate()
            .fold(HashMap::new(), |mut alpha, (val, ch)| {
                alpha.insert(ch, val);
                alpha
            });

    static ref NUM_2_ALPHA: HashMap<usize, char> =
        chars!('a'..='z')
            .iter()
            .enumerate()
            .fold(HashMap::new(), |mut alpha, (val, ch)| {
                alpha.insert(val, ch);
                alpha
            });
}

pub fn encode(key: &str, s: &str) -> Option<String> {
    if key.is_empty() {
        return None;
    }

    let key_vals = convert_alpha_to_num(key)?;
    let s_vals = convert_alpha_to_num(s)?;

    s_vals
        .iter()
        .zip(key_vals.iter().cycle())
        .map(|(x, y)| (**x + **y) % ALPHA_SIZE)
        .map(|new_val| NUM_2_ALPHA.get(&new_val))
        .collect()
}

pub fn decode(key: &str, s: &str) -> Option<String> {
    if key.is_empty() {
        return None;
    }

    let key_vals = convert_alpha_to_num(key)?;
    let s_vals = convert_alpha_to_num(s)?;

    s_vals
        .iter()
        .zip(key_vals.iter().cycle())
        .map(|(x, y)| (ALPHA_SIZE + **x - **y) % ALPHA_SIZE)
        .map(|new_val| NUM_2_ALPHA.get(&new_val))
        .collect()
}

pub fn encode_random(s: &str) -> (String, String) {
    let mut key: Vec<char> = Vec::new();

    for _ in 0..s.len().max(100) {
        let letter_val = rand::random::<usize>() % ALPHA_SIZE;
        let letter = NUM_2_ALPHA.get(&letter_val).unwrap();
        key.push(*letter);
    }
    let key = key.iter().collect::<String>();

    match encode(&key, s) {
        None => (key, "".into()),
        Some(encoding) => (key, encoding),
    }
}

fn convert_alpha_to_num(input: &str) -> Option<Vec<&usize>> {
    input
        .chars()
        .map(|c| ALPHA_2_NUM.get(&c))
        .collect::<Option<Vec<&usize>>>()
}
