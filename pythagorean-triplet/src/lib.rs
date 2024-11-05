use std::collections::HashSet;

// Euclids algo to find all pythagorean triplets for a given sum
pub fn find(sum: u32) -> HashSet<[u32; 3]> {
    let mut triplets = HashSet::new();
    for m in 2..=sum/3 {
        for n in 1..m {
            // Find a triangle that works, then check all multiples of it
            if (m + n) % 2 == 1 && gcd(m, n) == 1 {
                let a = (m * m) - (n * n);
                let b = 2 * m * n;
                let c = (m * m) + (n * n);

                let mut triplet = [a, b, c];
                let triplet_sum = triplet.iter().sum::<u32>();
                if triplet_sum > sum {
                    break;
                }
                if sum == triplet_sum {
                    triplet.sort();
                    triplets.insert(triplet);
                }

                // Check multiples
                let mut k = 2;
                while k * c <= sum {
                    let mut triplet_mul = [k * a, k * b, k * c];
                    if triplet_mul.iter().sum::<u32>() == sum {
                        triplet_mul.sort();
                        triplets.insert(triplet_mul);
                    }
                    k += 1;
                }
            }
        }
    }
    triplets
}

fn gcd(mut a: u32, mut b: u32) -> u32 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}