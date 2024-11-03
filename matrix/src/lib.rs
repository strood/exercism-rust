pub struct Matrix {
    matrix: Vec<Vec<u32>>
}

impl Matrix {
    pub fn new(input: &str) -> Self {
        Self { matrix: input.lines().map(|line| {
            line.split_whitespace().map(|num| num.parse().unwrap()).collect()
        }).collect() }
    }

    pub fn row(&self, row_no: usize) -> Option<Vec<u32>> {
        self.matrix.get(row_no - 1).cloned()
    }

    pub fn column(&self, col_no: usize) -> Option<Vec<u32>> {
        self.matrix.iter().map(|row| row.get(col_no - 1).cloned()).collect()
    }
}
