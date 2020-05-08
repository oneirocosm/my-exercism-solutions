#[derive(Debug, PartialEq)]
pub enum Error {
    SpanTooLong,
    InvalidDigit(char),
}

pub fn lsp(string_digits: &str, span: usize) -> Result<u64, Error> {
    if span == 0 {
        return Ok(1);
    }

    let digits = string_digits
        .chars()
        .map(|digit| {
            digit
                .to_digit(10)
                .map(|digit| digit as u64)
                .ok_or(Error::InvalidDigit(digit))
        })
        .collect::<Result<Vec<u64>, Error>>()?;

    digits
        .windows(span)
        .map(|sequence| sequence.iter().product())
        .max()
        .ok_or(Error::SpanTooLong)
}
