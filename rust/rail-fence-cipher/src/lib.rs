use std::collections::VecDeque;
use unicode_segmentation::UnicodeSegmentation;

pub struct RailFence {
    n_rows: usize,
}

impl RailFence {
    pub fn new(rails: u32) -> RailFence {
        Self {
            n_rows: rails as usize,
        }
    }

    pub fn encode(&self, text: &str) -> String {
        let processed_text = text.graphemes(true).collect::<Vec<&str>>();
        let mut arr = self.create_array(processed_text.len());

        processed_text
            .into_iter()
            .enumerate()
            .for_each(|(i, segment)| arr[self.get_row_num(i)][i] = segment);

        arr.iter().map(|row| row.join("")).collect::<String>()
    }

    pub fn decode(&self, cipher: &str) -> String {
        let mut processed_text = cipher.graphemes(true).collect::<VecDeque<&str>>();
        let processed_len = processed_text.len();

        let mut rail_rows = (0..self.n_rows)
            .map(|row_num| self.num_in_row(row_num, processed_len))
            .fold(Vec::new(), |mut rows, num_in_row| {
                let mut row = processed_text.split_off(num_in_row);
                std::mem::swap(&mut row, &mut processed_text);
                rows.push(row);
                rows
            });

        match ZigIterMut::from(&mut rail_rows).try_fold(String::new(), |mut message, row| {
            if let Some(segment) = row.pop_front() {
                message.push_str(segment);
                Ok(message)
            } else {
                Err(message)
            }
        }) {
            Ok(_) => panic!("impossible"),
            Err(result) => result,
        }
    }

    fn create_array(&self, len: usize) -> Vec<Vec<&str>> {
        vec![vec![""; len]; self.n_rows]
    }

    fn num_in_row(&self, row_num: usize, total_len: usize) -> usize {
        let row_num = row_num as i32;
        let sign_n = (self.n_rows - 1) as i32;

        let dist_to_even_mult = |x: i32| sign_n - ((x % (2 * sign_n)) - sign_n).abs();

        (0..total_len).fold(0, move |sum, x| {
            if dist_to_even_mult(x as i32) == row_num {
                sum + 1
            } else {
                sum
            }
        })
    }

    fn get_row_num(&self, x: usize) -> usize {
        let sign_x = x as i32;
        let sign_n = (self.n_rows - 1) as i32;

        (sign_n - ((sign_x % (2 * sign_n)) - sign_n).abs()) as usize
    }
}

struct ZigIterMut<'a, T> {
    vec: &'a mut Vec<T>,
    index: usize,
    increase: bool,
}

impl<'a, T> From<&'a mut Vec<T>> for ZigIterMut<'a, T> {
    fn from(vec: &'a mut Vec<T>) -> Self {
        Self {
            vec,
            index: 0,
            increase: true,
        }
    }
}

impl<'a, T> Iterator for ZigIterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        let max_pos = self.vec.len() - 1;
        let current = self.index;

        match (self.index, self.increase) {
            (0, _) => {
                self.increase = true;
                self.index += 1;
            }
            (n, _) if n == max_pos => {
                self.increase = false;
                self.index -= 1;
            }
            (_, true) => self.index += 1,
            (_, false) => self.index -= 1,
        }
        let ptr = self.vec.as_mut_ptr();
        Some(unsafe { &mut *ptr.add(current) })
    }
}
