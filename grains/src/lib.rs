pub fn square(s: u32) -> u64 {
    if s < 1 || s > 64 {
        panic!("Square must be between 1 and 64");
    }
    return 2u64.pow(s - 1);
}

pub fn total() -> u64 {
    (1..=64).map(square).sum() // Need to do this to avoid overflow error w/ 2^64
}
