pub fn abbreviate(phrase: &str) -> String {
    let mut acronym = String::new();
    let mut prev_char = ' ';

    for c in phrase.chars() {
        if c.is_alphabetic() && (!prev_char.is_alphabetic() && prev_char != '\'') {
            acronym.push(c.to_ascii_uppercase());
        } else if c.is_uppercase() && !prev_char.is_uppercase() {
            acronym.push(c);
        }
        prev_char = c;
    }

    acronym
}
