pub fn encode(n: u64) -> String {
    fn handle_encode(n: u64, div: u64, base: String) -> String {
        let remainder = n % div;
        let thousands = n / div;
        let remainder_str = encode(remainder);
        let thousands_str = encode(thousands);
        if remainder == 0 {
            format!("{} {}", thousands_str, base)
        } else {
            format!("{} {} {}", thousands_str, base, remainder_str)
        }
    }
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
        100..=999 => handle_encode(n, 100, String::from("hundred")),
        1000..=999_999 => handle_encode(n, 1000, String::from("thousand")),
        1_000_000..=999_999_999 => handle_encode(n, 1_000_000, String::from("million")),
        1_000_000_000..=999_999_999_999 => handle_encode(n, 1_000_000_000, String::from("billion")),
        1_000_000_000_000..=999_999_999_999_999 => {
            handle_encode(n, 1_000_000_000_000, String::from("trillion"))
        }
        1_000_000_000_000_000..=999_999_999_999_999_999 => {
            handle_encode(n, 1_000_000_000_000_000, String::from("quadrillion"))
        }
        1_000_000_000_000_000_000..=u64::MAX => {
            handle_encode(n, 1_000_000_000_000_000_000, String::from("quintillion"))
        }
    }
}
