#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    IncompleteNumber,
}

/// Convert a list of numbers to a stream of bytes encoded with variable length encoding.
pub fn to_bytes(values: &[u32]) -> Vec<u8> {
    println!("{:?}", values);
    let bytes = values
        .iter()
        .flat_map(|&x| {
            // Get a byte vec for each input value
            let mut byte = vec![];
            let mut x = x;
            let mut i = 0;
            loop {
                // Get least sig 7 bits of x
                let mut b = (x & 0b01111111) as u8;

                // Shift left by 7 bits
                x >>= 7;
                // If we are past first iteration, set the most sig bit to indicate more follow
                // if first loop we skip as its the end marker
                if i > 0 {
                    b |= 0b10000000;
                }
                i += 1;

                // Insert the byte at the beginning of the vec
                byte.insert(0, b);

                // If no more left, break
                if x == 0 {
                    break;
                }
            }
            byte
        })
        .collect();
    bytes
}

/// Given a stream of bytes, extract all numbers which are encoded in there.
pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, Error> {
    println!("{:?}", bytes);
    let mut numbers = vec![];
    let mut current = 0;
    let mut i = 0;

    for &b in bytes {
        // get the least sig 7 bits
        let value = b & 0b01111111;
        // shift left by 7 bits
        current = (current << 7) | value as u32;

        // if the most sig bit is not set, we have reached the end of the number
        if b & 0b10000000 == 0 {
            // if we have more than 4 bytes, we have an overflow
            if i > 4 {
                return Err(Error::IncompleteNumber);
            }
            numbers.push(current);
            current = 0;
            i = 0;
        } else {
            i += 1;
        }
    }

    // if we have a number that is not terminated, we have an incomplete number
    if i > 0 {
        return Err(Error::IncompleteNumber);
    }

    Ok(numbers)
}
