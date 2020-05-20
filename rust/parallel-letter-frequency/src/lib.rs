use rayon::prelude::*;
use std::collections::HashMap;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let chunk_sz = input.len() / worker_count + 1;

    rayon::ThreadPoolBuilder::new()
        .num_threads(worker_count)
        .build()
        .unwrap();

    input
        .par_chunks(chunk_sz)
        .map(|data| {
            data.into_iter()
                .fold(HashMap::new(), |mut part_count, line| {
                    update_from_str(&mut part_count, line);
                    part_count
                })
        })
        .reduce(
            || HashMap::new(),
            |mut counter, part_count| {
                update_from_hash(&mut counter, &part_count);
                counter
            },
        )
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
