pub fn map<F, T, U>(input: T, function: F) -> Vec<U>
where
    T: IntoIterator,
    F: FnMut(T::Item) -> U {
    input.into_iter().map(function).collect()
}
