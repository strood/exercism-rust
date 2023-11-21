// Binary search for any Ord type
pub fn find<T, U>(array: T, key: U) -> Option<usize>
where
    T: AsRef<[U]>,
    U: core::cmp::Ord,
{
    let array = array.as_ref();
    let len = array.len();

    match len {
        0 => return None,
        1 => match array[0].cmp(&key) {
            core::cmp::Ordering::Equal => return Some(0),
            _ => return None,
        },
        _ => {}
    }

    let mid = len / 2;
    let mid_val = array.get(mid)?;

    match key.cmp(mid_val) {
        core::cmp::Ordering::Equal => return Some(mid),
        core::cmp::Ordering::Less => {
            let (left_vals, _) = array.split_at(mid);
            return find(left_vals, key);
        }
        core::cmp::Ordering::Greater => {
            let (_, right_vals) = array.split_at(mid);
            if let Some(result) = find(right_vals, key) {
                return Some(mid + result);
            }
        }
    }

    None
}
