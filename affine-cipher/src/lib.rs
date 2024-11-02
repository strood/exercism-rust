/// While the problem description indicates a return status of 1 should be returned on errors,
/// it is much more common to return a `Result`, so we provide an error type for the result here.
#[derive(Debug, Eq, PartialEq)]
pub enum AffineCipherError {
    NotCoprime(i32),
}

/// Encodes the plaintext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn encode(plaintext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    is_coprime(a, 26)?;

    let encoded: String = plaintext.chars().filter(|c| { c.is_alphabetic() || c.is_numeric() }).map(|c| {
        if c.is_alphabetic() {
            let x = c.to_ascii_lowercase() as i32 - 'a' as i32;
            let y = (a * x + b) % 26;
            ('a' as i32 + y) as u8 as char
        } else {
            c
        }
    }).collect::<Vec<char>>().chunks(5).map(|chunk| chunk.iter().collect::<String>()).collect::<Vec<_>>().join(" ");
    Ok(encoded)


}

/// Decodes the ciphertext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn decode(ciphertext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    is_coprime(a, 26)?;
    let a_inv = mod_inverse(a, 26);
    let decoded = ciphertext.chars().filter(|c| { c.is_alphabetic() || c.is_numeric() }).map(|c| {
        if c.is_alphabetic() {
            let y = c.to_ascii_lowercase() as i32 - 'a' as i32;
            let mut x = (a_inv * (y - b)) % 26;
            if x < 0 {x += 26;}
            ('a' as i32 + x) as u8 as char
        } else {
            c
        }
    }).collect::<String>();

    Ok(decoded)
}

pub fn mod_inverse(a: i32, m: i32) -> i32 {
    for i in 1..m {
        if (a * i) % m == 1 {
            return i;
        }
    }
    1
}

pub fn is_coprime(a: i32, m: i32) -> Result<bool, AffineCipherError> {
    match common_elements(&factors(a), &factors(m)).len() == 1 {
        true => Ok(true),
        false => Err(AffineCipherError::NotCoprime(a))
    }
}

pub fn common_elements<T: Eq + Clone>(a: &[T], b: &[T]) -> Vec<T> {
    a.iter().fold(Vec::new(), |mut acc, x| {
        if b.contains(x) {
            acc.push((*x).clone());
        }
        acc
    })
}

pub fn factors(n: i32) -> Vec<i32> {
    (1..=n).filter(|x| n % x == 0).collect()
}
