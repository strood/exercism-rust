use std::fmt::{Display, Formatter, Result};
const ROMAN_SYMBOLS: [(u32, &str); 13] = [
    (1000, "M"),
    (900, "CM"),
    (500, "D"),
    (400, "CD"),
    (100, "C"),
    (90, "XC"),
    (50, "L"),
    (40, "XL"),
    (10, "X"),
    (9, "IX"),
    (5, "V"),
    (4, "IV"),
    (1, "I"),
];

pub struct Roman {
    value: String,
}

impl Display for Roman {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.value)
    }
}

impl From<u32> for Roman {
    fn from(num: u32) -> Self {
        let mut num = num;
        Roman { 
            value: ROMAN_SYMBOLS.iter().fold(String::new(), |mut acc, (value, symbol)| {
                while num >= *value {
                    acc.push_str(symbol);
                    num -= value;
                }
                acc
            })
        }
    }
}
