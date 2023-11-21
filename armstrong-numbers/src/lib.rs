pub fn is_armstrong_number(num: u32) -> bool {
    if num == 0 {
        return true;
    }
    let num = u64::from(num); // Boost it up to avoid overflow of u32
    let exp = (num as f64).log10() as u32 + 1; // Get our exponent

    let mut current = num as u64;
    let mut total = 0;
    while current > 0 {
        total += (current % 10).pow(exp);
        current /= 10;
    }

    // Check if armstrong number
    num == total
}
