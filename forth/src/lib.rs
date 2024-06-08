use std::collections::{HashMap, VecDeque};

pub type Value = i32;
pub type Result = std::result::Result<(), Error>;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Instruction {
    Add,
    Sub,
    Mul,
    Div,
    Dup,
    Drop,
    Swap,
    Over,
    Custom(String),
    Value(Value),
    Def(String),
}

impl Instruction {
    pub fn get_custom_string(&self) -> Option<&String> {
        if let Instruction::Custom(string) = self {
            Some(string)
        } else if let Instruction::Def(string) = self {
            Some(string)
        } else {
            None
        }
    }
}

impl Default for Forth {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Forth {
    stack: Vec<Value>, // Value(Value) - Stack of values
    custom_words: HashMap<String, VecDeque<Instruction>>, // Custom(String) - Custom words, mapping to current version of definition
    definitions: HashMap<String, Vec<Instruction>>,       // Definitions of custom words
}

impl Forth {
    pub fn new() -> Forth {
        Forth {
            stack: Vec::new(),
            custom_words: HashMap::new(),
            definitions: HashMap::new(),
        }
    }

    pub fn stack(&self) -> &[Value] {
        &self.stack
    }

    pub fn eval(&mut self, input: &str) -> Result {
        let mut open = false;
        let (chunks, errors) =
            input
                .split(' ')
                .fold((vec![Vec::new()], 0), |(mut chunks, mut errors), word| {
                    match word {
                        ":" => {
                            if open {
                                errors += 1;
                            }
                            open = true;
                            chunks.push(vec![Instruction::Def(word.to_string())]);
                        }
                        ";" => {
                            if !open {
                                errors += 1;
                            }
                            open = false;
                            chunks
                                .last_mut()
                                .unwrap()
                                .push(Instruction::Def(word.to_string()));
                            chunks.push(Vec::new());
                        }
                        _ => {
                            let val = if let Some(current_chunk) = chunks.last() {
                                let current_chunk_length = current_chunk.len();
                                let current_chunk_last = current_chunk.last();
                                current_chunk_length == 1
                                    && current_chunk_last.map_or(false, |last| {
                                        last.get_custom_string().unwrap_or(&"".to_string()) == ":"
                                    })
                            } else {
                                false
                            };

                            let instruction = self.match_str(word, &val);
                            chunks.last_mut().unwrap().push(instruction);
                        }
                    }
                    (chunks, errors)
                });

        if errors > 0 || open {
            Err(Error::InvalidWord)
        } else {
            self.eval_chunks(chunks)?;
            Ok(())
        }
    }

    pub fn eval_chunks(&mut self, chunks: Vec<Vec<Instruction>>) -> Result {
        for chunk in chunks {
            // Check the first value of the chunk, if definition treat as so
            // Else iterate through instructions evaling each
            if let Some(Instruction::Def(_word)) = chunk.first() {
                // Handle definition
                let words = chunk[1..(chunk.len() - 1)].to_vec();
                let name = words
                    .first()
                    .ok_or(Error::InvalidWord)?
                    .get_custom_string()
                    .ok_or(Error::InvalidWord)?;
                if name.parse::<f64>().is_ok() {
                    return Err(Error::InvalidWord);
                }

                let definition = words[1..].to_vec();

                if self.custom_words.contains_key(name) {
                    // Add new version of custom word
                    let mut custom_word_versions = self.custom_words.get_mut(name).unwrap().clone();
                    let custom_word_hash = name.to_lowercase().to_string()
                        + custom_word_versions.len().to_string().as_str();
                    custom_word_versions.push_back(Instruction::Custom(custom_word_hash.clone()));
                    self.definitions.insert(custom_word_hash, definition);

                    self.custom_words
                        .insert(name.to_lowercase().to_string(), custom_word_versions);
                } else {
                    // Create new custom word
                    let mut custom_word_versions = VecDeque::new();
                    let custom_word_hash = name.to_lowercase().to_string()
                        + custom_word_versions.len().to_string().as_str();
                    custom_word_versions.push_back(Instruction::Custom(custom_word_hash.clone()));
                    self.definitions
                        .insert(custom_word_hash.clone(), definition);
                    self.custom_words
                        .insert(name.to_lowercase().to_string(), custom_word_versions);
                }

                continue;
            }

            for instruction in chunk {
                self.eval_instruction(instruction)?;
            }
        }

        Ok(())
    }

