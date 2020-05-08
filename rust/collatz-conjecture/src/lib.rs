pub fn collatz(n: u64) -> Option<u64> {
    match n {
        0 => None,
        1 => Some(0),
        n if n % 2 == 0 => Some(collatz(n / 2)? + 1),
        n => Some(collatz(3 * n + 1)? + 1),
    }
}
