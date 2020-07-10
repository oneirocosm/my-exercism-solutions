use itertools::Itertools;
use std::collections::{BTreeSet, HashMap};

struct Alphametic {
    lhs_words: Vec<Vec<char>>,
    rhs_word: Vec<char>,
    letters: BTreeSet<char>,
}

impl Alphametic {
    fn new(input: &str) -> Self {
        let halves: Vec<&str> = input.split(" == ").collect();
        let (lhs, rhs) = (halves[0], halves[1]);

        let lhs_words = lhs
            .split(" + ")
            .map(|x| x.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let rhs_word = rhs.chars().collect::<Vec<_>>();

        let mut letters = lhs_words
            .clone()
            .into_iter()
            .flatten()
            .collect::<BTreeSet<_>>();
        let rhs_letters = rhs_word.clone().into_iter().collect::<BTreeSet<_>>();
        letters.extend(&rhs_letters);

        Self {
            lhs_words,
            rhs_word,
            letters,
        }
    }

    fn solve(&self) -> Option<HashMap<char, u8>> {
        let num_letters = self.letters.len();
        let num_eq = self.rhs_word.len();
        let non_zero = self.find_non_zero_chars();

        let candidates = (0u8..=9)
            .permutations(num_letters)
            .map(|vals| {
                self.letters
                    .clone()
                    .into_iter()
                    .zip(vals)
                    .collect::<HashMap<_, _>>()
            })
            .filter(|candidate| {
                !candidate
                    .iter()
                    .any(|(letter, val)| non_zero.contains(&letter) && *val == 0u8)
            });

        candidates.into_iter().find(|candidate| {
            let mut lhs_words = self.lhs_words.clone();
            let mut rhs_word = self.rhs_word.clone();

            (0..num_eq).try_fold(0u32, |carry, _| {
                let lhs_sum = lhs_words.iter_mut().fold(0, |sum, word| {
                    sum + word.pop().map(|c| candidate[&c] as u32).unwrap_or(0u32)
                }) + carry;
                let rhs_sum = candidate[&rhs_word.pop().unwrap()] as u32;

                if lhs_sum % 10 == rhs_sum {
                    Ok(lhs_sum / 10)
                } else {
                    Err(())
                }
            }) == Ok(0)
        })
    }

    fn find_non_zero_chars(&self) -> Vec<&char> {
        let mut non_zero = self
            .lhs_words
            .iter()
            .filter_map(|word| word.get(0))
            .collect::<Vec<_>>();
        non_zero.push(self.rhs_word.get(0).unwrap());
        non_zero
    }
}

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    Alphametic::new(input).solve()
}
