// WARNING: uses nightly
use itertools::Itertools;

pub fn translate(input: &str) -> String {
    let consonants = "bcdfghjklmnpqrstvwxyz"
        .split("")
        .filter(|cons| *cons != "")
        .collect::<Vec<&str>>();

    let complex_consonants = vec!["sch", "thr", "squ", "th", "qu", "rh", "ch"];
    let complex_vowels = vec!["yt", "xr"];

    input
        .split(' ')
        .map(|word| {
            if let Some((prefix, suffix)) = complex_consonants
                .iter()
                .find_map(|cons| word.strip_prefix(cons).map(|remainder| (cons, remainder)))
            {
                format!("{}{}ay", suffix, prefix)
            } else if complex_vowels.iter().any(|cons| word.starts_with(cons)) {
                format!("{}ay", word)
            } else if let Some((prefix, suffix)) = consonants
                .iter()
                .find_map(|cons| word.strip_prefix(cons).map(|remainder| (cons, remainder)))
            {
                format!("{}{}ay", suffix, prefix)
            } else {
                format!("{}ay", word)
            }
        })
        .join(" ")
}
