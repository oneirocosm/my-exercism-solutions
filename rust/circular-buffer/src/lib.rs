use std::collections::VecDeque;

pub struct CircularBuffer<T> {
    deque: VecDeque<T>,
    cap: usize,
}

#[derive(Debug, PartialEq)]
pub enum Error {
    EmptyBuffer,
    FullBuffer,
}

impl<T> CircularBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        Self {
            deque: VecDeque::new(),
            cap: capacity,
        }
    }

    pub fn write(&mut self, element: T) -> Result<(), Error> {
        if self.deque.len() == self.cap {
            Err(Error::FullBuffer)
        } else {
            self.deque.push_back(element);
            Ok(())
        }
    }

    pub fn read(&mut self) -> Result<T, Error> {
        self.deque.pop_front().ok_or(Error::EmptyBuffer)
    }

    pub fn clear(&mut self) {
        self.deque = VecDeque::new();
    }

    pub fn overwrite(&mut self, element: T) {
        if self.deque.len() == self.cap {
            let _ = self.deque.pop_front();
        }
        self.deque.push_back(element)
    }
}
