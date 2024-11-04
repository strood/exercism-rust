#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    SpanTooLong,
    InvalidDigit(char),
}

pub fn lsp(string_digits: &str, span: usize) -> Result<u64, Error> {
    if span > string_digits.len() {
        return Err(Error::SpanTooLong);
    }
    
    let digits: Vec<u64> = string_digits.chars().map(|c| {
        c.to_digit(10).map(|d| d as u64).ok_or(Error::InvalidDigit(c))
    }).collect::<Result<Vec<u64>, Error>>()?;

    Ok(digits.windows(span).map(|w| w.iter().product()).max().unwrap())
}
