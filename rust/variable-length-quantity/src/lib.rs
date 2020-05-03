#[derive(Debug, PartialEq)]
pub enum Error {
    IncompleteNumber,
    Overflow,
}

struct VLQ {
    value: u32,
    fwd_ind: usize,
    rev_ind: usize,
}

impl VLQ {
    fn new(value: u32) -> Self {
        VLQ {
            value,
            fwd_ind: 0,
            rev_ind: Self::max_ind(value),
        }
    }

    fn max_ind(value: u32) -> usize {
        match (0..32).filter(|i| value & (0x1u32 << i) != 0).last() {
            None => 1,
            Some(i) => i / 7 + 1,
        }
    }
}

impl Iterator for VLQ {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        let shifted = self.value >> 7 * self.fwd_ind;
        let new = (shifted & 0x7F) as u8;
        let old_ind = self.fwd_ind;
        self.fwd_ind += 1;

        match (new, old_ind) {
            (0, 0) => Some(0),
            (0, _) => None,
            (new, 0) => Some(new),
            (new, _) => Some(new | 0x80),
        }
    }
}

impl DoubleEndedIterator for VLQ {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.rev_ind = self.rev_ind.checked_sub(1)?;
        let shifted = self.value >> 7 * self.rev_ind;
        let new = (shifted & 0x7F) as u8;

        match (new, self.rev_ind) {
            (new, 0) => Some(new),
            (new, _) => Some(new | 0x80),
        }
    }
}

/// Convert a list of numbers to a stream of bytes encoded with variable length encoding.
pub fn to_bytes(values: &[u32]) -> Vec<u8> {
    values
        .iter()
        .map(|&val| VLQ::new(val).into_iter().rev())
        .flatten()
        .collect()
}

/// Given a stream of bytes, extract all numbers which are encoded in there.
pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, Error> {
    let mut stream =
        bytes
            .into_iter()
            .map(|&val| val as u32)
            .try_fold(vec![0u32], |mut acc, x| {
                let old: u32 = acc.pop().unwrap();
                match checking_shl(&old, 7) {
                    None => return Err(Error::Overflow),
                    Some(shifted) => acc.push(shifted | (x & 0x7F)),
                }
                if (x & 0x80) == 0 {
                    acc.push(0);
                }
                Ok(acc)
            })?;

    if stream == vec![0u32] {
        return Err(Error::IncompleteNumber);
    }

    match stream.pop() {
        Some(0) => Ok(stream),
        _ => Err(Error::IncompleteNumber),
    }
}

/// Left shift that retuns none if there is any overflow
fn checking_shl(x: &u32, y: u32) -> Option<u32> {
    let result = x.checked_shl(y)?;
    let original = result.checked_shr(y)?;

    if original == *x {
        Some(result)
    } else {
        None
    }
}
