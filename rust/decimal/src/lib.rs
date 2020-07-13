use itertools::Itertools;
use num_bigint::BigInt;
use std::cmp::Ordering;
use std::ops::{Add, Mul, Neg, Sub};

/// Type implementing arbitrary-precision decimal arithmetic
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Decimal {
    digits: BigInt,
    sig_figs: u32,
}

impl Decimal {
    pub fn try_from(input: &str) -> Option<Self> {
        let negative = input.starts_with('-');
        let (whole_repr, mut dec_repr) = if input.contains('.') {
            input.split('.').collect_tuple()?
        } else {
            (input, "")
        };
        dec_repr = dec_repr.trim_end_matches('0');

        let sig_figs = dec_repr.len() as u32;
        let whole = whole_repr.parse::<BigInt>().ok()?;
        let mut dec = dec_repr
            .parse::<BigInt>()
            .unwrap_or_else(|_| BigInt::from(0));
        if negative {
            dec = dec.neg();
        }
        let digits = whole * BigInt::from(10).pow(sig_figs) + dec;

        Some(Self { digits, sig_figs })
    }

    fn whole_part(&self) -> BigInt {
        if self.sig_figs == 0 {
            self.digits.clone()
        } else {
            self.digits.clone() / BigInt::from(10).pow(self.sig_figs)
        }
    }

    fn decimal_part(&self) -> BigInt {
        if self.sig_figs == 0 {
            BigInt::from(0)
        } else {
            self.digits.clone() % BigInt::from(10).pow(self.sig_figs)
        }
    }

    fn remove_trailing_zeros(&self) -> Self {
        let trailing_zeros = self
            .digits
            .to_string()
            .chars()
            .rev()
            .take(self.sig_figs as usize)
            .take_while(|&c| c == '0')
            .count() as u32;

        let sig_figs = self.sig_figs - trailing_zeros;
        let digits = if trailing_zeros == 0 {
            self.digits.clone()
        } else {
            self.digits.clone() / BigInt::from(10).pow(trailing_zeros)
        };

        Self { digits, sig_figs }
    }
}

impl PartialOrd for Decimal {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for Decimal {
    fn cmp(&self, other: &Self) -> Ordering {
        self.whole_part().cmp(&other.whole_part()).then_with(|| {
            let self_shift_dec = self.decimal_part() * BigInt::from(10).pow(other.sig_figs);
            let other_shift_dec = other.decimal_part() * BigInt::from(10).pow(self.sig_figs);
            self_shift_dec.cmp(&other_shift_dec)
        })
    }
}

impl Neg for Decimal {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            digits: self.digits.neg(),
            ..self
        }
    }
}

impl Add<Decimal> for Decimal {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        let pair = [&self, &other];
        let more_sf = pair.iter().max_by_key(|num| num.sig_figs).unwrap();
        let less_sf = pair.iter().min_by_key(|num| num.sig_figs).unwrap();
        let diff_sf = more_sf.sig_figs - less_sf.sig_figs;

        let digits =
            less_sf.digits.clone() * BigInt::from(10).pow(diff_sf) + more_sf.digits.clone();
        let sig_figs = more_sf.sig_figs;

        Self { digits, sig_figs }.remove_trailing_zeros()
    }
}

impl Sub<Decimal> for Decimal {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        self.add(other.neg())
    }
}

#[allow(clippy::suspicious_arithmetic_impl)]
impl Mul<Decimal> for Decimal {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        let digits = self.digits * other.digits;
        let sig_figs = self.sig_figs + other.sig_figs;

        Self { digits, sig_figs }.remove_trailing_zeros()
    }
}
