trait Multiple {
    fn has_factor_in(&self, factors: &[u32]) -> bool;
    fn is_factor_of(&self, n: u32) -> bool;
}

impl Multiple for u32 {
    fn is_factor_of(&self, n: u32) -> bool {
        match *self {
            0 => false,
            _ => (n % *self) == 0,
        }
    }

    fn has_factor_in(&self, factors: &[u32]) -> bool {
        factors.iter().any(|x| x.is_factor_of(*self))
    }
}

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    // if x  is a factor of y, then y is                      a multiple of x
    // if i has a factor in y, then y has some number that is a multiple of i
    (1..limit).filter(|i| i.has_factor_in(factors)).sum()
}
