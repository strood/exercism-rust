/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    let mut count = 0;
    plain.chars().fold(String::new(), |mut acc, c| {
        let is_alphabetic = c.is_alphabetic();
        if c.is_numeric() || is_alphabetic {
            if is_alphabetic {
                acc.push(find_letter(c.to_ascii_lowercase()));
            } else {
                acc.push(c);
            }
            count += 1;
            if count % 5 == 0 {
                acc.push(' ');
                count = 0;
            }
        }
        acc
    }).trim().to_string()
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    cipher.chars().fold(String::new(), |mut acc, c| {
        if c.is_numeric() {
            acc.push(c);
        }
        if c.is_alphabetic() {
            acc.push(find_letter(c));
        }
        acc
    })
}

pub fn find_letter(c: char) -> char {
    match c {
        'a' => 'z',
        'b' => 'y',
        'c' => 'x',
        'd' => 'w',
        'e' => 'v',
        'f' => 'u',
        'g' => 't',
        'h' => 's',
        'i' => 'r',
        'j' => 'q',
        'k' => 'p',
        'l' => 'o',
        'm' => 'n',
        'n' => 'm',
        'o' => 'l',
        'p' => 'k',
        'q' => 'j',
        'r' => 'i',
        's' => 'h',
        't' => 'g',
        'u' => 'f',
        'v' => 'e',
        'w' => 'd',
        'x' => 'c',
        'y' => 'b',
        'z' => 'a',
        _ => c,
    }
}