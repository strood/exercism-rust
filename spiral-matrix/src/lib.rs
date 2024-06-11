pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    let mut res = (0..size)
        .map(|_| vec![0; size as usize])
        .collect::<Vec<_>>();
    let mut x = 0;
    let mut y = 0;
    let mut dx = 1;
    let mut dy = 0;
    let mut num = 1;
    let max = size * size;

    while num <= max {
        res[y as usize][x as usize] = num;
        num += 1;

        if x + dx >= size as i32
            || y + dy >= size as i32
            || x + dx < 0
            || y + dy < 0
            || res[(y + dy) as usize][(x + dx) as usize] != 0
        {
            let temp = dx;
            dx = -dy;
            dy = temp;
        }

        x += dx;
        y += dy;

        if x >= size as i32
            || y >= size as i32
            || x < 0
            || y < 0
            || res[y as usize][x as usize] != 0
        {
            break;
        }
    }

    res
}
