pub struct Item {
    pub weight: u32,
    pub value: u32,
}

pub fn maximum_value(_max_weight: u32, _items: &[Item]) -> u32 {
    // Given max_weight and items, find the maximum value that can be generated
    // combining items without going over the max_weight limit of our knapsack.
    // Trying the knapsack table approach

    let num_items = _items.len();
    let mut table = vec![vec![0; (_max_weight + 1) as usize]; (num_items + 1) as usize];

    for i in 1..=num_items {
        let item_weight = _items[i - 1].weight;
        let item_value = _items[i - 1].value;

        for current_weight in 1..=_max_weight {
            if item_weight <= current_weight {
                let include_item_value =
                    item_value + table[i - 1][current_weight as usize - item_weight as usize];
                let exclude_item_value = table[i - 1][current_weight as usize];
                table[i][current_weight as usize] = include_item_value.max(exclude_item_value);
            } else {
                table[i][current_weight as usize] = table[i - 1][current_weight as usize];
            }
        }
    }

    table[num_items][_max_weight as usize]
}
