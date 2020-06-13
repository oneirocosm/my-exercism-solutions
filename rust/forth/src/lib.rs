pub type Value = i32;
pub type ForthResult = Result<(), Error>;

#[derive(Default)]
pub struct Forth {
    stack: Vec<Value>,
}

#[derive(Debug, PartialEq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

impl Forth {
    pub fn new() -> Forth {
        Default::default()
    }

    pub fn stack(&self) -> Vec<Value> {
        self.stack.clone()
    }

    pub fn eval(&mut self, input: &str) -> ForthResult {
        let words = input.split(' ');

        for word in words {
            match word.to_ascii_uppercase().as_str() {
                n if n.parse::<Value>().is_ok() => {
                    self.stack.push(n.parse::<Value>().unwrap());
                }
                "+" => {
                    let top1 = self.pop()?;
                    let top2 = self.pop()?;
                    let sum = top2 + top1;
                    self.stack.push(sum);
                }
                "-" => {
                    let top1 = self.pop()?;
                    let top2 = self.pop()?;
                    let diff = top2 - top1;
                    self.stack.push(diff);
                }
                "*" => {
                    let top1 = self.pop()?;
                    let top2 = self.pop()?;
                    let prod = top2 * top1;
                    self.stack.push(prod);
                }
                "/" => {
                    let top1 = self.pop()?;
                    if top1 == 0 {
                        return Err(Error::DivisionByZero);
                    }
                    let top2 = self.pop()?;
                    let prod = top2 / top1;
                    self.stack.push(prod);
                }
                "DUP" => {
                    let top = self.pop()?;
                    self.stack.push(top);
                    self.stack.push(top);
                }
                "DROP" => {
                    self.pop()?;
                }
                "SWAP" => {
                    let top1 = self.pop()?;
                    let top2 = self.pop()?;
                    self.stack.push(top1);
                    self.stack.push(top2);
                }
                "OVER" => {
                    let top1 = self.pop()?;
                    let top2 = self.pop()?;
                    self.stack.push(top2);
                    self.stack.push(top1);
                    self.stack.push(top2);
                }

                _ => unimplemented!(),
            }
        }

        Ok(())
    }

    fn pop(&mut self) -> Result<Value, Error> {
        self.stack.pop().ok_or(Error::StackUnderflow)
    }
}
