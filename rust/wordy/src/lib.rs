use itertools::Itertools;

pub struct WordProblem {
    value: i32,
    queued_op: Option<Box<dyn Fn(i32, i32) -> i32>>,
}

impl WordProblem {
    fn new() -> Self {
        Self {
            value: 0,
            queued_op: None,
        }
    }

    fn is_digit(candidate: &str) -> bool {
        Self::parse_value(candidate).is_some()
    }

    fn parse_value(mut candidate: &str) -> Option<i32> {
        const SUFFIXES: &[&str] = &["st", "nd", "rd", "th"];
        for suffix in SUFFIXES {
            candidate = candidate.trim_end_matches(suffix);
        }
        candidate.parse::<i32>().ok()
    }
}

pub fn answer(command: &str) -> Option<i32> {
    const PLUS: &str = "plus";
    const MINUS: &str = "minus";
    const MULT: &str = "multiplied by";
    const DIV: &str = "divided by";
    const EXP: &str = "raised to the";
    const IGNORE: &str = "power";

    let mut state = WordProblem::new();
    let ch = command
        .trim_end_matches('?')
        .split(' ')
        .group_by(|word| WordProblem::is_digit(*word));
    let mut iter = ch.into_iter().map(|(_, mut group)| group.join(" "));

    if iter.next() != Some("What is".to_owned()) {
        return None;
    }

    let first_value = WordProblem::parse_value(iter.next()?.as_str())?;
    state.value = first_value;

    loop {
        match (iter.next(), state.queued_op) {
            (None, op) => {
                if op.is_none() {
                    break;
                } else {
                    return None;
                }
            }
            (Some(next_token), None) => {
                match next_token.as_str() {
                    PLUS => state.queued_op = Some(Box::new(|x, y| x + y)),
                    MINUS => state.queued_op = Some(Box::new(|x, y| x - y)),
                    MULT => state.queued_op = Some(Box::new(|x, y| x * y)),
                    DIV => state.queued_op = Some(Box::new(|x, y| x / y)),
                    EXP => state.queued_op = Some(Box::new(|x, y| x.pow(y as u32))),
                    IGNORE => state.queued_op = None,
                    _ => return None,
                };
            }
            (Some(next_token), Some(queued_op)) => {
                let next_value = WordProblem::parse_value(next_token.as_str())?;
                state.value = queued_op(state.value, next_value);
                state.queued_op = None;
            }
        }
    }
    Some(state.value)
}
