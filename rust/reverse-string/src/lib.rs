use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    // reverse the iterator and collect in a string
    input.graphemes(true).rev().collect::<String>()
}
