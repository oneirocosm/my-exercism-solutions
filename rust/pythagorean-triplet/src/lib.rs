use integer_sqrt::IntegerSquareRoot;
use std::collections::HashSet;

pub fn find(sum: u32) -> HashSet<[u32; 3]> {
    (1..=(sum / 2).integer_sqrt())
        .map(|m| (1..=(sum / (2 * m * m))).map(move |k| (k, m)))
        .flatten()
        .filter(|(k, m)| (sum - 2 * k * m * m) % (2 * k * m) == 0 && (sum - 2 * k * m * m) != 0)
        .filter_map(|(k, m)| {
            let n = (sum - 2 * k * m * m) / (2 * k * m);

            if n >= m {
                return None;
            }

            let a = k * (m * m - n * n);
            let b = k * 2 * m * n;
            let c = k * (m * m + n * n);

            let mut sides = [a, b, c];
            sides.sort();

            Some(sides)
        })
        .collect()
}
