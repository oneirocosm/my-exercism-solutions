use integer_sqrt::IntegerSquareRoot;
use itertools::Itertools;
use ndarray::Array;

pub fn encrypt(input: &str) -> String {
    let mut filtered = input
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .map(|c| c.to_ascii_lowercase())
        .collect::<Vec<char>>();

    if filtered.is_empty() {
        return String::new();
    }

    let rows = filtered.len().integer_sqrt();
    let cols = if rows * rows == filtered.len() {
        rows
    } else {
        rows + 1
    };

    filtered.append(&mut vec![' '; rows * cols - filtered.len()]);

    Array::from_shape_vec((rows, cols), filtered)
        .expect("dimension error")
        .gencolumns()
        .into_iter()
        .map(|col| col.iter().cloned().collect::<String>())
        .join(" ")
}
