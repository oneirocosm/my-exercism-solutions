pub fn is_armstrong_number(num: u32) -> bool {
    let len = num_digits(num);
    (0..)
        .map(|i| digit(num, i))
        .map(|i| i.pow(len as u32))
        .take(len)
        .sum::<u32>()
        == num
}

fn num_digits(num: u32) -> usize {
    // returns the number of digits in num
    (1..)
        .map(|x| 10_u32.pow(x))
        .take_while(|i| num > *i)
        .count()
        + 1
}

fn digit(num: u32, i: u32) -> u32 {
    // returns the "i"th digit of 10
    //     0:   1s place
    //     1:  10s place
    //     2: 100s place ... etc
    // division by power of 10 removes ealier digits
    // modding by 10 returns the next digit
    num / 10_u32.pow(i) % 10
}
