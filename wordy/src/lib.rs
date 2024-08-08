#[derive(Debug)]
pub enum Token {
    Number(i32),
    Plus,
    Minus,
    Multiply,
    Divide,
    Exponential,
}

pub fn answer(command: &str) -> Option<i32> {
    // Tokenize our string into important words and numbers
    let tokens = tokenize(command);

    // If we have no tokens, return None
    tokens.as_ref()?;

    // Compute our tokens
    compute(tokens.unwrap())
}

fn compute(tokens: Vec<Token>) -> Option<i32> {
    let mut result = 0;
    let mut operator: Option<Token> = None;
    let mut operand: Option<i32> = None;

    // For each token, either add to operator or operand, or compute the result
    // if we alreadyhave whatever we are tryin to add, we have error
    for token in tokens {
        if let (Some(op), Some(opd)) = (&operator, &operand) {
            match token {
                Token::Number(current_num) => {
                    match op {
                        Token::Plus => result = opd + current_num,
                        Token::Minus => result = opd - current_num,
                        Token::Multiply => result = opd * current_num,
                        Token::Divide => result = opd / current_num,
                        Token::Exponential => result = opd.pow(current_num as u32),
                        _ => {}
                    }
                    operator = None;
                    operand = None;
                }
                _ => return None,
            }
        } else {
            match token {
                Token::Number(num) => {
                    if operand.is_none() {
                        operand = Some(num);
                    } else {
                        return None;
                    }
                }
                Token::Plus
                | Token::Minus
                | Token::Multiply
                | Token::Divide
                | Token::Exponential => {
                    if operator.is_none() {
                        operator = Some(token);
                    } else {
                        return None;
                    }
                }
            }
        }
    }

    if let (Some(op), Some(opd)) = (&operator, &operand) {
        if result == 0 {
            return None;
        }
        match op {
            Token::Plus => result += opd,
            Token::Minus => result -= opd,
            Token::Multiply => result *= opd,
            Token::Divide => result /= opd,
            Token::Exponential => result = result.pow(*opd as u32),
            _ => {}
        }
    }

    if operand.is_some() && operator.is_none() {
        if result == 0 {
            println!("this one here");
            result = operand.unwrap();
        } else {
            return None;
        }
    }

    Some(result)
}

fn tokenize(command: &str) -> Option<Vec<Token>> {
    let mut tokens = Vec::new();
    let mut words = command.split_whitespace();
    if words.next() != Some("What") || words.next() != Some("is") || words.clone().count() == 2 {
        return None;
    }

    let mut last_was_num = false;
    for word in words {
        match word {
            "plus" | "minus" | "multiplied" | "divided" | "raised" => {
                if last_was_num {
                    last_was_num = false;
                    match word {
                        "plus" => tokens.push(Token::Plus),
                        "minus" => tokens.push(Token::Minus),
                        "multiplied" => tokens.push(Token::Multiply),
                        "divided" => tokens.push(Token::Divide),
                        "raised" => tokens.push(Token::Exponential),
                        _ => {}
                    }
                } else {
                    return None;
                }
            }
            _ => {
                // if no numbers in the word, just skip
                if !word.chars().any(|c| c.is_ascii_digit()) {
                    continue;
                }

                if last_was_num {
                    return None;
                }

                if let Ok(num) = word.parse::<i32>() {
                    last_was_num = true;
                    tokens.push(Token::Number(num));
                } else if word.ends_with('?') {
                    if let Ok(num) = word.strip_suffix('?').unwrap().parse::<i32>() {
                        last_was_num = true;
                        tokens.push(Token::Number(num));
                    } else {
                        return None;
                    }
                } else if let Some(num) = word.chars().next().unwrap().to_digit(10) {
                    last_was_num = true;
                    tokens.push(Token::Number(num as i32));
                }
            }
        }
    }

    Some(tokens)
}
