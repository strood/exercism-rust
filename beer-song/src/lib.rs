pub fn verse(n: u32) -> String {
    if n == 0 {
        return format!(
            "No more bottles of beer on the wall, no more bottles of beer.\n\
             Go to the store and buy some more, 99 bottles of beer on the wall.\n"
        );
    }

    format!(
        "{0} of beer on the wall, {0} of beer.\n{1}, {2} of beer on the wall.\n",
        bottles(n),
        action(n),
        bottles(n - 1)
    )
}

pub fn bottles(n: u32) -> String {
    match n {
        0 => "no more bottles".to_string(),
        1 => "1 bottle".to_string(),
        _ => format!("{} bottles", n),
    }
}

pub fn action(n: u32) -> String {
    match n {
        0 => "Go to the store and buy some more".to_string(),
        1 => "Take it down and pass it around".to_string(),
        _ => "Take one down and pass it around".to_string(),
    }
}

pub fn sing(start: u32, end: u32) -> String {
    (end..=start)
        .rev()
        .map(|n| verse(n))
        .collect::<Vec<_>>()
        .join("\n")
}
