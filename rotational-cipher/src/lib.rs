pub fn rotate(input: &str, key: u8) -> String {
    input.chars().into_iter().fold(String::new(), |mut acc, c| {
        if c.is_alphabetic() {
            let base = if c.is_lowercase() { b'a' } else { b'A' };
            let offset = c as u8 - base;
            let rotated = (offset + key) % 26;
            acc.push((base + rotated) as char);
        } else {
            acc.push(c);
        }
        acc
    })
}
