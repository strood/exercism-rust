#[derive(Debug, PartialEq, Eq)]
pub struct CustomSet<T> {
    items: Vec<T>
}

impl<T: Clone + PartialEq + Ord> CustomSet<T> {
    pub fn new(input: &[T]) -> Self {
        let mut set = CustomSet {
            items: Vec::new()
        };
        for item in input {
            set.add(item.clone());
        }
        set.sort();
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
        let mut set = CustomSet {
            items: Vec::new()
        };
        for item in self.items.iter() {
            if _other.contains(item) {
                set.add(item.clone());
            }
        }
        set
    }

    #[must_use]
    pub fn difference(&self, _other: &Self) -> Self {
        let mut set = CustomSet {
            items: Vec::new()
        };
        for item in self.items.iter() {
            if !_other.contains(item) {
                set.add(item.clone());
            }
        }
        set
    }

    #[must_use]
    pub fn union(&self, _other: &Self) -> Self {
        let mut set = CustomSet {
            items: Vec::new()
        };
        for item in self.items.iter() {
            set.add(item.clone());
        }
        for item in _other.items.iter() {
            set.add(item.clone());
        }
        set
    }
}
