pub fn reply(message: &str) -> &str {
    let message = message.trim();

    match message {
        m if m.is_empty() => "Fine. Be that way!",
        m if m.ends_with("?") && m.to_uppercase() == m && m.to_lowercase() != m => {
            "Calm down, I know what I'm doing!"
        }
        m if m.ends_with("?") => "Sure.",
        m if m.to_uppercase() == m && m.to_lowercase() != m => "Whoa, chill out!",
        _ => "Whatever.",
    }
}
