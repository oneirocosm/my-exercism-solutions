//! sublist
//! Determines whether a list of elements is one of the
//! following when compared to another list:
//! * Equal
//! * Sublist
//! * Superlist
//! * Unequal
//!
//! note that the order of elements in the list matters

#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

/// # Description:
/// Compares one list to another and returns an enum that
/// corresponds with how it relates to the other list
///
/// # Inputs:
/// * list: &[T] - the list that will be compared with another
/// * compare_to: &[T] - the list being compared to
///
/// # Return:
/// Comparison - an enum representing how list compares to compare_to
pub fn sublist<T: PartialEq>(list: &[T], compare_to: &[T]) -> Comparison {
    // note: if A is a subset of B, and B is a subset of A, then A=B
    match (is_subset(list, compare_to), is_subset(compare_to, list)) {
        (true, true) => Comparison::Equal,
        (true, false) => Comparison::Sublist,
        (false, true) => Comparison::Superlist,
        (false, false) => Comparison::Unequal,
    }
}

/// # Description:
/// Compares two lists and returns whether or not the smaller
/// list is contained within the larger one.  The order of
/// the elements in the list matter
///
/// # Arguments:
/// * small: &[T] the smaller list of the two
/// * large: &[T] the larger list of the two
///
/// # Return:
/// bool - true if the smaller list is within the larger list
fn is_subset<T: PartialEq>(small: &[T], big: &[T]) -> bool {
    match small.len() {
        0 => true,
        n => big.windows(n).any(|subset| subset == small),
    }
}
