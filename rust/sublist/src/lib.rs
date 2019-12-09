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
    let list_len = list.len();
    let compare_len = compare_to.len();

    if list_len < compare_len {
        if contained_within(list, compare_to) {
            Comparison::Sublist
        }
        else {
            Comparison::Unequal
        }
    }
    else if list_len == compare_len {
        if list == compare_to {
            Comparison::Equal
        }
        else {
            Comparison::Unequal
        }
    }
    else {
        if contained_within(compare_to, list) {
            Comparison::Superlist
        }
        else {
            Comparison::Unequal
        }
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
fn contained_within<T: PartialEq>(small: &[T], big: &[T]) -> bool {
    let small_len = small.len();
    if small_len == 0 {
        return true
    }

    let mut subsets = big.windows(small_len);
    subsets.any(|subset| subset == small)
}
