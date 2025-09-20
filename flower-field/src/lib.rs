pub fn annotate(garden: &[&str]) -> Vec<String> {
    let height = garden.len();
    garden
        .iter()
        .enumerate()
        .fold(Vec::new(), |mut acc, (idx, &row)| {
            acc.push(
                row.as_bytes()
                    .iter()
                    .enumerate()
                    .map(|(jdx, &ch)| {
                        let mut flower_count = 0;
                        // check all surrounding spots
                        for di in -1..=1 {
                            for dj in -1..=1 {
                                if di == 0 && dj == 0 {
                                    continue; // skip the center spot
                                }
                                let ni = idx as isize + di;
                                let nj = jdx as isize + dj;
                                if ni >= 0
                                    && (ni as usize) < height
                                    && nj >= 0
                                    && (nj as usize) < row.len()
                                {
                                    if garden[ni as usize].as_bytes()[nj as usize] == 42 {
                                        flower_count += 1;
                                    }
                                }
                            }
                        }
                        if ch == 42 {
                            '*'
                        } else if flower_count > 0 {
                            std::char::from_digit(flower_count, 10).unwrap()
                        } else {
                            ' '
                        }
                    })
                    .collect(),
            );
            acc
        })
}
