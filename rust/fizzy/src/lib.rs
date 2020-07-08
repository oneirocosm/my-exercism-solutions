/// A Matcher is a single rule of fizzbuzz: given a function on T, should
/// a word be substituted in? If yes, which word?
pub struct Matcher<'a, T>(Box<dyn Fn(T) -> Option<String> + 'a>);

impl<'a, T> Matcher<'a, T> {
    pub fn new<F, S>(matcher: F, subs: S) -> Matcher<'a, T>
    where
        F: Fn(T) -> bool + 'a,
        S: ToString + 'a,
    {
        Self(Box::new(move |input: T| {
            if matcher(input) {
                Some(subs.to_string())
            } else {
                None
            }
        }))
    }
}

/// A Fizzy is a set of matchers, which may be applied to an iterator.
///
/// Strictly speaking, it's usually more idiomatic to use `iter.map()` than to
/// consume an iterator with an `apply` method. Given a Fizzy instance, it's
/// pretty straightforward to construct a closure which applies it to all
/// elements of the iterator. However, we're using the `apply` pattern
/// here because it's a simpler interface for students to implement.
///
/// Also, it's a good excuse to try out using impl trait.
#[derive(Default)]
pub struct Fizzy<'a, T>(Vec<Matcher<'a, T>>);

impl<'a, T> Fizzy<'a, T>
where
    T: ToString + Copy + Default + 'a,
{
    pub fn new() -> Self {
        Self::default()
    }

    // feel free to change the signature to `mut self` if you like
    pub fn add_matcher(mut self, matcher: Matcher<'a, T>) -> Self {
        self.0.push(matcher);
        self
    }

    /// map this fizzy onto every element of an iterator, returning a new iterator
    pub fn apply<I>(self, iter: I) -> impl Iterator<Item = String> + 'a
    where
        I: Iterator<Item = T> + 'a,
    {
        iter.map(move |elem| self.convert(elem))
    }

    fn convert(&self, item: T) -> String {
        let mut sounds = self
            .0
            .iter()
            .filter_map(move |matcher| (matcher.0)(item))
            .collect::<Vec<String>>();

        if sounds.is_empty() {
            sounds.push(item.to_string())
        }
        sounds.join("")
    }
}

/// convenience function: return a Fizzy which applies the standard fizz-buzz rules
pub fn fizz_buzz<'a, T>() -> Fizzy<'a, T>
where
    T: std::ops::Rem<Output = T> + From<u8> + PartialEq + Copy + ToString + Default + 'a,
{
    Fizzy::new()
        .add_matcher(Matcher::new(|n: T| n % 3.into() == 0.into(), "fizz"))
        .add_matcher(Matcher::new(|n: T| n % 5.into() == 0.into(), "buzz"))
}
