use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crossbeam_utils::thread;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let mut counter = HashMap::new();
    let _ = thread::scope(|scope| {
        let input = Arc::new(Mutex::new(input.iter()));
        let mut handles = Vec::new();

        for _ in 0..worker_count {
            let data = input.clone();
            let mut part_count = HashMap::<char, usize>::new();
            let handle = scope.spawn(move |_| {
                loop {
                    let line = data.lock().unwrap().next();
                    match line {
                        None => break,
                        Some(content) => update_from_str(&mut part_count, content),
                    }
                }
                part_count
            });
            handles.push(handle);
        }

        for handle in handles {
            let received = handle.join().unwrap();
            update_from_hash(&mut counter, received);
        }
    });
    counter
}

fn update_from_hash(main: &mut HashMap<char, usize>, other: HashMap<char, usize>) {
    other
        .iter()
        .for_each(|(&letter, count)| *main.entry(letter).or_insert(0) += count);
}

fn update_from_str(counter: &mut HashMap<char, usize>, input: &str) {
    input
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphabetic())
        .for_each(|c| *counter.entry(c).or_insert(0) += 1);
}
