use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};
static RECURSIVE_CALLS: AtomicUsize = AtomicUsize::new(0);

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let (left, right, letters) = parse_input(input);
    println!("input: {}", input);
    println!("left: {:?}", left);
    println!("right: {:?}", right);
    println!("letters: {:?}", letters);

    // Check the right side can accomidate the left side vars, else fail early
    if invalid_width(&left, &right) {
        return None;
    }

    let digits: Vec<u8> = (0..=9).rev().collect();
    let mut char_to_digit = HashMap::new();

    RECURSIVE_CALLS.store(0, Ordering::SeqCst); // Reset the recursive calls counter
    solve_arithmetic(&left, &right, &letters, &digits, &mut char_to_digit)
}

fn solve_arithmetic(
    left: &Vec<String>,
    right: &Vec<String>,
    letters: &Vec<char>,
    digits: &Vec<u8>,
    char_to_digit: &mut HashMap<char, u8>,
) -> Option<HashMap<char, u8>> {
    RECURSIVE_CALLS.fetch_add(1, Ordering::SeqCst); // Increment the recursive calls counter

    // Base case: no letters left to assign, check if equation is valid
    if letters.is_empty() {
        // println!("check this {:?}", char_to_digit);
        if check_equation(left, right, char_to_digit) {
            println!(
                "Solution found! Recursive Calls: {}",
                RECURSIVE_CALLS.load(Ordering::Relaxed)
            );
            return Some(char_to_digit.clone());
        } else {
            return None;
        }
    }

    // Recursive case: try assigning each digit to the first letter, and recurse
    let current_char = letters[0];

    for digit in digits {
        // Check if the digit is already assigned to a letter
        if char_to_digit.values().any(|&v| v == *digit) {
            continue;
        }
        char_to_digit.insert(current_char, *digit);

        // Remove the assigned letter from the vector
        let remaining_letters = &letters[1..].to_vec();

        // Recursive call
        if let Some(solution) =
            solve_arithmetic(left, right, remaining_letters, digits, char_to_digit)
        {
            return Some(solution);
        }

        // Backtrack: Remove the assigned letter and digit
        char_to_digit.remove(&current_char);
    }

    None
}

fn check_equation(left: &Vec<String>, right: &Vec<String>, solution: &HashMap<char, u8>) -> bool {
    let mut left_sum: u64 = 0;
    let mut right_sum: u64 = 0;

    for word in left {
        // Check if the first character of the word has a digit value of 0
        if let Some(&digit) = solution.get(&word.chars().next().unwrap()) {
            if digit == 0 {
                return false;
            }
        }

        let mut word_sum: u64 = 0;
        for (i, c) in word.chars().enumerate() {
            let power = (word.len() - i - 1) as u32;
            let num = solution.get(&c).unwrap();
            word_sum += (*num as u64) * 10u64.pow(power);
        }
        left_sum += word_sum;
    }

    for word in right {
        // Check if the first character of the word has a digit value of 0
        if let Some(&digit) = solution.get(&word.chars().next().unwrap()) {
            if digit == 0 {
                return false;
            }
        }
        let mut word_sum: u64 = 0;
        for (i, c) in word.chars().enumerate() {
            let power = (word.len() - i - 1) as u32;
            let num = solution.get(&c).unwrap();
            word_sum += (*num as u64) * 10u64.pow(power);
        }
        right_sum += word_sum;
    }
    // println!("left_sum: {}", left_sum);
    // println!("right_sum: {}", right_sum);

    left_sum == right_sum
}

fn parse_input(input: &str) -> (Vec<String>, Vec<String>, Vec<char>) {
    let mut left = Vec::new();
    let mut right = Vec::new();
    let mut letters = Vec::new();

    let mut left_side = true;
    let mut current_word = String::new();

    for c in input.chars() {
        match c {
            '+' => {
                if !current_word.is_empty() {
                    left.push(current_word);
                    current_word = String::new();
                }
            }
            '=' => {
                left_side = false;
                if !current_word.is_empty() {
                    left.push(current_word);
                    current_word = String::new();
                }
            }
            ' ' => {
                if !current_word.is_empty() {
                    if left_side {
                        left.push(current_word);
                    } else {
                        right.push(current_word);
                    }
                    current_word = String::new();
                }
            }
            _ => {
                // Check if it is a char before adding to current_word
                if !c.is_alphabetic() {
                    continue;
                }

                current_word.push(c);
                if !letters.contains(&c) {
                    letters.push(c);
                }
            }
        }
    }

    if !current_word.is_empty() {
        right.push(current_word);
    }

    (left, right, letters)
}

fn invalid_width(left: &Vec<String>, right: &Vec<String>) -> bool {
    let left_len = left.iter().map(|s| s.len()).max().unwrap();
    let right_len = right.iter().map(|s| s.len()).max().unwrap();
    if left_len > right_len {
        return true;
    }
    false
}
