pub fn encode(key: &str, s: &str) -> Option<String> {
    if !is_valid_key(key) {
        return None;
    }
    let mut valid = true;
    let mut res = String::new();

    while res.len() < s.len() && valid {
        for (i, c) in s.char_indices() {
            if c.is_alphabetic() {
                let shift = key.chars().nth(i % key.len()).unwrap() as u8 - 'a' as u8;
                let new_c = (((c as u8 - b'a' + shift) % 26) + b'a') as char;
                res.push(new_c);
            } else {
                valid = false;
            }
        }
    }

    if valid {
        Some(res)
    } else {
        None
    }
}

pub fn decode(key: &str, s: &str) -> Option<String> {
    if !is_valid_key(key) {
        return None;
    }
    let mut valid = true;
    let mut res = String::new();

    while res.len() < s.len() && valid {
        for (i, c) in s.char_indices() {
            if c.is_alphabetic() {
                let shift = key.chars().nth(i % key.len()).unwrap() as u8 - 'a' as u8;
                let base = if c.is_lowercase() { b'a' } else { b'A' };
                let new_c = ((c as u8 + 26 - shift - base) % 26 + base) as char;
                res.push(new_c);
            } else {
                valid = false;
                break;
            }
        }
    }

    if valid {
        Some(res)
    } else {
        None
    }
}

pub fn encode_random(s: &str) -> (String, String) {
    let key = String::from_utf8((0..100).map(|_| rand::random::<u8>() % 26 + b'a').collect::<Vec<u8>>()).unwrap();
    let encoded = encode(&key, s).unwrap();
    (key, encoded)
}


fn is_valid_key(key: &str) -> bool {
    !key.is_empty() && key.chars().all(|c| c.is_ascii_lowercase())
}