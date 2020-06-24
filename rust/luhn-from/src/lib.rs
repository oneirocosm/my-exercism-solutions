pub struct Luhn {
    digits: Vec<u32>,
}

impl Luhn {
    /// Checks if the luhn number is valid
    pub fn is_valid(&self) -> bool {
        if self.digits.len() < 2 {
            return false;
        }

        self.calc_luhn_sum() % 10 == 0
    }

    /// Compute sum used by the luhn algorithm
    fn calc_luhn_sum(&self) -> u32 {
        self.digits
            .iter()
            .rev()
            .enumerate()
            .map(|(i, &x)| Self::parse_digit(i, x))
            .sum::<u32>()
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
}

impl<T: ToString> From<T> for Luhn {
    fn from(input: T) -> Self {
        let digits = input
            .to_string()
            .chars()
            .filter(|&digit| digit != ' ')
            .map(|digit| digit.to_digit(10))
            .collect::<Option<Vec<_>>>()
            .unwrap_or(Vec::new());

        Self { digits }
    }
}
