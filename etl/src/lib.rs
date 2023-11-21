use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    // For each key  in the input map, we want to turn each
    // of the values into a key in the output map, with the
    // value being the key from the input map. Also turning the key into
    // its lowercase form
    let mut output = BTreeMap::new();
    for (key, value) in h {
        for c in value {
            output.insert(c.to_lowercase().next().unwrap(), *key);
        }
    }
    output
}
