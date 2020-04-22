use maplit::hashmap;
use std::collections::HashMap;

struct PeriodSeparatedNumber {
    value: u64,
}

impl PeriodSeparatedNumber {
    fn from(value: u64) -> Self {
        PeriodSeparatedNumber { value }
    }
}

impl Iterator for PeriodSeparatedNumber {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        let new_period = self.value % 1000;
        let new_value = self.value / 1000;
        self.value = new_value;

        match (new_value, new_period) {
            (0, 0) => None,
            (_, value) => Some(value),
        }
    }
}

pub fn encode(n: usize) -> String {
    let period_scales = [
        "",
        "thousand",
        "million",
        "billion",
        "trillion",
        "quadrillion",
        "quintillion",
    ];

    if n == 0 {
        return "zero".into();
    }

    PeriodSeparatedNumber::from(n as u64)
        .map(parse_period)
        .enumerate()
        .map(
            |(period_num, period)| match (period.as_str(), period_scales[period_num]) {
                ("", _) => "".into(),
                (per_detail, "") => per_detail.into(),
                (per_detail, per_scale) => format!("{} {}", per_detail, per_scale),
            },
        )
        .collect::<Vec<String>>()
        .into_iter()
        .rev()
        //.collect()
        .fold(String::new(), |acc, next| {
            if next.as_str() == "" {
                acc
            } else if acc.as_str() == "" {
                next
            } else {
                format!("{} {}", acc.as_str(), next.as_str())
            }
        })
}

fn parse_period(n: u64) -> String {
    match (parse_hundreds(n).as_str(), parse_tens_and_ones(n).as_str()) {
        ("", "") => "".into(),
        ("", tens) => tens.into(),
        (hundr, "") => hundr.into(),
        (hundr, tens) => format!("{} {}", hundr, tens),
    }
}

fn parse_hundreds(n: u64) -> String {
    let ones: HashMap<u64, &str> = hashmap! {
        1 => "one",
        2 => "two",
        3 => "three",
        4 => "four",
        5 => "five",
        6 => "six",
        7 => "seven",
        8 => "eight",
        9 => "nine",
    };
    let hundreds_place = n / 100;
    match hundreds_place {
        0 => "".into(),
        x => format!("{} hundred", ones[&x]),
    }
}

fn parse_tens_and_ones(n: u64) -> String {
    // remove hundreds place
    let trunc = n - (n / 100) * 100;

    let ones: HashMap<u64, &str> = hashmap! {
        1 => "one",
        2 => "two",
        3 => "three",
        4 => "four",
        5 => "five",
        6 => "six",
        7 => "seven",
        8 => "eight",
        9 => "nine",
    };

    let tens: HashMap<u64, &str> = hashmap! {
        1 => "ten",
        2 => "twenty",
        3 => "thirty",
        4 => "forty",
        5 => "fifty",
        6 => "sixty",
        7 => "seventy",
        8 => "eighty",
        9 => "ninety",
    };

    let tens_place = trunc / 10;
    let ones_place = trunc % 10;

    match (tens_place, ones_place) {
        (0, 0) => "".into(),
        (0, y) => (*ones.get(&y).unwrap()).into(),
        (x, 0) => (*tens.get(&x).unwrap()).into(),
        (1, 1) => "eleven".into(),
        (1, 2) => "twelve".into(),
        (1, 3) => "thirteen".into(),
        (1, 4) => "fourteen".into(),
        (1, 5) => "fifteen".into(),
        (1, 6) => "sixteen".into(),
        (1, 7) => "seventeen".into(),
        (1, 8) => "eighteen".into(),
        (1, 9) => "nineteen".into(),
        (x, y) => format!("{}-{}", tens[&x], ones[&y]),
    }
}
