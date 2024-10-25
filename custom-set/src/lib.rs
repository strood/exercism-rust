#[derive(Debug, PartialEq, Eq)]
pub struct CustomSet<T> {
    items: Vec<T>
}

impl<T: Clone + PartialEq + Ord> CustomSet<T> {
    pub fn new(input: &[T]) -> Self {
        let mut set = CustomSet {
            items: Vec::new()
        };
        input.iter().for_each(|item| set.add(item.clone()));
        set
    }

    fn sort(&mut self) {
        self.items.sort();
    }

    pub fn contains(&self, element: &T) -> bool {
        self.items.contains(element)
    }

    pub fn add(&mut self, _element: T) {
        if self.contains(&_element) { return; }
        self.items.push(_element);
        self.sort();
    }

    pub fn is_subset(&self, other: &Self) -> bool {
        self.items.iter().all(|item| other.contains(item))
    }

    pub fn is_empty(&self) -> bool {
        self.items.len() == 0
    }

    pub fn is_disjoint(&self, other: &Self) -> bool {
        self.items.iter().all(|item| !other.contains(item))
    }

    #[must_use]
    pub fn intersection(&self, _other: &Self) -> Self {
        CustomSet {
            items: self.items.iter().fold(Vec::new(), |mut acc, item| {
                if _other.contains(item) {
                    acc.push(item.clone());
                }
                acc
            })
        }
    }

    #[must_use]
    pub fn difference(&self, _other: &Self) -> Self {
        CustomSet {
            items: self.items.iter().fold(Vec::new(), |mut acc, item| {
                if !_other.contains(item) {
                    acc.push(item.clone());
                }
                acc
            })
        }
    }

    #[must_use]
    pub fn union(&self, other: &Self) -> Self {
        let mut set = CustomSet {
            items: Vec::new()
        };
        self.items.iter().for_each(|item| set.add(item.clone()));
        other.items.iter().for_each(|item| set.add(item.clone()));
        set
    }
}
