struct Matrix<'a> {
    data: &'a [Vec<u64>],
    num_rows: usize,
    num_cols: usize,
}

impl<'a> Matrix<'a> {
    fn new(data: &'a [Vec<u64>]) -> Option<Self> {
        if data.is_empty() {
            return None;
        }
        if data.iter().any(|row| row.is_empty()) {
            return None;
        }

        Some(Matrix {
            data,
            num_rows: data.len(),
            num_cols: data[0].len(),
        })
    }

    fn saddle_points(&self) -> Vec<(usize, usize)> {
        let max_rows: Vec<u64> = (0..self.num_rows)
            .map(|index| self.max_row(index))
            .collect();
        let min_cols: Vec<u64> = (0..self.num_cols)
            .map(|index| self.min_col(index))
            .collect();

        let mut points: Vec<(usize, usize)> = Vec::new();
        for (i, row) in self.data.iter().enumerate() {
            for (j, &elem) in row.iter().enumerate() {
                if elem == max_rows[i] && elem == min_cols[j] {
                    points.push((i, j));
                }
            }
        }
        points
    }

    fn max_row(&self, index: usize) -> u64 {
        *self.data[index].iter().max().unwrap()
    }

    fn min_col(&self, index: usize) -> u64 {
        self.data.iter().map(|v| v[index]).min().unwrap()
    }
}

pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    match Matrix::new(input) {
        Some(matrix) => matrix.saddle_points(),
        None => Vec::new(),
    }
}
