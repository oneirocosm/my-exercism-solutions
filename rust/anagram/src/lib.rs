use std::collections::{HashMap, HashSet};

/// Returns a HashSet of the words in possible that are anagrams of
/// word (ignores case)
pub fn anagrams_for<'a>(word: &str, possible: &[&'a str]) -> HashSet<&'a str> {
    let freq = occurances(word);

    possible
        .iter()
        .map(|other| &'a other.to_lowercase().as_str())
        .filter(|&other| occurances(other) == freq)
        .filter(|&other| other != &word.to_lowercase())
        .copied()
        .collect()
}

/// Given a word, returns a HashMap recording how frequently each
/// char occurs ignoring case
fn occurances(word: &str) -> HashMap<char, u32> {
    let mut frequency: HashMap<char, u32> = HashMap::new();

    for c in word.chars() {
        let count = frequency.entry(c).or_insert(0);
        *count += 1;
    }
    frequency
}

#[cfg(test)]
mod occurances_tests {
    // Import function to be tested
    use super::occurances;
    use std::collections::HashMap;

    #[test]
    fn test_word_foo() {
        // test a regular lowercase word
        let word = "foo";
        let freq: HashMap<char, u32> = [('f', 1), ('o', 2)].iter().cloned().collect();

        assert_eq!(occurances(word), freq);
    }

    #[test]
    fn test_word_OoPs() {
        // test a word with mixed capital and lowercase letters
        let word = "OoPs";
        let freq: HashMap<char, u32> = [('o', 2), ('p', 1), ('s', 1)].iter().cloned().collect();

        assert_eq!(occurances(word), freq);
    }

    #[test]
    fn test_a_1000_times() {
        // test a very long string
        let word = &['a'; 1000].iter().collect::<String>();
        let freq: HashMap<char, u32> = [('a', 1000)].iter().cloned().collect();

        assert_eq!(occurances(word), freq);
    }
}
