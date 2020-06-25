use maplit::hashmap;
use std::collections::{BTreeSet, HashMap};

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref COSTS: HashMap<usize, u32> = hashmap! {
        0 => discounted_cost(0),
        1 => discounted_cost(0),
        2 => discounted_cost(5),
        3 => discounted_cost(10),
        4 => discounted_cost(20),
        5 => discounted_cost(25),
    };
}

fn discounted_cost(discount_percent: u32) -> u32 {
    const BOOK_COST: f32 = 800.0;
    let fraction = 1.0 - (discount_percent as f32) / 100.0;
    (BOOK_COST * fraction) as u32
}

pub fn lowest_price(books: &[u32]) -> u32 {
    let book_sets = books.iter().fold(vec![BTreeSet::new()], push_to_best_set);

    calc_cost(book_sets)
}

fn push_to_best_set(mut sets: Vec<BTreeSet<u32>>, book: &u32) -> Vec<BTreeSet<u32>> {
    let mut lowest_cost_jump = u32::max_value();
    let mut best_set = None;
    for book_set in &mut sets {
        if !book_set.contains(book) {
            let old_len = book_set.len();
            let new_len = old_len + 1;
            let old_set_cost = (old_len as u32) * COSTS[&old_len];
            let new_set_cost = (new_len as u32) * COSTS[&new_len];
            let cost_jump = new_set_cost - old_set_cost;

            if cost_jump < lowest_cost_jump {
                lowest_cost_jump = cost_jump;
                best_set = Some(book_set);
            }
        }
    }

    if let Some(best_set) = best_set {
        best_set.insert(*book);
    }

    if !sets.last().unwrap().is_empty() {
        sets.push(BTreeSet::new())
    }

    sets
}

fn calc_cost(book_sets: Vec<BTreeSet<u32>>) -> u32 {
    book_sets
        .iter()
        .map(|set| set.len())
        .fold(0, |total, num_in_set| {
            total + (num_in_set as u32) * COSTS[&num_in_set]
        })
}
