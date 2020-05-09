pub fn number(user_number: &str) -> Option<String> {
    let parsed = user_number
        .trim_start_matches(|c: char| c.is_ascii_punctuation())
        .trim_start_matches('1')
        .chars()
        .filter(|c| c.is_digit(10))
        .collect::<String>();

    check_not_1_or_0(&parsed, 0)?;
    check_not_1_or_0(&parsed, 3)?;

    match parsed {
        number if number.len() == 10 => Some(number),
        _ => None,
    }
}

fn check_not_1_or_0(number: &str, index: usize) -> Option<()> {
    match number.chars().nth(index) {
        None | Some('0') | Some('1') => None,
        _ => Some(()),
    }
}
