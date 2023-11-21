/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    // Clean input into a digit vector
    let mut digits = Vec::new();
    for (i, c) in isbn.char_indices() {
        if c.is_digit(10) {
            digits.push(c);
        } else if c == 'X' && i == isbn.len() - 1 {
            digits.push(c);
        } else if c == '-' {
            continue;
        } else {
            return false;
        }
    }

    // Check length
    if digits.len() != 10 {
        return false;
    }

    // Check checksum
    let mut sum = 0;
    for (i, c) in digits.iter().enumerate() {
        let value = if *c == 'X' {
            10
        } else {
            c.to_digit(10).unwrap()
        };
        sum += value * (10 - i as u32);
    }

    sum % 11 == 0
}
