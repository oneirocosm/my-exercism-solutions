use itertools::Itertools;

const ATBASH_GROUP: usize = 5;

/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    plain
        .chars()
        .filter_map(convert_char)
        .chunks(ATBASH_GROUP)
        .into_iter()
        .map(|chunk| chunk.collect::<String>())
        .join(" ")
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    cipher.chars().filter_map(convert_char).collect()
}

/// Converts a single char to its atbash equivalent while ignoring spaces
fn convert_char(c: char) -> Option<char> {
    match c.to_ascii_lowercase() {
        letter if letter.is_ascii_alphabetic() => Some(converter(letter)),
        num if num.is_numeric() => Some(num),
        _ => None,
    }
}

/// Performs the conversion of lowercase ascii chars
fn converter(c: char) -> char {
    let num_repr = c as u8;
    let offset = num_repr - b'a';
    let converted = b'z' - offset;
    converted as char
}
