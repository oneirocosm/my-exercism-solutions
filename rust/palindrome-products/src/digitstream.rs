#[derive(Debug, Clone, Copy, Hash)]
pub struct DigitStream {
    value: u64,
    length: usize,
    f_iter: usize,
    r_iter: usize,
}

impl DigitStream {
    pub fn new(value: u64) -> Self {
        let length = Self::calc_num_digits(value);
        Self {
            value,
            length,
            f_iter: 0,
            r_iter: length,
        }
    }

    fn calc_num_digits(value: u64) -> usize {
        let mut count = 0;
        let mut number = value;
        while number != 0 {
            number /= 10;
            count += 1;
        }
        count
    }
}

impl Iterator for DigitStream {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let iter = self.f_iter as u32;
        self.f_iter += 1;

        if iter < self.length as u32 {
            Some(self.value / 10u64.pow(iter) % 10)
        } else {
            None
        }
    }
}

impl DoubleEndedIterator for DigitStream {
    fn next_back(&mut self) -> Option<Self::Item> {
        if let Some(iter) = self.r_iter.checked_sub(1) {
            self.r_iter = iter;
            Some(self.value / 10u64.pow(iter as u32) % 10)
        } else {
            None
        }
    }
}