    pub fn eval_instruction(&mut self, instruction: Instruction) -> Result {
        match instruction {
            Instruction::Custom(word) => {
                if let Some(definition) = self.definitions.get(&word) {
                    // Eval custom words within custom word value at time of definition
                    self.eval_chunks(vec![definition.clone()])?;
                } else if let Some(definition_keys) = self.custom_words.get(&word) {
                    // Eval custom word values
                    let current_definition_key = definition_keys.back().unwrap();
                    let definition = self
                        .definitions
                        .get(current_definition_key.get_custom_string().unwrap())
                        .unwrap();
                    self.eval_chunks(vec![definition.clone()])?;
                } else {
                    return Err(Error::UnknownWord);
                }
                Ok(())
            }
            Instruction::Value(value) => self.push(value),
            Instruction::Add => self.add(),
            Instruction::Sub => self.sub(),
            Instruction::Mul => self.mul(),
            Instruction::Div => self.div(),
            Instruction::Dup => self.dup(),
            Instruction::Drop => self.drop(),
            Instruction::Swap => self.swap(),
            Instruction::Over => self.over(),

            _ => Ok(()),
        }
    }

    pub fn match_str(&self, byte: &str, first_open: &bool) -> Instruction {
        if first_open == &true {
            return Instruction::Custom(byte.to_string().to_lowercase().to_string());
        }
        if let Some(definition) = self.custom_words.get(byte.to_lowercase().as_str()) {
            return Instruction::Custom(
                definition
                    .back()
                    .unwrap()
                    .get_custom_string()
                    .unwrap()
                    .to_string(),
            );
        }
        if self.custom_words.contains_key(byte) {
            return Instruction::Custom(byte.to_string().to_lowercase().to_string());
        }
        match byte.to_lowercase().as_str() {
            "+" => Instruction::Add,
            "-" => Instruction::Sub,
            "*" => Instruction::Mul,
            "/" => Instruction::Div,
            "dup" => Instruction::Dup,
            "drop" => Instruction::Drop,
            "swap" => Instruction::Swap,
            "over" => Instruction::Over,
            _ => {
                if let Ok(val) = byte.parse::<Value>() {
                    Instruction::Value(val)
                } else {
                    Instruction::Custom(byte.to_string().to_lowercase().to_string())
                }
            }
        }
    }

    fn push(&mut self, value: Value) -> Result {
        self.stack.push(value);
        Ok(())
    }

    fn add(&mut self) -> Result {
        if let (Some(a), Some(b)) = (self.stack.pop(), self.stack.pop()) {
            self.stack.push(a + b);
            Ok(())
        } else {
            Err(Error::StackUnderflow)
        }
    }

    fn sub(&mut self) -> Result {
        if let (Some(a), Some(b)) = (self.stack.pop(), self.stack.pop()) {
            self.stack.push(b - a);
            Ok(())
        } else {
            Err(Error::StackUnderflow)
        }
    }

    fn mul(&mut self) -> Result {
        if let (Some(a), Some(b)) = (self.stack.pop(), self.stack.pop()) {
            self.stack.push(a * b);
            Ok(())
        } else {
            Err(Error::StackUnderflow)
        }
    }

    fn div(&mut self) -> Result {
        if let (Some(a), Some(b)) = (self.stack.pop(), self.stack.pop()) {
            if a == 0 || b == 0 {
                Err(Error::DivisionByZero)
            } else {
                self.stack.push(b / a);
                Ok(())
            }
        } else {
            Err(Error::StackUnderflow)
        }
    }

    fn dup(&mut self) -> Result {
        if let Some(&a) = self.stack.last() {
            self.stack.push(a);
            Ok(())
        } else {
            Err(Error::StackUnderflow)
        }
    }

    fn drop(&mut self) -> Result {
        if let Some(_a) = self.stack.pop() {
            Ok(())
        } else {
            Err(Error::StackUnderflow)
        }
    }

    fn swap(&mut self) -> Result {
        if let (Some(a), Some(b)) = (self.stack.pop(), self.stack.pop()) {
            self.stack.push(a);
            self.stack.push(b);
            Ok(())
        } else {
            Err(Error::StackUnderflow)
        }
    }

    fn over(&mut self) -> Result {
        if let (Some(a), Some(b)) = (self.stack.pop(), self.stack.pop()) {
            self.stack.push(b);
            self.stack.push(a);
            self.stack.push(b);
            Ok(())
        } else {
            Err(Error::StackUnderflow)
        }
    }
}
