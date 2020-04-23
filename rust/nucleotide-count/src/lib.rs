use std::collections::HashMap;

const VALID: &str = "ACGT";

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    if !VALID.contains(nucleotide) {
        return Err(nucleotide);
    }

    dna.chars()
        .try_fold(0, |acc, strand_elem| match strand_elem {
            elem if elem == nucleotide => Ok(acc + 1),
            elem if VALID.contains(elem) => Ok(acc),
            elem => Err(elem),
        })
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    VALID
        .chars()
        .try_fold(HashMap::new(), |mut acc, nucleotide| {
            acc.insert(nucleotide, count(nucleotide, dna)?);
            Ok(acc)
        })
}
