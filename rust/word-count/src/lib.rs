use std::collections::HashMap;

/// Count occurrences of words.
pub fn word_count(words: &str) -> HashMap<String, u32> {
    words
        .split_whitespace()
        .flat_map(|word| word.split(','))
        .map(|word| {
            word.trim_matches(|c: char| c.is_ascii_punctuation())
                .to_lowercase()
        })
        .filter(|word| word != "")
        .fold(HashMap::new(), |mut counter, word| {
            *counter.entry(word).or_insert(0) += 1;
            counter
        })
}
