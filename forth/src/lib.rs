pub type Value = i32;
pub type Result = std::result::Result<(), Error>;

pub struct Forth {
    stack: Vec<Value>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

impl Forth {
    pub fn new() -> Forth {
        Forth { stack: Vec::new() }
    }

    pub fn stack(&self) -> &[Value] {
        &self.stack
    }

    pub fn eval(&mut self, input: &str) -> Result {
        let mut words = input.to_lowercase().split_whitespace().peekable();
        while let Some(word) = words.next() {
            match word {
                "+" => self.add(),
                "-" => self.sub(),
                "*" => self.mul(),
                "/" => self.div(),
                "dup" => self.dup(),
                "drop" => self.drop(),
                "swap" => self.swap(),
                "over" => self.over(),
                _ => {
                    if let Ok(num) = word.parse::<Value>() {
                        self.stack.push(num);
                    } else {
                        return Err(Error::UnknownWord);
                    }
                }
            }
        }
        Ok(())
    }
}
