const A_LCASE: u8 = b'a';

/// Determine whether a sentence is a pangram using a bit field
pub fn is_pangram(sentence: &str) -> bool {
    sentence
        .bytes()
        .filter(|&c| c.is_ascii_alphabetic())
        .map(|c| 1u32 << (c.to_ascii_lowercase() - A_LCASE))
        .fold(0u32, |ltr_flags, ltr| ltr_flags | ltr)
        .count_ones()
        == 26
}
