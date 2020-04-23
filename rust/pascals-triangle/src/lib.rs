use num_integer::binomial;

pub struct PascalsTriangle {
    rows: Vec<Vec<u32>>,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        let rows: Vec<Vec<u32>> = (0..row_count)
            .map(|row| (0..=row).map(|elem| binomial(row, elem)).collect())
            .collect();
        PascalsTriangle { rows }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        self.rows.clone()
    }
}
