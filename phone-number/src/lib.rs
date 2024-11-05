pub fn number(user_number: &str) -> Option<String> {
    let clean_string = user_number.chars().filter(|c| c.is_digit(10)).collect::<String>();
    match clean_string.len() {
        10 => {
            if clean_string.starts_with("0") || clean_string.starts_with("1") 
                || clean_string.chars().nth(3).unwrap() == '0' || clean_string.chars().nth(3).unwrap() == '1' {
                None
            } else {
                Some(clean_string)
            }
        }
        11 => {
            if clean_string.chars().nth(0).unwrap() != '1' ||
                clean_string.chars().nth(1).unwrap() == '0' || clean_string.chars().nth(1).unwrap() == '1' ||
                clean_string.chars().nth(4).unwrap() == '0' || clean_string.chars().nth(4).unwrap() == '1' {
                 None
                } else {
                 Some(clean_string.chars().skip(1).collect())
                }
        }
        _ => None
    }
}
