pub fn raindrops(n: u32) -> String {
    // define constants to define inputs
    const PLING_DIVISOR: u32 = 3;
    const PLANG_DIVISOR: u32 = 5;
    const PLONG_DIVISOR: u32 = 7;

    // function to determine if a number is divisible by another
    let divisible = |n, x| (n % x) == 0;

    // container to hold answer
    let mut raindrop = String::from("");

    // check for each divisible number and append if divisible
    if divisible(n, PLING_DIVISOR) {
        raindrop.push_str("Pling");
    }
    if divisible(n, PLANG_DIVISOR) {
        raindrop.push_str("Plang");
    }
    if divisible(n, PLONG_DIVISOR) {
        raindrop.push_str("Plong");
    }

    // if raindrop has not changed, convert the number to a string and use it
    if raindrop == "" {
        raindrop.push_str(&n.to_string());
    }

    raindrop
}
