#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    if num == 0 {
        return None;
    }

    match Factors::from(num).into_iter().sum::<u64>() {
        x if x < num => Some(Classification::Deficient),
        x if x > num => Some(Classification::Abundant),
        _ => Some(Classification::Perfect),
    }
}

struct Factors {
    value: u64,
    current: Option<u64>,
}

impl Factors {
    fn from(value: u64) -> Self {
        Factors {
            value,
            current: None,
        }
    }
}

impl Iterator for Factors {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        let new = match (self.value, self.current) {
            (0, _) => None,
            (1, _) => None,
            (_, Some(old)) => ((old + 1)..self.value)
                .filter(|x| self.value % x == 0)
                .take(1)
                .last(),
            (_, None) => Some(1),
        };
        self.current = new;
        new
    }
}
