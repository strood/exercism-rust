pub fn recite(start_bottles: u32, take_down: u32) -> String {
    let mut verses = Vec::new();

    for i in 0..take_down {
        let bottles = start_bottles - i;
        let next_bottles = if bottles > 1 { bottles - 1 } else { 0 };

        let verse = match bottles {
            0 => String::new(),
            1 => format!(
                "One green bottle hanging on the wall,\n\
                 One green bottle hanging on the wall,\n\
                 And if one green bottle should accidentally fall,\n\
                 There'll be no green bottles hanging on the wall."
            ),
            2 => format!(
                "Two green bottles hanging on the wall,\n\
                 Two green bottles hanging on the wall,\n\
                 And if one green bottle should accidentally fall,\n\
                 There'll be one green bottle hanging on the wall."
            ),
            _ => format!(
                "{} green bottles hanging on the wall,\n\
                 {} green bottles hanging on the wall,\n\
                 And if one green bottle should accidentally fall,\n\
                 There'll be {} green bottles hanging on the wall.",
                capitalize_first(&convert_number_to_words(bottles)),
                capitalize_first(&convert_number_to_words(bottles)),
                convert_number_to_words(next_bottles)
            ),
        };

        verses.push(verse);
    }

    verses.join("\n\n")
}

fn capitalize_first(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

fn convert_number_to_words(n: u32) -> String {
    match n {
        0 => "no".to_string(),
        1 => "one".to_string(),
        2 => "two".to_string(),
        3 => "three".to_string(),
        4 => "four".to_string(),
        5 => "five".to_string(),
        6 => "six".to_string(),
        7 => "seven".to_string(),
        8 => "eight".to_string(),
        9 => "nine".to_string(),
        10 => "ten".to_string(),
        _ => n.to_string(),
    }
}
