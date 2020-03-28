use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    let mut new_map = BTreeMap::new();
    for (&score, letters) in h.iter() {
        for letter in letters.iter() {
            new_map.insert(letter.to_ascii_lowercase(), score);
        }
    }
    new_map
}
