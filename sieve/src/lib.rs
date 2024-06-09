pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let num_u64s = (upper_bound + 1 + 63) / 64;
    let mut bit_vector = vec![0xffffffffffffffffu64; num_u64s as usize];

    // Set 0 and 1 as non primes
    bit_vector[0] &= !0b11;

    let mut p = 2;
    while p * p <= upper_bound {
        if bit_vector[(p / 64) as usize] & (1 << (p % 64)) != 0 {
            println!("p: {}", p);
            let mut multiple = p * p;
            while multiple <= upper_bound {
                println!("multiple: {}", multiple);
                bit_vector[(multiple / 64) as usize] &= !(1 << (multiple % 64)); // mark multiple
                multiple += p;
            }
        }
        p += 1;
    }

    // Collect primes
    let mut primes = Vec::new();
    for num in 2..=upper_bound {
        if bit_vector[(num / 64) as usize] & (1 << (num % 64)) != 0 {
            primes.push(num);
        }
    }

    primes
}
