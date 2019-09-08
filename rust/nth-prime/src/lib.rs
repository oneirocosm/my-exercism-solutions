use num_integer::sqrt;

pub fn nth(n: u32) -> u32 {
    (2..).filter(|i| is_prime(*i)).nth(n as usize).unwrap()
}

fn divisible(x: u32, y: u32) -> bool {
    // check if x is divisible by y
    (x % y) == 0
}

fn is_prime(i: u32) -> bool {
    !(2..=sqrt(i)).any(|j| divisible(i, j))
}
