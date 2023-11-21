use unicode_reverse::reverse_grapheme_clusters_in_place;

pub fn reverse(input: &str) -> String {
  // This workd for normal chars, but when you encounter a grapheme cluster
  // THings get weird. See: https://crates.io/crates/unicode-reverse , the crate we will use explains it!
  // let mut chars: Vec<char> = input.chars().collect();
  // chars.reverse();
  // chars.iter().collect()

  // This is the way to do it with grapheme clusters
  // this will reverse it in place
  let mut output = input.to_string();
  reverse_grapheme_clusters_in_place(&mut output);
  output
}
