use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
  let mut results = HashSet::new();

  let lowercase_word = word.to_lowercase();
  let mut word_chars = lowercase_word.chars().collect::<Vec<char>>();
  word_chars.sort_unstable();

  for possible_anagram in possible_anagrams {
    let lowercase_anagram = possible_anagram.to_lowercase();
    if lowercase_anagram == lowercase_word {
      continue;
    }

    let mut anagram_chars = lowercase_anagram.chars().collect::<Vec<char>>();
    println!("{:?}", anagram_chars);
    anagram_chars.sort_unstable();
    
    if anagram_chars == word_chars {
      results.insert(*possible_anagram);
    }

  }
  

  return results;


}
