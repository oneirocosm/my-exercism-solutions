use itertools::Itertools;
use modinverse::modinverse;

/// While the problem description indicates a return status of 1 should be returned on errors,
/// it is much more common to return a `Result`, so we provide an error type for the result here.
#[derive(Debug, Eq, PartialEq)]
pub enum AffineCipherError {
    NotCoprime(i32),
}

/// Encodes the plaintext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn encode(plaintext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    let _a_inv = modinverse(a, 26).ok_or(AffineCipherError::NotCoprime(a))?;

    Ok(plaintext
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .map(|c| encode_char(c, a, b))
        .chunks(5)
        .into_iter()
        .map(|chunk| chunk.collect::<String>())
        .join(" "))
}

/// Decodes the ciphertext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn decode(ciphertext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    let a_inv = modinverse(a, 26).ok_or(AffineCipherError::NotCoprime(a))?;
    println!("{}", a_inv);

    Ok(ciphertext
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .map(|c| decode_char(c, a_inv, b))
        .join(""))
}

fn encode_char(input: char, a: i32, b: i32) -> char {
    if input.is_ascii_alphabetic() {
        let num_rep = (input.to_ascii_lowercase() as i32) - ('a' as i32);
        let encode_num_rep = (a * num_rep + b) % 26;
        ((encode_num_rep as u8) + b'a') as char
    } else {
        input
    }
}

fn decode_char(input: char, a_inv: i32, b: i32) -> char {
    if input.is_ascii_alphabetic() {
        let num_rep = (input as i32) - (b'a' as i32);
        let decode_num_rep = (a_inv * (num_rep - b)).rem_euclid(26);
        ((decode_num_rep as u8) + b'a') as char
    } else {
        input
    }
}
