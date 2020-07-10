use std::collections::BTreeMap;

#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidRowCount(usize),
    InvalidColumnCount(usize),
}
const ZERO: &str = " _ | ||_|   ";
const ONE: &str = "     |  |   ";
const TWO: &str = " _  _||_    ";
const THREE: &str = " _  _| _|   ";
const FOUR: &str = "   |_|  |   ";
const FIVE: &str = " _ |_  _|   ";
const SIX: &str = " _ |_ |_|   ";
const SEVEN: &str = " _   |  |   ";
const EIGHT: &str = " _ |_||_|   ";
const NINE: &str = " _ |_| _|   ";

pub fn convert(input: &str) -> Result<String, Error> {
    let actual_rows = input
        .split('\n')
        .map(|inner| ActualRow(inner))
        .collect::<Vec<_>>();

    if actual_rows.len() % 4 != 0 {
        return Err(Error::InvalidRowCount(actual_rows.len()));
    }

    if let Some(count) = actual_rows.iter().find_map(|row| {
        if row.0.len() % 3 == 0 {
            None
        } else {
            Some(row.0.len())
        }
    }) {
        return Err(Error::InvalidColumnCount(count));
    }

    let optical_rows = break_into_optical_rows(actual_rows);

    Ok(optical_rows
        .into_iter()
        .map(|opt_row| -> Vec<OpticalDigit> { opt_row.into() })
        .map(|opt_digits| {
            opt_digits
                .into_iter()
                .map(|opt_digit| match opt_digit.0.as_str() {
                    ZERO => "0",
                    ONE => "1",
                    TWO => "2",
                    THREE => "3",
                    FOUR => "4",
                    FIVE => "5",
                    SIX => "6",
                    SEVEN => "7",
                    EIGHT => "8",
                    NINE => "9",
                    _ => "?",
                })
                .collect::<String>()
        })
        .collect::<Vec<_>>()
        .join(","))
}

#[derive(Clone)]
struct ActualRow<'a>(&'a str);
struct OpticalRow<'a>(Vec<ActualRow<'a>>);
struct OpticalDigit(String);

fn break_into_optical_rows<'a>(actual_rows: Vec<ActualRow<'a>>) -> Vec<OpticalRow<'a>> {
    actual_rows
        .chunks(4)
        .map(|actual_rows| OpticalRow(actual_rows.to_vec()))
        .collect::<Vec<_>>()
}

impl<'a> Into<Vec<OpticalDigit>> for OpticalRow<'a> {
    fn into(self) -> Vec<OpticalDigit> {
        let num_sub_pixels_in_row = self.0[0].0.len();
        self.0
            .into_iter()
            .map(|pixel_row| pixel_row.0.split("").filter(|c| c != &""))
            .flatten()
            .enumerate()
            .fold(
                BTreeMap::new(),
                |mut optical_pixels: BTreeMap<usize, Vec<&str>>, (index, sub_pixel)| {
                    let optical_ind = (index % num_sub_pixels_in_row) / 3;
                    match optical_pixels.get_mut(&optical_ind) {
                        Some(opt_pixel) => {
                            opt_pixel.push(sub_pixel);
                        }
                        None => {
                            optical_pixels.insert(optical_ind, vec![sub_pixel]);
                        }
                    }
                    optical_pixels
                },
            )
            .values()
            .map(|sub_pixels| OpticalDigit(sub_pixels.join("")))
            .collect::<Vec<_>>()
    }
}
