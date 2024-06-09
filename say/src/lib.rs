pub fn encode(n: u64) -> String {
    match n {
        0 => String::from("zero"),
        1 => String::from("one"),
        2 => String::from("two"),
        3 => String::from("three"),
        4 => String::from("four"),
        5 => String::from("five"),
        6 => String::from("six"),
        7 => String::from("seven"),
        8 => String::from("eight"),
        9 => String::from("nine"),
        10 => String::from("ten"),
        11 => String::from("eleven"),
        12 => String::from("twelve"),
        13 => String::from("thirteen"),
        14 => String::from("fourteen"),
        15 => String::from("fifteen"),
        16 => String::from("sixteen"),
        17 => String::from("seventeen"),
        18 => String::from("eighteen"),
        19 => String::from("nineteen"),
        20..=99 => {
            let tens = n / 10;
            let ones = n % 10;
            let tens_str = match tens {
                2 => String::from("twenty"),
                3 => String::from("thirty"),
                4 => String::from("forty"),
                5 => String::from("fifty"),
                6 => String::from("sixty"),
                7 => String::from("seventy"),
                8 => String::from("eighty"),
                9 => String::from("ninety"),
                _ => String::from("unknown"),
            };
            let ones_str = match ones {
                0 => String::new(),
                1 => String::from("one"),
                2 => String::from("two"),
                3 => String::from("three"),
                4 => String::from("four"),
                5 => String::from("five"),
                6 => String::from("six"),
                7 => String::from("seven"),
                8 => String::from("eight"),
                9 => String::from("nine"),
                _ => String::from("unknown"),
            };
            if ones == 0 {
                tens_str
            } else {
                format!("{}-{}", tens_str, ones_str)
            }
        }
        100..=999 => {
            let hundreds = n / 100;
            let remainder = n % 100;
            let hundreds_str = encode(hundreds);
            let remainder_str = encode(remainder);
            if remainder == 0 {
                format!("{} hundred", hundreds_str)
            } else {
                format!("{} hundred {}", hundreds_str, remainder_str)
            }
        }
        1000..=999_999 => {
            let thousands = n / 1000;
            let remainder = n % 1000;
            let thousands_str = encode(thousands);
            let remainder_str = encode(remainder);
            if remainder == 0 {
                format!("{} thousand", thousands_str)
            } else {
                format!("{} thousand {}", thousands_str, remainder_str)
            }
        }
        1_000_000..=999_999_999 => {
            let millions = n / 1_000_000;
            let remainder = n % 1_000_000;
            let millions_str = encode(millions);
            let remainder_str = encode(remainder);
            if remainder == 0 {
                format!("{} million", millions_str)
            } else {
                format!("{} million {}", millions_str, remainder_str)
            }
        }
        1_000_000_000..=999_999_999_999 => {
            let billions = n / 1_000_000_000;
            let remainder = n % 1_000_000_000;
            let billions_str = encode(billions);
            let remainder_str = encode(remainder);
            if remainder == 0 {
                format!("{} billion", billions_str)
            } else {
                format!("{} billion {}", billions_str, remainder_str)
            }
        }
        1_000_000_000_000..=999_999_999_999_999 => {
            let trillions = n / 1_000_000_000_000;
            let remainder = n % 1_000_000_000_000;
            let trillions_str = encode(trillions);
            let remainder_str = encode(remainder);
            if remainder == 0 {
                format!("{} trillion", trillions_str)
            } else {
                format!("{} trillion {}", trillions_str, remainder_str)
            }
        }
        1_000_000_000_000_000..=999_999_999_999_999_999 => {
            let quadrillions = n / 1_000_000_000_000_000;
            let remainder = n % 1_000_000_000_000_000;
            let quadrillions_str = encode(quadrillions);
            let remainder_str = encode(remainder);
            if remainder == 0 {
                format!("{} quadrillion", quadrillions_str)
            } else {
                format!("{} quadrillion {}", quadrillions_str, remainder_str)
            }
        }
        1_000_000_000_000_000_000..=u64::MAX => {
            let quintillions = n / 1_000_000_000_000_000_000;
            let remainder = n % 1_000_000_000_000_000_000;
            let quintillions_str = encode(quintillions);
            let remainder_str = encode(remainder);
            if remainder == 0 {
                format!("{} quintillion", quintillions_str)
            } else {
                format!("{} quintillion {}", quintillions_str, remainder_str)
            }
        }
    }
}
