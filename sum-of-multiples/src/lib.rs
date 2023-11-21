use std::collections::HashSet;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    // Sum all the unique multiples of particular numbers up to but not including that number.
    let mut multiples: Vec<u32> = Vec::new();

    for factor in factors {
        let mut i: u32 = 1;

        if *factor == 0 {
            continue;
        }

        while i * factor < limit {
            multiples.push(i * factor);
            i += 1;
        }
    }

    let unique_multiples = remove_duplicates(&multiples);

    unique_multiples.iter().sum()
}

fn remove_duplicates(numbers: &[u32]) -> Vec<u32> {
    let unique_numbers: HashSet<u32> = numbers.iter().cloned().collect();
    unique_numbers.into_iter().collect()
}
