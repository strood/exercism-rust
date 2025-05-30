pub fn square_of_sum(n: u32) -> u32 {
    return (1..n + 1).fold(0, |acc, x| acc + x).pow(2);
}

pub fn sum_of_squares(n: u32) -> u32 {
    return (1..n + 1).fold(0, |acc, x| acc + x.pow(2));
}

pub fn difference(n: u32) -> u32 {
    return square_of_sum(n) - sum_of_squares(n);
}
