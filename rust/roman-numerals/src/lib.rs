#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate maplit;

use std::collections::BTreeMap;
use std::fmt::{Display, Formatter, Result};

pub struct Roman(u32);

lazy_static! {
    static ref NUMERAL_VALS: BTreeMap<u32, &'static str> = btreemap! {
        1 => "I",
        4 => "IV",
        5 => "V",
        9 => "IX",
        10 => "X",
        40 => "XL",
        50 => "L",
        90 => "XC",
        100 => "C",
        400 => "CD",
        500 => "D",
        900 => "CM",
        1000 => "M",
    };
}

impl Display for Roman {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let (output, _) = NUMERAL_VALS.iter().rev().fold(
            (String::new(), self.0),
            |(mut output, rem), (glyph_val, glyph)| {
                let num_repeats = rem / glyph_val;
                let new_rem = rem % glyph_val;

                output.push_str(&glyph.repeat(num_repeats as usize));
                (output, new_rem)
            },
        );
        write!(f, "{}", output)
    }
}

impl From<u32> for Roman {
    fn from(num: u32) -> Self {
        Self(num)
    }
}
