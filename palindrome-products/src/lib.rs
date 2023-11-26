use std::collections::HashMap;
/// `Palindrome` is a newtype which only exists when the contained value is a palindrome number in base ten.
///
/// A struct with a single field which is used to constrain behavior like this is called a "newtype", and its use is
/// often referred to as the "newtype pattern". This is a fairly common pattern in Rust.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Palindrome(u64);

impl Palindrome {
    /// Create a `Palindrome` only if `value` is in fact a palindrome when represented in base ten. Otherwise, `None`.
    pub fn new(value: u64) -> Option<Palindrome> {
        if Palindrome::is_palindrome(value) {
            Some(Palindrome(value))
        } else {
            None
        }
    }

    /// Get the value of this palindrome.
    pub fn into_inner(self) -> u64 {
        self.0
    }

    pub fn is_palindrome(mut n: u64) -> bool {
        let mut reversed: u64 = 0;
        let original: u64 = n;

        while n > 0 {
            let digit: u64 = n % 10;
            reversed = reversed * 10 + digit;
            n /= 10;
        }

        reversed == original
    }
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    if min > max {
        None
    } else {
        // let mut products: Vec<u64> = Vec::new();
        let mut products: HashMap<u64, Vec<(u64, u64)>> = HashMap::new();
        let mut min_palindrome: u64 = u64::MAX;
        let mut max_palindrome: u64 = u64::MIN;
        // Need to find all our unique products that can be produced by multiplying two numbers between min and max
        for i in min..=max {
            for j in i..=max {
                // try to create palindrome, if it works, add to products
                let product = i * j;
                if Palindrome::is_palindrome(product) {
                    if product < min_palindrome {
                        min_palindrome = product;
                    }
                    if product > max_palindrome {
                        max_palindrome = product;
                    }
                    products.entry(product).or_default().push((i, j));
                }
            }
        }

        if products.is_empty() {
            return None;
        }

        Some((
            Palindrome::new(min_palindrome).unwrap(),
            Palindrome::new(max_palindrome).unwrap(),
        ))
    }
}
