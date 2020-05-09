use std::collections::HashSet;
use std::sync::Mutex;

use lazy_static::lazy_static;
use rand::{rngs::ThreadRng, Rng};
use unic_char_range::chars;

lazy_static! {
    static ref IDS: Mutex<HashSet<String>> = Mutex::new(HashSet::new());
}

#[derive(Default)]
pub struct Robot {
    id: String,
    rng: ThreadRng,
}

impl Robot {
    pub fn new() -> Self {
        let mut robot: Self = Default::default();

        robot.reset_name();
        robot
    }

    pub fn name(&self) -> &str {
        &self.id
    }

    pub fn reset_name(&mut self) {
        let mut taken_ids = IDS.lock().unwrap();

        // free up old name for other robots
        taken_ids.remove(&self.id);

        loop {
            let mut name = String::new();
            for _ in 0..2 {
                let next = self.generate_letter();
                name.push(next);
            }

            for _ in 0..3 {
                let next = self.generate_digit();
                name.push(next);
            }

            // do not allow repeat ids
            if taken_ids.get(&name) == None {
                taken_ids.insert(name.clone());
                self.id = name;
                break;
            }
        }
    }

    fn generate_letter(&mut self) -> char {
        let index = self.rng.gen_range(0, 26);
        chars!('A'..='Z').iter().nth(index).unwrap()
    }

    fn generate_digit(&mut self) -> char {
        std::char::from_digit(self.rng.gen_range(0, 10), 10).unwrap()
    }
}
