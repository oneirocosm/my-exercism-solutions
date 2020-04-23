use std::collections::HashSet;

/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    let alphabet: HashSet<char> = (b'a'..=b'z').map(|c| c as char).collect();

    let letters_used = sentence
        .chars()
        .filter(|letter| letter.is_ascii_alphabetic())
        .map(|letter| letter.to_ascii_lowercase())
        .fold(HashSet::new(), |mut letters, new_letter| {
            letters.insert(new_letter);
            letters
        });
    letters_used == alphabet
}
