pub fn series(digits: &str, len: usize) -> Vec<String> {
    // Get all series of length len from digits
    let mut series: Vec<String> = Vec::new();
    let mut i: usize = 0;

    while i + len <= digits.len() {
        series.push(digits[i..i + len].to_string());
        i += 1;
    }

    series
}
