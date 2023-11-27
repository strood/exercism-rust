pub struct PascalsTriangle(u32);

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        PascalsTriangle(row_count)
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        let mut rows: Vec<Vec<u32>> = Vec::with_capacity(self.0 as usize);

        for row in 0..self.0 {
            let mut row_vec: Vec<u32> = Vec::with_capacity(row as usize + 1);

            for col in 0..=row {
                let value = if col > 0 && col < row {
                    let prev_row_index = (row - 1) as usize;
                    let col_index = col as usize;
                    let prev_row = &rows[prev_row_index];
                    prev_row[col_index - 1] + prev_row[col_index]
                } else {
                    1
                };

                row_vec.push(value);
            }

            rows.push(row_vec);
        }

        rows
    }
}
