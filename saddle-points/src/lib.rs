pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let row_count = input.len();
    input
        .iter()
        .enumerate()
        .fold(Vec::new(), |mut result, (row_num, row)| {
            // Check each value in each row
            for (col_num, val) in row.iter().enumerate() {
                // If largest or equal to in its row, and smaller than all values in its column, push to results
                if row.iter().all(|x| x <= val) && (0..row_count).all(|x| input[x][col_num] >= *val)
                {
                    result.push((row_num, col_num));
                }
            }
            result
        })
}
