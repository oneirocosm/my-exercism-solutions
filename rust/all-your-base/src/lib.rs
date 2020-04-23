#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

///
/// Convert a number between two bases.
///
/// A number is any slice of digits.
/// A digit is any unsigned integer (e.g. u8, u16, u32, u64, or usize).
/// Bases are specified as unsigned integers.
///
/// Return an `Err(.)` if the conversion is impossible.
/// The tests do not test for specific values inside the `Err(.)`.
///
///
/// You are allowed to change the function signature as long as all test still pass.
///
///
/// Example:
/// Input
///   number: &[4, 2]
///   from_base: 10
///   to_base: 2
/// Result
///   Ok(vec![1, 0, 1, 0, 1, 0])
///
/// The example corresponds to converting the number 42 from decimal
/// which is equivalent to 101010 in binary.
///
///
/// Notes:
///  * The empty slice ( "[]" ) is equal to the number 0.
///  * Never output leading 0 digits. However, your function must be able to
///     process input with leading 0 digits.
///
pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    if from_base <= 1 {
        return Err(Error::InvalidInputBase);
    }

    if to_base <= 1 {
        return Err(Error::InvalidOutputBase);
    }

    if let Some(&bad_value) = number.iter().find(|&digit| digit >= &from_base) {
        return Err(Error::InvalidDigit(bad_value));
    }

    let num_value = number
        .iter()
        .rev()
        .enumerate()
        .fold(0, |acc, (power, digit)| {
            acc + digit * from_base.pow(power as u32)
        });

    Ok((0..)
        .map(|power| to_base.pow(power))
        .take_while(|&x| x <= num_value)
        .collect::<Vec<u32>>()
        .iter()
        .rev()
        .scan(num_value, |state, &x| {
            let new_digit = *state / x;
            *state = *state % x;
            Some(new_digit)
        })
        .collect())
}
