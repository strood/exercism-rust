pub fn egg_count(display_value: u32) -> usize {
    format!("{:b}", display_value)
        .chars()
        .fold(0, |acc, x| if x == '1' { acc + 1 } else { acc })
}
