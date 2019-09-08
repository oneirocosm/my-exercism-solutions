pub fn square(s: u32) -> u64 {
    match s {
        0 => panic!("Square must be between 1 and 64"),
        i if i > 64 => panic!("Square must be between 1 and 64"),
        _ => 2_u64.pow(s - 1),
    }
}

pub fn total() -> u64 {
    // properties of sums of geometric series can make this simpler, but
    // then I wouldn't get to show off Rust's great features :P
    (1..=64).map(square).sum()
}
