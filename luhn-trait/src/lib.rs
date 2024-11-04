// Impl valid_lugn for all types that implement ToString
// nstead of defining a full struct and implementing toString from T for it like in luhn-from
pub trait Luhn: ToString {
    fn valid_luhn(&self) -> bool {
        let mut sum = 0;
        let mut len = 0;
        for c in self.to_string().chars().rev() {
            if c.is_whitespace() {
                continue;
            }
            match c.to_digit(10) {
                Some(d) => {
                    if len % 2 == 1 {
                        sum += d * 2 - if d > 4 { 9 } else { 0 };
                    } else {
                        sum += d;
                    }
                    len += 1;
                }
                None => return false,
            }
        }
        len > 1 && sum % 10 == 0
    }
}

impl<T: ToString> Luhn for T {}
