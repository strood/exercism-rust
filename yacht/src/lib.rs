pub enum Category {
    Ones,
    Twos,
    Threes,
    Fours,
    Fives,
    Sixes,
    FullHouse,
    FourOfAKind,
    LittleStraight,
    BigStraight,
    Choice,
    Yacht,
}

type Dice = [u8; 5];
pub fn score(_dice: Dice, _category: Category) -> u8 {
    match _category {
        Category::Ones => get_counts(1, _dice),
        Category::Twos => get_counts(2, _dice) * 2,
        Category::Threes => get_counts(3, _dice) * 3,
        Category::Fours => get_counts(4, _dice) * 4,
        Category::Fives => get_counts(5, _dice) * 5,
        Category::Sixes => get_counts(6, _dice) * 6,
        Category::FullHouse => check_contains(3, _dice)
            .and_then(|_contains| check_contains(2, _dice).map(|_| _dice.iter().sum()))
            .unwrap_or(0),
        Category::FourOfAKind => check_contains_at_least(4, _dice)
            .map(|contains| contains)
            .unwrap_or(0),
        Category::LittleStraight => check_straight(1, _dice),
        Category::BigStraight => check_straight(2, _dice),
        Category::Choice => _dice.iter().sum(),
        Category::Yacht => _dice.iter().all(|&x| x == _dice[0]) as u8 * 50,
    }
}

fn check_straight(_start_index: u8, _dice: Dice) -> u8 {
    // create object to hold counts, index is die value - 1
    let mut counts: [u8; 6] = [0; 6];
    // iterate over dice
    for die in _dice.iter() {
        // increment count for each die value
        counts[*die as usize - 1] += 1;
    }

    if counts[_start_index as usize - 1..=_start_index as usize + 3]
        .iter()
        .all(|&x| x == 1)
    {
        30
    } else {
        0
    }
}

fn check_contains_at_least(_num: u8, _dice: Dice) -> Option<u8> {
    // create object to hold counts, index is die value - 1
    let mut counts: [u8; 6] = [0; 6];
    // iterate over dice
    for die in _dice.iter() {
        // increment count for each die value
        counts[*die as usize - 1] += 1;
    }
    // return count for _num if at least _num
    if counts.iter().any(|&x| x >= _num) {
        // Check which index contains _num or greater, return index * index
        let pos = counts.iter().position(|&x| x >= _num).unwrap();
        Some((pos + 1) as u8 * _num)
    } else {
        None
    }
}

fn check_contains(_num: u8, _dice: Dice) -> Option<u8> {
    // create object to hold counts, index is die value - 1
    let mut counts: [u8; 6] = [0; 6];
    // iterate over dice
    for die in _dice.iter() {
        // increment count for each die value
        counts[*die as usize - 1] += 1;
    }
    // return count for _num
    if counts.contains(&_num) {
        // Check which index contains _num, return index * index
        let pos = counts.iter().position(|&x| x == _num).unwrap();
        Some((pos + 1) as u8 * _num)
    } else {
        None
    }
}

fn get_counts(_num: u8, _dice: Dice) -> u8 {
    // create object to hold counts, index is die value - 1
    let mut counts: [u8; 6] = [0; 6];
    // iterate over dice
    for die in _dice.iter() {
        // increment count for each die value
        counts[*die as usize - 1] += 1;
    }
    // return count for _num
    counts[_num as usize - 1]
}
