use std::cmp::Ordering;

pub fn find(array: &[i32], key: i32) -> Option<usize> {
    if array.is_empty() {
        return None;
    }

    let index = array.len() / 2;

    match key.cmp(&array[index]) {
        Ordering::Less => find(&array[..index], key),
        Ordering::Greater => find(&array[index + 1..], key).map(|result| result + index + 1),
        Ordering::Equal => Some(index),
    }
}
