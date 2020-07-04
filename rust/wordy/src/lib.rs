use itertools::Itertools;

pub struct WordProblem {
    value: i32,
    queued_op: Option<Box<dyn Fn (i32, i32) -> i32>>,
}

impl WordProblem {
    fn new() -> Self {
        Self { 
            value: 0,
            queued_op: Some(Box::new(|x, y| x+y)),
        }
    }

    fn is_digit(candidate: char) -> bool {
        match candidate {
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '-' => true,
            _ => false,
        }
    }

    fn parse_value(candidate: String) -> Option<i32> {
        candidate.parse::<i32>().ok()
    }
}

pub fn answer(command: &str) -> Option<i32> {
    const PLUS: &str = " plus ";
    const MINUS: &str = " minus ";
    const MULT: &str = " multiplied by ";
    const DIV: &str = " divided by ";

    let mut state = WordProblem::new();
    let ch = command.chars()
        .group_by(|c| WordProblem::is_digit(*c));
    let mut iter = ch
        .into_iter()
        .map(|(_, group)| group.collect::<String>());

    if iter.next() != Some("What is ".to_owned()) {
        return None;
    }

    loop {
        match (iter.next(), state.queued_op) {
            (None, _) => return None,
            (Some(next_token), None) => {
                println!("a");
                match next_token.as_str() {
                    PLUS => state.queued_op = Some(Box::new(|x,y| x+y)),
                    MINUS => state.queued_op = Some(Box::new(|x,y| x-y)),
                    MULT => state.queued_op = Some(Box::new(|x,y| x * y)),
                    DIV => state.queued_op = Some(Box::new(|x,y| x / y)),
                    "?" => break,
                    _ => return None,
                };
            }
            (Some(next_token), Some(queued_op)) => {
                let next_value = WordProblem::parse_value(next_token)?;
                state.value = queued_op(state.value, next_value);
                state.queued_op = None;
            }
        }
    }
    Some(state.value)
}
