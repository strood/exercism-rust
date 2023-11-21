pub fn factors(n: u64) -> Vec<u64> {
    // Get the prime factors of n
    let mut factors: Vec<u64> = Vec::new();
    let mut i: u64 = 2;
    let mut n = n;

    while n > 1 {
        if n % i == 0 {
            factors.push(i);
            n /= i;
        } else {
            i += 1;
        }
    }

    factors
}
