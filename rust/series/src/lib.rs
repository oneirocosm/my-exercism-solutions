pub fn series(digits: &str, len: usize) -> Vec<String> {
    match len {
        0 => vec![String::new(); digits.len() + 1],
        n => digits
            .chars()
            .collect::<Vec<_>>()
            .windows(n)
            .map(|window| window.iter().collect::<String>())
            .collect::<Vec<_>>(),
    }
}
