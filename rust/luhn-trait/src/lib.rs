pub trait Luhn {
    fn valid_luhn(&self) -> bool;
}

impl<T: ToString> Luhn for T {
    fn valid_luhn(&self) -> bool {
        let digits = self
            .to_string()
            .chars()
            .filter(|&digit| digit != ' ')
            .map(|digit| digit.to_digit(10))
            .collect::<Option<Vec<_>>>()
            .unwrap_or(Vec::new());

        if digits.len() < 2 {
            return false;
        }

        let sum = digits
            .iter()
            .rev()
            .enumerate()
            .map(|(i, &x)| match (i % 2 == 1, x) {
                (true, x) if x < 5 => x * 2,
                (true, x) => x * 2 - 9,
                (false, x) => x,
            })
            .sum::<u32>();

        sum % 10 == 0
    }
}
