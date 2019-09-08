const BRACKETS: &str = r#"{[()]}"#;

pub fn brackets_are_balanced(input: &str) -> bool {
    input.chars().fold(String::new(), collapse).is_empty()
}

/// A function used to fold a string with matching bracket pairs
/// canceling each other out.  Does not work with graphemes
fn collapse(past: String, next: char) -> String {
    let mut sz = past.len();

    // empty string is a special case
    if sz != 0 {
        sz -= 1;
    }

    // trunc contains all but the last char in past
    // past contains only the last char in past
    let (trunc, prev) = past.split_at(sz);

    match (prev, next) {
        // pairs of brackets cancel each other out
        ("[", ']') => trunc.to_string(),
        ("(", ')') => trunc.to_string(),
        ("{", '}') => trunc.to_string(),

        // non-matching brackets are appended
        (_, b) if BRACKETS.contains(b) => format!("{}{}", past, b),

        // everything else is ignored
        (_, _) => past,
    }
}
