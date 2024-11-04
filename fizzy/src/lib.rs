use std::ops::Rem;

/// A Matcher is a single rule of fizzbuzz: given a function on T, should
/// a word be substituted in? If yes, which word?
pub struct Matcher<T> {
    matcher:fn(T) -> bool,
    subs: String,
}

impl<T> Matcher<T> {
    pub fn new<S: ToString>(matcher: fn(T) -> bool, subs: S) -> Matcher<T> 
    {
        Matcher {
            matcher: matcher,
            subs: subs.to_string(),
        }
    }
}

pub struct Fizzy<T>{
    matchers: Vec<Matcher<T>>,
}

impl<T: ToString + Clone> Fizzy<T>
{
    pub fn new() -> Self {
        Self {
            matchers: vec![],
        }
    }

    #[must_use]
    pub fn add_matcher(mut self, matcher: Matcher<T>) -> Self {
        self.matchers.push(matcher);
        self
    }

    /// map this fizzy onto every element of an iterator, returning a new iterator
    pub fn apply<I>(self, iter: I) -> impl Iterator<Item = String> 
    where 
        I: Iterator<Item = T>,
    {
        iter.map(move |n| {
            let result: String = self
                .matchers
                .iter()
                .filter(|m| (m.matcher)(n.clone()))
                .map(|m| m.subs.clone())
                .collect();

            if result.is_empty() {
                n.to_string()
            } else {
                result
            }
        }).into_iter()
    }
}

/// convenience function: return a Fizzy which applies the standard fizz-buzz rules
pub fn fizz_buzz<T>() -> Fizzy<T> 
where
    T: Rem<Output = T> + PartialEq + From<u8> + Copy + ToString
{
    Fizzy::new()
        .add_matcher(Matcher::new(|n| n % 3.into() == 0.into(), "fizz"))
        .add_matcher(Matcher::new(|n| n % 5.into() == 0.into(), "buzz"))
        
}
