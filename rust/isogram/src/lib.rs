use std::collections::HashSet;

/// Reterns a boolean indicating if a word is an isogram
///
/// # Arguments
///
/// * `candidate` - the word that will be checked
///
/// # Example
///
/// ```
/// use isogram::check;
/// assert_eq!(check("super"), true);
/// assert_eq!(check("exciting"), false);
/// ```
pub fn check(candidate: &str) -> bool {
    // assume the candidate is an isogram until a repeated character is encountered
    let mut is_isogram = true;
    // a container to hold the characters previously encountered
    let mut encountered = HashSet::new();

    // for each char, verify no repeats except spaces and dashes
    for ch in candidate.to_lowercase().chars() {
        is_isogram = match ch {
            ' ' => is_isogram,
            '-' => is_isogram,
            // insert returns false if the character already existed in the HashSet
            _ => is_isogram && encountered.insert(ch),
        };
    }
    is_isogram
}
