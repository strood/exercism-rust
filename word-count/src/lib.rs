use std::collections::HashMap;
/// Count occurrences of words.
pub fn word_count(words: &str) -> HashMap<String, u32> {
    words.split(|c:char| (!c.is_alphabetic() && !c.is_alphanumeric() && c != '\''))
        .map(|word| word.to_lowercase().trim_matches('\'').to_string())
        .filter(|word| !word.is_empty())
        .fold(HashMap::new(), |mut res, word| {
            *res.entry(word).or_insert(0) += 1;
            res
        })
}
