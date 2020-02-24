struct BoardState {
    n_cols: usize,
    n_mines: Vec<Option<u8>>,
}

impl BoardState {
    fn new(n_rows: usize, n_cols: usize) -> Self {
        let size: usize = n_rows * n_cols;
        BoardState {
            n_cols,
            n_mines: vec![Some(0); size],
        }
    }

    fn update_surround(&mut self, index: usize) {

        let to_update = self.find_squares_to_update(index);

        self.n_mines[index] = None;
        for surround in to_update.iter() {
            if let Some(current) = self.n_mines[*surround] {
                self.n_mines[*surround] = Some(current + 1);
            }
        }
    }
    fn find_squares_to_update(&self, index: usize) -> Vec<usize> {
        let has_left = self.has_left(index);
        let has_right = self.has_right(index);
        let has_top = self.has_top(index);
        let has_bottom = self.has_bottom(index);
        
        let mut to_update: Vec<usize> = Vec::new();
        if has_left && has_top {
            to_update.push(index - self.n_cols - 1);
        }
        if has_top {
            to_update.push(index - self.n_cols);
        }
        if has_right && has_top {
            to_update.push(index - self.n_cols + 1);
        }
        if has_left {
            to_update.push(index - 1);
        }
        if has_right {
            to_update.push(index + 1);
        }
        if has_left && has_bottom {
            to_update.push(index + self.n_cols - 1);
        }
        if has_bottom {
            to_update.push(index + self.n_cols);
        }
        if has_right && has_bottom {
            to_update.push(index + self.n_cols + 1);
        }

        to_update
    }

    fn has_left(&self, index: usize) -> bool {
        index % self.n_cols != 0
    }

    fn has_right(&self, index: usize) -> bool {
        (index + 1) % self.n_cols != 0
    }

    fn has_top(&self, index: usize) -> bool {
        index >= self.n_cols
    }

    fn has_bottom(&self, index: usize) -> bool {
        let total = self.n_mines.len();
        index < total - self.n_cols
    }
    fn parse_square(square: Option<u8>) -> char {
        match square {
            None => '*',
            Some(0) => ' ',
            Some(n) => std::char::from_digit(n.into(), 10).unwrap(),
        }
    }
}

impl Into<Vec<String>> for BoardState {
    fn into(self) -> Vec<String> {
        let n_mines: Vec<char> = self
            .n_mines
            .iter()
            .map(|square| Self::parse_square(*square))
            .collect();
        n_mines
            .chunks(self.n_cols)
            .map(|row| row.iter().collect())
            .collect()
    }
}

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    // handle corner cases
    let n_rows = minefield.len();
    if n_rows == 0 {
        return Vec::new();
    }
    let n_cols = if n_rows > 0 { minefield[0].len() } else { 0 };
    if n_cols == 0 {
        return vec![String::new()];
    }

    // find index of every mine
    let mine_locs = minefield
        .iter()
        .map(|line| line.chars())
        .flatten()
        .enumerate()
        .filter_map(|(i, square)| if is_mine(square) { Some(i) } else { None });

    // for each mine, add 1 to surrounding tiles
    let mut board = BoardState::new(n_rows, n_cols);
    for mine in mine_locs {
        board.update_surround(mine);
    }

    board.into()
}

fn is_mine(square: char) -> bool {
    match square {
        '*' => true,
        _ => false,
    }
}
