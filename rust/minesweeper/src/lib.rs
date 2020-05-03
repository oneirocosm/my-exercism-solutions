struct BoardState {
    n_mines: Vec<Vec<Option<u8>>>,
}

impl BoardState {
    fn new(n_rows: usize, n_cols: usize) -> Self {
        BoardState {
            n_mines: vec![vec![Some(0); n_cols]; n_rows],
        }
    }

    fn update_surround(&mut self, index: (usize, usize)) {
        let (mid_row, mid_col) = index;
        let top = mid_row.wrapping_sub(1);
        let bottom = mid_row.wrapping_add(1);
        let left = mid_col.wrapping_sub(1);
        let right = mid_col.wrapping_add(1);

        self.increment(top, left);
        self.increment(top, mid_col);
        self.increment(top, right);
        self.increment(mid_row, left);
        self.n_mines[mid_row][mid_col] = None;
        self.increment(mid_row, right);
        self.increment(bottom, left);
        self.increment(bottom, mid_col);
        self.increment(bottom, right);
    }

    fn increment(&mut self, row_index: usize, col_index: usize) -> Option<()> {
        match self.n_mines.get_mut(row_index)?.get_mut(col_index)? {
            None => {}
            Some(mine) => *mine += 1,
        }
        Some(())
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
        self.n_mines
            .into_iter()
            .map(|row| {
                row.iter()
                    .map(|x| Self::parse_square(*x))
                    .collect::<String>()
            })
            .collect()
    }
}

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    // handle corner cases
    let n_rows = minefield.len();
    if n_rows == 0 {
        return Vec::new();
    }
    let n_cols = minefield[0].len();
    if n_cols == 0 {
        return vec![String::new()];
    }

    // find index of every mine
    let mine_locs: Vec<(usize, usize)> = minefield
        .iter()
        .enumerate()
        .map(|(i, line)| {
            line.chars()
                .enumerate()
                .filter_map(move |(j, square)| match square {
                    '*' => Some((i, j)),
                    _ => None,
                })
        })
        .flatten()
        .collect();

    // for each mine, add 1 to surrounding tiles
    let mut board = BoardState::new(n_rows, n_cols);
    for mine in mine_locs {
        board.update_surround(mine);
    }

    board.into()
}
