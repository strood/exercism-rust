pub fn annotate(minefield: &[&str]) -> Vec<String> {
    if minefield.is_empty() {
        return Vec::new();
    }

    const MINE: u8 = b'*';

    let mut result: Vec<String> = Vec::new();
    let rows = minefield.len();
    let cols = minefield[0].len();
    let offsets: [(isize, isize); 8] = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    for (i, row) in minefield.iter().enumerate() {
        let mut row_result = String::with_capacity(cols);

        for (j, &cell) in row.as_bytes().iter().enumerate() {
            if cell == MINE {
                row_result.push('*');
            } else {
                let count = offsets.iter().fold(0, |acc, &(dx, dy)| {
                    let x = i as isize + dx as isize;
                    let y = j as isize + dy as isize;

                    if x >= 0 && x < rows as isize && y >= 0 && y < cols as isize {
                        acc + (minefield[x as usize].as_bytes()[y as usize] == MINE) as u8
                    } else {
                        acc
                    }
                });

                if count > 0 {
                    row_result.push_str(&count.to_string());
                } else {
                    row_result.push(' ');
                }
            }
        }

        result.push(row_result);
    }

    result
}
