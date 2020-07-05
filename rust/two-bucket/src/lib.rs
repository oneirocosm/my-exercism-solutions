use std::collections::HashSet;
use std::hash::{Hash, Hasher};

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Bucket {
    One,
    Two,
}

/// A struct to hold your results in.
#[derive(PartialEq, Eq, Debug)]
pub struct BucketStats {
    /// The total number of "moves" it should take to reach the desired number of liters, including
    /// the first fill.
    pub moves: u8,
    /// Which bucket should end up with the desired number of liters? (Either "one" or "two")
    pub goal_bucket: Bucket,
    /// How many liters are left in the other bucket?
    pub other_bucket: u8,
}

impl From<BucketsState> for BucketStats {
    fn from(state: BucketsState) -> Self {
        let goal_bucket = *state.bucket_at_goal().unwrap();
        let other_bucket = match goal_bucket {
            Bucket::One => state.amount_2,
            Bucket::Two => state.amount_1,
        };

        Self {
            moves: state.moves,
            goal_bucket,
            other_bucket,
        }
    }
}

/// Solve the bucket problem
pub fn solve(
    capacity_1: u8,
    capacity_2: u8,
    goal: u8,
    start_bucket: &Bucket,
) -> Option<BucketStats> {
    let start_state = BucketsState::new(capacity_1, capacity_2, goal, start_bucket);
    if start_state.bucket_at_goal().is_some() {
        return Some(BucketStats::from(start_state));
    }

    let actions: Vec<Box<dyn Fn(BucketsState) -> BucketsState>> = vec![
        Box::new(BucketsState::pour_1_to_2),
        Box::new(BucketsState::pour_2_to_1),
        Box::new(BucketsState::empty_1),
        Box::new(BucketsState::empty_2),
        Box::new(BucketsState::fill_1),
        Box::new(BucketsState::fill_2),
    ];

    let non_start = match start_bucket {
        Bucket::One => &Bucket::Two,
        Bucket::Two => &Bucket::One,
    };

    let mut prev_states = HashSet::new();
    prev_states.insert(BucketsState::empty(capacity_1, capacity_2, goal));
    prev_states.insert(BucketsState::new(capacity_1, capacity_2, goal, non_start));

    let mut current_states = vec![start_state];
    loop {
        let mut new_states = Vec::new();
        for state in current_states {
            for action in &actions {
                let new_state = action(state);

                if new_state.bucket_at_goal().is_some() {
                    return Some(BucketStats::from(new_state));
                } else if !prev_states.contains(&new_state) {
                    prev_states.insert(new_state);
                    new_states.push(new_state);
                }
            }
        }
        if new_states.is_empty() {
            return None;
        }
        current_states = new_states;
    }
}

#[derive(Clone, Copy, Debug)]
struct BucketsState {
    capacity_1: u8,
    capacity_2: u8,
    amount_1: u8,
    amount_2: u8,
    goal: u8,
    moves: u8,
}

impl BucketsState {
    fn new(capacity_1: u8, capacity_2: u8, goal: u8, start_bucket: &Bucket) -> Self {
        let amount_1 = match start_bucket {
            Bucket::One => capacity_1,
            Bucket::Two => 0,
        };
        let amount_2 = match start_bucket {
            Bucket::One => 0,
            Bucket::Two => capacity_2,
        };

        Self {
            capacity_1,
            capacity_2,
            amount_1,
            amount_2,
            goal,
            moves: 1,
        }
    }

    fn empty(capacity_1: u8, capacity_2: u8, goal: u8) -> Self {
        Self {
            capacity_1,
            capacity_2,
            amount_1: 0,
            amount_2: 0,
            goal,
            moves: 1,
        }
    }

    fn bucket_at_goal(&self) -> Option<&Bucket> {
        if self.amount_1 == self.goal {
            Some(&Bucket::One)
        } else if self.amount_2 == self.goal {
            Some(&Bucket::Two)
        } else {
            None
        }
    }

    fn pour_1_to_2(self) -> Self {
        let sum = self.amount_2.saturating_add(self.amount_1);
        let amount_2 = if sum > self.capacity_2 {
            self.capacity_2
        } else {
            sum
        };

        let amount_1 = if sum > self.capacity_2 {
            sum - self.capacity_2
        } else {
            0
        };

        Self {
            capacity_1: self.capacity_1,
            capacity_2: self.capacity_2,
            amount_1,
            amount_2,
            goal: self.goal,
            moves: self.moves + 1,
        }
    }

    fn pour_2_to_1(self) -> Self {
        let sum = self.amount_1.saturating_add(self.amount_2);
        let amount_1 = if sum > self.capacity_1 {
            self.capacity_1
        } else {
            sum
        };

        let amount_2 = if sum > self.capacity_1 {
            sum - self.capacity_1
        } else {
            0
        };

        Self {
            amount_1,
            amount_2,
            moves: self.moves + 1,
            ..self
        }
    }

    fn empty_1(self) -> Self {
        Self {
            amount_1: 0,
            moves: self.moves + 1,
            ..self
        }
    }

    fn empty_2(self) -> Self {
        Self {
            amount_2: 0,
            moves: self.moves + 1,
            ..self
        }
    }

    fn fill_1(self) -> Self {
        Self {
            amount_1: self.capacity_1,
            moves: self.moves + 1,
            ..self
        }
    }

    fn fill_2(self) -> Self {
        Self {
            amount_2: self.capacity_2,
            moves: self.moves + 1,
            ..self
        }
    }
}

impl PartialEq for BucketsState {
    fn eq(&self, other: &Self) -> bool {
        self.amount_1 == other.amount_1 && self.amount_2 == other.amount_2
    }
}

impl Eq for BucketsState {}

impl Hash for BucketsState {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.amount_1.hash(state);
        self.amount_2.hash(state);
    }
}
