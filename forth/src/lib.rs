use std::collections::{HashMap, VecDeque};

pub type Value = i32;
pub type Result = std::result::Result<(), Error>;

pub struct Forth {
    stack: Vec<Value>,                               // Our stack
    custom_words: HashMap<String, VecDeque<String>>, // Custom word tracking/updating points to definitions via has from the key string
    definitions: HashMap<String, String>, // stores the definitions of custom words via their 'hash' from custom_words Deque
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

impl Default for Forth {
    fn default() -> Self {
        Self::new()
    }
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
                .bytes()
                .fold((vec![Vec::new()], 0), |(mut chunks, mut errors), byte| {
                    // For each byte, push to most recent chunk, if it is :, start a new array and push to output,
                    // and keep pushing the next byets, if it is ;, push to curent array and start a new one.
                    // Tracks bad opens or closes to bail later
                    match byte {
                        b':' => {
                            if open {
                                errors += 1
                            }
                            open = true;
                            chunks.push(vec![byte]);
                            (chunks, errors)
                        }
                        b';' => {
                            if !open {
                                errors += 1
                            }
                            open = false;
                            chunks.last_mut().unwrap().push(byte);
                            chunks.push(Vec::new());
                            (chunks, errors)
                        }
                        _ => {
                            chunks.last_mut().unwrap().push(byte);
                            (chunks, errors)
                        }
                    }
                });

        if errors > 0 || open {
            Err(Error::InvalidWord)
        } else {
            for chunk in chunks {
                let chunk = String::from_utf8(chunk).map_err(|_| Error::InvalidWord)?;
                self.eval_chunk(chunk.trim())?;
            }
            Ok(())
        }
    }

    fn eval_chunk(&mut self, chunk: &str) -> Result {
        // Check if it is a definition string and handle if so
        if chunk.starts_with(':') {
            let trimmed = chunk.trim_start_matches(':').trim_end_matches(';').trim();
            let mut words = trimmed.split_whitespace();
            let name = words.next().ok_or(Error::InvalidWord)?;
            // Need to check definitions if any nested definitons in there we want to store the hash of the definition
            // instead of the word so then we store it in time
            let definition = words
                .collect::<Vec<_>>()
                .iter()
                .map(|word| -> String {
                    if let Some(definition) = self.custom_words.get(word.to_lowercase().as_str()) {
                        return definition.back().unwrap().to_string();
                    } else {
                        word.to_string()
                    }
                })
                .collect::<Vec<_>>()
                .join(" ");

            if name.parse::<f64>().is_ok() {
                return Err(Error::InvalidWord);
            }

            if self.custom_words.contains_key(name) {
                let mut custom_word_versions = self.custom_words.get_mut(name).unwrap().clone();
                let custom_word_hash = name.to_lowercase().to_string()
                    + custom_word_versions.len().to_string().as_str();
                custom_word_versions.push_back(custom_word_hash.clone());
                self.definitions.insert(custom_word_hash, definition);

                self.custom_words
                    .insert(name.to_lowercase().to_string(), custom_word_versions);
            } else {
                let mut custom_word_versions = VecDeque::new();
                let custom_word_hash = name.to_lowercase().to_string()
                    + custom_word_versions.len().to_string().as_str();
                custom_word_versions.push_back(custom_word_hash.clone());
                self.definitions.insert(custom_word_hash, definition);

                self.custom_words.insert(
                    name.to_lowercase().to_string(),
                    custom_word_versions.clone(),
                );
            }

            return Ok(());
        }

        let instructions = chunk.split_whitespace().collect::<Vec<_>>().into_iter();

        for instruction in instructions {
            if let Some(definition) = self.custom_words.get(instruction.to_lowercase().as_str()) {
                let current_definiton = definition.back().unwrap().clone();
                self.eval_definition(&current_definiton)?;
                continue;
            }

            self.eval_basic(instruction)?;
        }

        Ok(())
    }

    fn eval_definition(&mut self, definition_key: &str) -> Result {
        let definitions = self.definitions.clone();

        let instructions = definitions
            .get(definition_key)
            .unwrap()
            .split_whitespace()
            .collect::<Vec<_>>();

        for instruction in instructions.into_iter() {
            let definition = self
                .definitions
                .get(instruction.to_lowercase().as_str())
                .or(Some(&instruction.to_string()))
                .cloned();

            let nested_definition = self
                .definitions
                .get(definition.clone().unwrap().as_str())
                .cloned();

            if let Some(nested_definition) = nested_definition {
                let current_definition = nested_definition.clone();
                self.eval_definition(&current_definition)?;
                continue;
            }

            self.eval_basic(definition.unwrap().as_str())?;
        }

        Ok(())
    }

    fn eval_basic(&mut self, instruction: &str) -> Result {
        match instruction.to_lowercase().as_str() {
            "+" => self.add(),
            "-" => self.sub(),
            "*" => self.mul(),
            "/" => self.div(),
            "dup" => self.dup(),
            "drop" => self.drop(),
            "swap" => self.swap(),
            "over" => self.over(),
            _ => self.push(instruction),
        }
    }

    fn push(&mut self, value: &str) -> Result {
        if let Ok(num) = value.parse::<Value>() {
            self.stack.push(num);
            Ok(())
        } else {
            Err(Error::UnknownWord)
        }
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
                return Err(Error::DivisionByZero);
            }
            self.stack.push(b / a);
            Ok(())
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
