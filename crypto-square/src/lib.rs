pub fn encrypt(input: &str) -> String {
    // Normalize string
    let normalized: String = input
        .to_lowercase()
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .collect();

    // Find r and c
    let (r, c) = match normalized.len() {
        0 => (0, 0),
        _ => find_rectangle_dimensions(normalized.len())        
    };

    // Build matrix
    let mut matrix = vec![vec![' '; c]; r];
    for (i, ch) in normalized.chars().enumerate() {
        matrix[i / c][i % c] = ch;
    }

    // construt output from matrix
    (0..c).map(|i| {
        (0..r).map(|j| matrix[j][i]).collect::<String>()
    }).collect::<Vec<String>>().join(" ")
}

pub fn find_rectangle_dimensions(length: usize) -> (usize, usize) {
    let mut c = (length as f64).sqrt().ceil() as usize;
    let mut r = (length + c - 1) / c; // This ensures r * c >= length

    while c < r || c - r > 1 {
        c += 1;
        r = (length + c - 1) / c;
    }

    (r, c)
}
