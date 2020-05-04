/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    match calc_luhn_sum(code) {
        None => false,
        Some(sum) => sum % 10 == 0,
    }
}

/// Calculates the sum of all digits from a luhn
/// number as Option<32>.  Returns None if the
/// given code is invalid
fn calc_luhn_sum(code: &str) -> Option<u32> {
    let valid_digits = code
        .chars()
        .filter(|&digit| digit != ' ')
        .map(|digit| digit.to_digit(10))
        .collect::<Option<Vec<_>>>()?;

    if valid_digits.len() < 2 {
        return None;
    }

    let luhn_sum = valid_digits
        .into_iter()
        .rev()
        .enumerate()
        .map(|(i, x)| parse_digit(i, x))
        .sum::<u32>();
    Some(luhn_sum)
}

/// Convert each digit to the proper value required
/// to compute the luhn sum.
fn parse_digit(index: usize, digit: u32) -> u32 {
    match (index % 2 == 1, digit) {
        (true, x) if x < 5 => x * 2,
        (true, x) => x * 2 - 9,
        (false, x) => x,
    }
}
