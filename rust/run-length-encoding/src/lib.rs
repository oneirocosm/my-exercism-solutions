use itertools::Itertools;
use regex::Regex;

/// Encodes a given string slice into run length encoding
///
/// # Arguments
///
/// * `source` - The string slice to be encoded
///
/// # Examples
///
/// ```
/// use run_length_encoding::encode;
/// assert_eq!(encode("AABCCCDEEEE"), "2AB3CD4E".to_string())
/// ```
pub fn encode(source: &str) -> String {
    // group like characters together, count, and make the 1s invisible
    source
        .chars()
        .group_by(|&ch| ch)
        .into_iter()
        .map(|(ch, group)| {
            let len = group.count();
            let num = match len {
                1 => "".to_string(),
                _ => len.to_string(),
            };
            format!("{}{}", num, ch)
        })
        .collect()
}

/// Decodes a run length encoding into the original text
///
/// # Arguments
///
/// * source - string slice containing the encoded message
///
/// # Examples
///
/// ```
/// use run_length_encoding::decode;
/// assert_eq!(decode("2AB3CD4E"), "AABCCCDEEEE".to_string())
/// ```
pub fn decode(source: &str) -> String {
    let re = Regex::new(r"\d*\D").unwrap();
    // convert each regex group to string, then separate into numbers and letters
    // then repeat the letter the appropriate number of times
    re.captures_iter(source)
        .map(|x| x.get(0).map_or("", |y| y.as_str()))
        .map(|x| {
            let (num, ch): (String, String) = x.chars().partition(|y| y.is_digit(10));
            match num.as_str() {
                "" => ch,
                _ => ch.repeat(num.parse().unwrap()),
            }
        })
        .collect()
}
