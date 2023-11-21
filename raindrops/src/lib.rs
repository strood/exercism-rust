pub fn raindrops(n: u32) -> String {
    // Convert a number to a string, the contents of which depend on the number's factors.
    let mut result = String::new();

    if n % 3 == 0 {
        result.push_str("Pling");
    }

    if n % 5 == 0 {
        result.push_str("Plang");
    }

    if n % 7 == 0 {
        result.push_str("Plong");
    }

    if result.is_empty() {
        result.push_str(&n.to_string());
    }

    result
}
