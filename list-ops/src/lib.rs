/// Yields each item of a and then each item of b
pub fn append<I, J>(a: I, b: J) -> impl Iterator<Item = I::Item>
where
    I: Iterator,
    J: Iterator<Item = I::Item>,
{
    let mut res = vec![];
    for item in a {
        res.push(item);
    }
    for item in b {
        res.push(item);
    }
    res.into_iter()
}

/// Combines all items in all nested iterators inside into one flattened iterator
pub fn concat<I>(nested_iter: I) -> impl Iterator<Item = <I::Item as Iterator>::Item>
where
    I: Iterator,
    I::Item: Iterator,
{
    let mut res = vec![];
    for item in nested_iter {
        for inner_item in item {
            res.push(inner_item);
        }
    }
    res.into_iter()
}

/// Returns an iterator of all items in iter for which `predicate(item)` is true
pub fn filter<I, F>(iter: I, predicate: F) -> impl Iterator<Item = I::Item>
where
    I: Iterator,
    F: Fn(&I::Item) -> bool,
{
    let mut res = vec![];
    for item in iter {
        if predicate(&item) {
            res.push(item);
        }
    }
    res.into_iter()
}

pub fn length<I: Iterator>(iter: I) -> usize {
    let mut count = 0;
    for _ in iter {
        count += 1;
    }
    count
}

/// Returns an iterator of the results of applying `function(item)` on all iter items
pub fn map<I, F, U>(iter: I, function: F) -> impl Iterator<Item = U>
where
    I: Iterator,
    F: Fn(I::Item) -> U,
{
   let mut res = vec![];
    for item in iter {
          res.push(function(item));
     }
     res.into_iter()
}

pub fn foldl<I, F, U>(iter: I, initial: U, function: F) -> U
where
    I: Iterator,
    F: Fn(U, I::Item) -> U,
{
    let mut acc = initial;
    for item in iter {
        acc = function(acc, item);
    }
    acc
}

pub fn foldr<I, F, U>(iter: I, initial: U, function: F) -> U
where
    I: DoubleEndedIterator,
    F: Fn(U, I::Item) -> U,
{
    foldl(reverse(iter), initial, function)
}

/// Returns an iterator with all the original items, but in reverse order
pub fn reverse<I: DoubleEndedIterator>(iter: I) -> impl Iterator<Item = I::Item> {
    let mut res = vec![];
    for item in iter {
        res.insert(0, item);
    }
    res.into_iter()
}
