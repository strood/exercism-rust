pub fn collatz(n: u64) -> Option<u64> {
    // Return the number of steps required to reach 1
    if n == 0 {
        return None;
    }

    let mut steps: u64 = 0;
    let mut num: u64 = n; // Use u128 for larger range, test want None answers though so we wont
                          // Instead use checked mul, div and add so we return None if we overflow our u64

    while num != 1 {
        if num % 2 == 0 {
            num = num.checked_div(2)?;
        } else {
            num = num.checked_mul(3)?.checked_add(1)?;
        }

        steps = steps.checked_add(1)?;
    }

    Some(steps)
}
