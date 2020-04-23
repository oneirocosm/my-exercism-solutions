use convert_case::{Case, Casing};

pub fn abbreviate(phrase: &str) -> String {
    phrase
        .to_case(Case::Title)
        .split(' ')
        .filter_map(|word| word.chars().next())
        .collect::<String>()
        .to_uppercase()
}
