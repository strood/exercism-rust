pub fn plants(_diagram: &str, _student: &str) -> Vec<&'static str> {
    // Match the given student, then return the result of a plant decoding function we will make
    match _student {
        "Alice" => decode_plants(_diagram, 0),
        "Bob" => decode_plants(_diagram, 2),
        "Charlie" => decode_plants(_diagram, 4),
        "David" => decode_plants(_diagram, 6),
        "Eve" => decode_plants(_diagram, 8),
        "Fred" => decode_plants(_diagram, 10),
        "Ginny" => decode_plants(_diagram, 12),
        "Harriet" => decode_plants(_diagram, 14),
        "Ileana" => decode_plants(_diagram, 16),
        "Joseph" => decode_plants(_diagram, 18),
        "Kincaid" => decode_plants(_diagram, 20),
        "Larry" => decode_plants(_diagram, 22),
        _ => vec![], // Default case, return an empty vector
    }
}

pub fn decode_plants(_diagram: &str, _index: usize) -> Vec<&'static str> {
    // Create a vector to hold the plants
    let mut plants = Vec::new();

    // Split the diagram into lines
    let lines: Vec<&str> = _diagram.lines().collect();

    // Iterate over the lines and push the plants into the vector for students position/index
    for line in lines.iter() {
        for plant in line.chars().skip(_index).take(2) {
            match plant {
                'V' => plants.push("violets"),
                'R' => plants.push("radishes"),
                'C' => plants.push("clover"),
                'G' => plants.push("grass"),
                _ => (),
            }
        }
    }

    // Return the vector
    plants
}
