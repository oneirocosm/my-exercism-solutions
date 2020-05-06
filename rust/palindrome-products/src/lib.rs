use maplit::hashset;
use std::cmp::Ordering;
use std::collections::HashSet;

mod digitstream;
use digitstream::DigitStream;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Palindrome {
    pairs: HashSet<(u64, u64)>,
}

impl Palindrome {
    pub fn new(a: u64, b: u64) -> Palindrome {
        let pairs = if a == b {
            hashset! {(a,b)}
        } else {
            hashset! {(a,b), (b,a)}
        };
        Palindrome { pairs }
    }

    pub fn value(&self) -> u64 {
        let pair = self.pairs.iter().next().unwrap();
        pair.0 * pair.1
    }

    pub fn insert(&mut self, a: u64, b: u64) {
        self.pairs.insert((a, b));

        if a != b {
            self.pairs.insert((b, a));
        }
    }

    fn update_with_min(&mut self, other: (u64, u64)) {
        let other_val = other.0 * other.1;
        match self.value().cmp(&other_val) {
            Ordering::Greater => self.pairs = hashset!(other, (other.1, other.0)),
            Ordering::Less => {}
            Ordering::Equal => self.insert(other.0, other.1),
        }
    }

    fn update_with_max(&mut self, other: (u64, u64)) {
        let other_val = other.0 * other.1;
        match self.value().cmp(&other_val) {
            Ordering::Greater => {}
            Ordering::Less => self.pairs = hashset!(other, (other.1, other.0)),
            Ordering::Equal => self.insert(other.0, other.1),
        }
    }
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    (min..=max)
        .map(|i| (min..=i).map(move |j| (i, j)))
        .flatten()
        .filter(|(i, j)| palindrome_possible(*i, *j))
        .fold(None, |acc, pal_factors| {
            let (fact0, fact1) = pal_factors;
            match acc {
                None => Some((Palindrome::new(fact0, fact1), Palindrome::new(fact0, fact1))),
                Some((mut min, mut max)) => {
                    min.update_with_min(pal_factors);
                    max.update_with_max(pal_factors);
                    Some((min, max))
                }
            }
        })
}

fn palindrome_possible(i: u64, j: u64) -> bool {
    let forward = DigitStream::new(i * j);
    let reverse = forward.rev();

    forward.zip(reverse).all(|(x, y)| x == y)
}
