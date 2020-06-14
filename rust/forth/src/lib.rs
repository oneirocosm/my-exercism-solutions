use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;

pub type Value = i32;
pub type ForthResult = Result<(), Error>;

#[derive(Clone)]
pub struct Forth<'a> {
    stack: Vec<Value>,
    words: HashMap<String, Rc<RefCell<dyn FnMut(&mut Forth) -> ForthResult + 'a>>>,
}

#[derive(Debug, PartialEq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

impl<'a> Forth<'a> {
    pub fn new() -> Forth<'a> {
        Forth {
            stack: Vec::new(),
            words: Self::default_ops(),
        }
    }

    pub fn stack(&self) -> Vec<Value> {
        self.stack.clone()
    }

    pub fn eval(&mut self, input: &'a str) -> ForthResult {
        let word_commands: Vec<&'a str> = input.split(' ').collect();

        if word_commands.get(0) == Some(&":") {
            let key = word_commands.get(1).ok_or(Error::InvalidWord)?;
            if key.parse::<Value>().is_ok() {
                return Err(Error::InvalidWord);
            }

            let val = self.create_new(word_commands.clone())?;
            self.words.insert(key.to_ascii_uppercase(), val);
        } else {
            for cmd in word_commands {
                if let Ok(digit_rep) = cmd.parse::<Value>() {
                    self.stack.push(digit_rep);
                } else {
                    let cmd = cmd.to_ascii_uppercase();
                    let temp_words = self.words.clone();
                    let op = temp_words.get(&cmd).ok_or(Error::UnknownWord)?;
                    // workaround for bug that does not allow borrow_mut (issue 26186)
                    op.deref().borrow_mut().deref_mut()(self)?;
                }
            }
        }

        Ok(())
    }

    fn pop(&mut self) -> Result<Value, Error> {
        self.stack.pop().ok_or(Error::StackUnderflow)
    }

    fn default_ops() -> HashMap<String, Rc<RefCell<dyn FnMut(&mut Forth) -> ForthResult + 'a>>> {
        let mut operations: HashMap<String, Rc<RefCell<dyn FnMut(&mut Forth) -> ForthResult>>> =
            HashMap::new();
        operations.insert("+".to_owned(), Self::create_add());
        operations.insert("-".to_owned(), Self::create_sub());
        operations.insert("*".to_owned(), Self::create_mul());
        operations.insert("/".to_owned(), Self::create_div());
        operations.insert("DUP".to_owned(), Self::create_dup());
        operations.insert("DROP".to_owned(), Self::create_drop());
        operations.insert("SWAP".to_owned(), Self::create_swap());
        operations.insert("OVER".to_owned(), Self::create_over());

        operations
    }

    fn create_number(digit_rep: Value) -> Rc<RefCell<dyn FnMut(&mut Forth) -> ForthResult + 'a>> {
        Rc::new(RefCell::new(move |forth: &mut Forth| {
            forth.stack.push(digit_rep);
            Ok(())
        }))
    }

    fn create_add() -> Rc<RefCell<impl FnMut(&mut Forth) -> ForthResult>> {
        Rc::new(RefCell::new(|forth: &mut Forth| {
            let top1 = forth.pop()?;
            let top2 = forth.pop()?;
            let sum = top2 + top1;
            forth.stack.push(sum);
            Ok(())
        }))
    }

    fn create_sub() -> Rc<RefCell<impl FnMut(&mut Forth) -> ForthResult>> {
        Rc::new(RefCell::new(|forth: &mut Forth| {
            let top1 = forth.pop()?;
            let top2 = forth.pop()?;
            let diff = top2 - top1;
            forth.stack.push(diff);
            Ok(())
        }))
    }

    fn create_mul() -> Rc<RefCell<impl FnMut(&mut Forth) -> ForthResult>> {
        Rc::new(RefCell::new(|forth: &mut Forth| {
            let top1 = forth.pop()?;
            let top2 = forth.pop()?;
            let prod = top2 * top1;
            forth.stack.push(prod);
            Ok(())
        }))
    }

    fn create_div() -> Rc<RefCell<impl FnMut(&mut Forth) -> ForthResult>> {
        Rc::new(RefCell::new(|forth: &mut Forth| {
            let top1 = forth.pop()?;
            if top1 == 0 {
                return Err(Error::DivisionByZero);
            }
            let top2 = forth.pop()?;
            let prod = top2 / top1;
            forth.stack.push(prod);
            Ok(())
        }))
    }

    fn create_dup() -> Rc<RefCell<impl FnMut(&mut Forth) -> ForthResult>> {
        Rc::new(RefCell::new(|forth: &mut Forth| {
            let top = forth.pop()?;
            forth.stack.push(top);
            forth.stack.push(top);
            Ok(())
        }))
    }

    fn create_drop() -> Rc<RefCell<impl FnMut(&mut Forth) -> ForthResult>> {
        Rc::new(RefCell::new(|forth: &mut Forth| {
            forth.pop()?;
            Ok(())
        }))
    }

    fn create_swap() -> Rc<RefCell<impl FnMut(&mut Forth) -> ForthResult>> {
        Rc::new(RefCell::new(|forth: &mut Forth| {
            let top1 = forth.pop()?;
            let top2 = forth.pop()?;
            forth.stack.push(top1);
            forth.stack.push(top2);
            Ok(())
        }))
    }

    fn create_over() -> Rc<RefCell<impl FnMut(&mut Forth) -> ForthResult>> {
        Rc::new(RefCell::new(|forth: &mut Forth| {
            let top1 = forth.pop()?;
            let top2 = forth.pop()?;
            forth.stack.push(top2);
            forth.stack.push(top1);
            forth.stack.push(top2);
            Ok(())
        }))
    }

    fn create_new(
        &self,
        words: Vec<&str>,
    ) -> Result<Rc<RefCell<impl FnMut(&mut Forth) -> ForthResult + 'a>>, Error> {
        let mut words = words.iter().skip(2);
        let mut subroutines = Vec::new();
        loop {
            match words.next() {
                Some(&";") => break,
                None => return Err(Error::InvalidWord),
                Some(word) => {
                    if let Ok(digit_rep) = word.parse::<Value>() {
                        let op = Self::create_number(digit_rep);
                        subroutines.push(op.clone());
                    } else {
                        let word = word.to_ascii_uppercase();
                        let op = self.words.get(&word).ok_or(Error::UnknownWord)?;
                        subroutines.push(op.clone());
                    }
                }
            }
        }

        Ok(Rc::new(RefCell::new(move |forth: &mut Forth| {
            for subroutine in subroutines.clone() {
                // workaround for bug that does not allow borrow_mut (issue 26186)
                subroutine.deref().borrow_mut().deref_mut()(forth)?;
            }
            Ok(())
        })))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_redefs_do_not_change_old_redefs() {
        let mut f = Forth::new();
        assert!(f.eval(": FOO 8 ;").is_ok());
        assert!(f.eval("FOO").is_ok());
        assert_eq!(vec![8], f.stack());

        assert!(f.eval(": BAR FOO ;").is_ok());
        assert!(f.eval("BAR").is_ok());
        assert_eq!(vec![8, 8], f.stack());

        assert!(f.eval(": FOO 5 ;").is_ok());
        assert!(f.eval("FOO").is_ok());
        assert_eq!(vec![8, 8, 5], f.stack());

        assert!(f.eval("BAR").is_ok());
        assert_eq!(vec![8, 8, 5, 8], f.stack());
    }
}
