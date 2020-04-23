pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    Sieve::new(upper_bound).collect()
}

struct Sieve {
    upper_bound: u64,
    current_prime: Option<u64>,
    composites: Vec<u64>,
}

impl Sieve {
    fn new(upper_bound: u64) -> Self {
        Sieve {
            upper_bound,
            current_prime: None,
            composites: Vec::new(),
        }
    }
}

impl Iterator for Sieve {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(old) = self.current_prime {
            if let Some(new) = ((old + 1)..=self.upper_bound).find(|x| !self.composites.contains(x))
            {
                let mut new_composites = (2..=self.upper_bound / 2)
                    .map(|x| x * new)
                    .collect::<Vec<u64>>();
                self.composites.append(new_composites.as_mut());
                self.current_prime = Some(new);
                Some(new)
            } else {
                None
            }
        } else if self.upper_bound < 2 {
            None
        } else {
            let mut new_composites = (2..=self.upper_bound / 2)
                .map(|x| x * 2)
                .collect::<Vec<u64>>();
            self.composites.append(new_composites.as_mut());
            self.current_prime = Some(2);
            Some(2)
        }
    }
}
