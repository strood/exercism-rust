// Previously used a hashset to check for duplicate letters, but doing bit field
// comparison is much faster, so tried that on second iteration
const A_LCASE: u8 = b'a';

pub fn check(candidate: &str) -> bool {
    candidate
        .bytes() // iterate through u8 bytes of string
        .filter(|&c| c.is_ascii_alphabetic())
        .map(|c| 1u32 << (c.to_ascii_lowercase() - A_LCASE))
        .try_fold(0u32, |ltr_flags, ltr| {
            (ltr_flags & ltr == 0).then_some(ltr_flags | ltr) // Bitwise AND to check if ltr is already in ltr_flags
                                                              // If not, return Some(ltr_flags | ltr) to set the bit(bitwise OR)
        })
        .is_some()
}
