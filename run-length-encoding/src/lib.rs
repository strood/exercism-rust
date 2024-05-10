pub fn encode(source: &str) -> String {
    let mut encoded = String::new();
    let mut count = 1;
    let mut prev = source.chars().next().unwrap_or('\0');
    for c in source.chars().skip(1) {
        if c == prev {
            count += 1;
        } else {
            if count > 1 {
                encoded.push_str(&count.to_string());
            }
            encoded.push(prev);
            count = 1;
            prev = c;
        }
    }

    if count > 1 {
        encoded.push_str(&count.to_string());
    }
    if prev != '\0' {
        encoded.push(prev);
    }

    encoded
}

pub fn decode(source: &str) -> String {
    let mut decoded = String::new();
    let mut count = String::new();
    for c in source.chars() {
        if c.is_ascii_digit() {
            count.push(c);
        } else {
            let times = if count.is_empty() {
                1
            } else {
                count.parse().unwrap()
            };
            decoded.push_str(&c.to_string().repeat(times));
            count.clear();
        }
    }
    decoded
}
