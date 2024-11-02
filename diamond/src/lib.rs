pub fn get_diamond(c: char) -> Vec<String> {
    let n = c as u8 - b'A' + 1;
    (0..n).chain((0..(n - 1)).rev()).map(|i| {
        let letter = (b'A' + i) as char;
        let outer = " ".repeat((n - 1 - i) as usize);
        let mid = match i {
            0..1 => " ".repeat(i as usize),
            _ => " ".repeat((i * 2 - 1) as usize),
        };
        if i == 0 {
            format!("{}{}{}", outer, letter, outer)
        } else {
            format!("{}{}{}{}{}", outer, letter, mid, letter, outer)
        }
    }).collect()
}
