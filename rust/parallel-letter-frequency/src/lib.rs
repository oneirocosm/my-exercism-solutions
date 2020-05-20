use rayon::prelude::*;
use std::collections::HashMap;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let chunk_sz = input.len() / worker_count + 1;

    let handles = input
        .par_chunks(chunk_sz)
        .map(|data| {
            let mut part_count = HashMap::new();
            for line in data {
                update_from_str(&mut part_count, line);
            }
            part_count
        })
        .collect::<Vec<_>>();

    let mut counter = HashMap::new();
    for received in handles {
        update_from_hash(&mut counter, &received);
    }
    counter
}

fn update_from_hash(main: &mut HashMap<char, usize>, other: &HashMap<char, usize>) {
    other
        .into_iter()
        .for_each(|(&letter, count)| *main.entry(letter).or_insert(0) += count);
}

fn update_from_str(counter: &mut HashMap<char, usize>, input: &str) {
    input
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphabetic())
        .for_each(|c| *counter.entry(c).or_insert(0) += 1);
}
