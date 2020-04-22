type Factorization = dyn Iterator<Item = u64>;

pub fn factors(n: u64) -> Vec<u64> {
    // separates n into its prime factor decomposition
    (2..)
        .scan(n, |unfactored, i| divide_out_all(unfactored, i))
        .flatten()
        .collect()
}

fn divide_out_all(unfactored: &mut u64, p: u64) -> Option<Box<Factorization>> {
    // divides all factors of p out of unfactored and returns them as
    // an Option<Box<Iterator>.  unfactored changes to the result after
    // these factors are divided out
    // For example:
    //    (unfactored = 35) and (p = 5) => (5      ) and (unfactored = 7)
    //    (unfactored = 8)  and (p = 2) => (2, 2, 2) and (unfactored = 1)
    //    (unfactored = 5)  and (p = 3) => (       ) and (unfactored = 5)
    let valuation = times_divisible_by(*unfactored, p);
    *unfactored /= p.pow(valuation as u32);

    match (valuation, *unfactored) {
        (0, 1) => None,
        (0, _) => Some(Box::new(std::iter::empty::<u64>())),
        (_, _) => Some(Box::new(std::iter::repeat(p).take(valuation))),
    }
}

fn times_divisible_by(unfactored: u64, p: u64) -> usize {
    // calculates the p-adic valuation of the remaining unfactored number
    // in other words, what is the highest power of p that divides unfactored
    (1..)
        .map(|i| p.pow(i))
        .take_while(|i| is_multiple_of(unfactored, *i))
        .count()
}

fn is_multiple_of(n: u64, i: u64) -> bool {
    // returns true if n is a multiple of i
    // and false otherwise
    (n % i) == 0
}
