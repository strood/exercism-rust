use rand::{thread_rng, Rng};

pub fn private_key(p: u64) -> u64 {
    // Generate a private key that is a random number in the range [2, p)
    let mut rng = thread_rng();
    let private_key = rng.gen_range(2..p);
    private_key
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    // unimplemented!("Calculate public key using prime numbers {p} and {g}, and private key {a}")
    // Calculate public key using prime numbers {p} and {g}, and private key {a}
    // A loop that calculates g^a mod p memory eficciently
    modular_exponentiation(g, a, p)
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    // unimplemented!("Calculate secret key using prime number {p}, public key {b_pub}, and private key {a}")
    // Calculate secret key using prime number {p}, public key {b_pub}, and private key {a}
    // A loop that calculates b_pub^a mod p memory eficciently
    modular_exponentiation(b_pub, a, p)
}

fn modular_exponentiation(base: u64, exponent: u64, modulus: u64) -> u64 {
    if modulus == 1 {
        return 0;
    }

    let mut result = 1;
    let mut base = base % modulus;
    let mut exponent = exponent;
    while exponent > 0 {
        println!(
            "result: {}, base: {}, exponent: {} modules: {}",
            result, base, exponent, modulus
        );
        if exponent % 2 == 1 {
            println!("IN here");
            result = (result * base) % modulus;
        }
        // base = (base * base) % modulus; // This overflows with big primes, so we need to break it down to avoid this
        base = modular_multiply(base, base, modulus);
        exponent /= 2;
    }

    result
}

fn modular_multiply(a: u64, b: u64, modulus: u64) -> u64 {
    let mut result = 0;
    let mut a = a % modulus;
    let mut b = b % modulus;

    while b > 0 {
        if b % 2 == 1 {
            result = (result + a) % modulus;
        }
        a = (a << 1) % modulus;
        b /= 2;
    }

    result
}
