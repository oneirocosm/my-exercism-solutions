use custom_set::*;

#[test]
fn test_into_iter_with_empty_set() {
    let mut iter = CustomSet::<i32>::new(&[]).into_iter();

    assert_eq!(iter.next(), None);
}

#[test]
fn test_into_iter_with_non_empty_set() {
    let mut iter = CustomSet::new(&[2, 3, 5, 7]).into_iter();

    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.next(), Some(3));
    assert_eq!(iter.next(), Some(5));
    assert_eq!(iter.next(), Some(7));
    assert_eq!(iter.next(), None);
}
