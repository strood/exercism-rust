pub fn actions(n: u8) -> Vec<&'static str> {
    // Convert the number to binary and reverse it
    let mut binary = format!("{:b}", n).chars().rev().collect::<String>();

    // Check each char of the binary string and add the corresponding action to the vector
    let mut actions: Vec<&str> = Vec::new();

    for (i, c) in binary.chars().enumerate() {
        if c == '1' {
            match i {
                0 => actions.push("wink"),
                1 => actions.push("double blink"),
                2 => actions.push("close your eyes"),
                3 => actions.push("jump"),
                4 => actions.reverse(),
                _ => (),
            }
        }
    }

    actions
}
