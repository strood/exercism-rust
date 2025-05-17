use std::collections::HashMap;

const PRICES: &[u32] = &[800, 1520, 2160, 2560, 3000];

pub fn lowest_price(books: &[u32]) -> u32 {
    if books.is_empty() {
        return 0;
    }

    // Get frequency of each book & sort it in descending order
    let mut piles: Vec<u32> = books.iter().fold(HashMap::<u32, u32>::new(), |mut h, i| {
        *h.entry(*i).or_insert(0) += 1;
        h        
    }).values().cloned().collect();
    println!("{:?}", piles);

    piles.sort_by(|a, b| b.cmp(a));
    println!("{:?}", piles);

    // Subtract the frequency of each book from the next book
    // leaving us with the number of unique books
    for i in 0..(piles.len() - 1) {
        piles[i] -= piles[i + 1];
    }
    println!("{:?}", piles);

    // Special case for 5 different books
    // reorganize the piles to get the best price
    if piles.len() >= 5 {
        let n = piles[2].min(piles[4]);
        piles[2] -= n;
        piles[4] -= n;
        piles[3] += 2 * n;
    }

    // Go through the piles and calculate the price of each based on the size in the pile
    piles.iter().zip(PRICES.iter()).fold(0, |sum, (n,p)| sum + n * p)
}