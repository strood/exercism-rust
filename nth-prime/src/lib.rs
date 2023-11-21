pub fn nth(n: u32) -> u32 {
    // Get the nth prime of n
    let mut primes: Vec<u32> = Vec::new();
    let mut i: u32 = 2;

    while primes.len() <= n as usize {
        if is_prime(i) {
            primes.push(i);
        }
        i += 1;
    }

    primes[n as usize]
}

fn is_prime(n: u32) -> bool {
    // Check if n is prime
    if n == 2 {
        return true;
    }

    if n % 2 == 0 {
        return false;
    }

    let mut i: u32 = 3;

    while i < n {
        if n % i == 0 {
            return false;
        }
        i += 2;
    }

    true
}
