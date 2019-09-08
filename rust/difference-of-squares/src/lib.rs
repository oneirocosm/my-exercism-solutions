pub fn square_of_sum(n: u32) -> u32 {
    // sum of first n is (n**2 + n)/2 so squaring that gives this formula
    (n.pow(4) + 2 * n.pow(3) + n.pow(2)) / 4
}

pub fn sum_of_squares(n: u32) -> u32 {
    // a web search gave me this formula
    (2 * n.pow(3) + 3 * n.pow(2) + n) / 6
}

pub fn difference(n: u32) -> u32 {
    square_of_sum(n) - sum_of_squares(n)
}
