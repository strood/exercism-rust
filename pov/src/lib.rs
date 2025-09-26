use std::fmt::Debug;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Tree<T: Debug + Ord> {
    label: T,
    children: Vec<Tree<T>>,
}

impl<T: Debug + Ord> Tree<T> {
    pub fn new(label: T) -> Self {
        Tree {
            label,
            children: Default::default(),
        }
    }

    /// Builder-method for constructing a tree with children
    pub fn with_child(self, child: Self) -> Self {
        let mut children = self.children;
        children.push(child);
        children.sort_unstable();
        Tree {
            label: self.label,
            children,
        }
    }

    pub fn pov_from(&mut self, from: &T) -> bool {
        self.path_to(from).is_some()
    }

    fn path_to(&mut self, from: &T) -> Option<Vec<usize>> {
        if &self.label == from {
            return Some(Vec::new());
        }

        let (pos, mut index_list) = self
            .children
            .iter_mut()
            .enumerate()
            .find_map(|(i, child)| child.path_to(from).map(|index_list| (i, index_list)))?;

        let mut old_pov = self.children.remove(pos);
        std::mem::swap(self, &mut old_pov);

        let mut parent_of_old_pov = self;
        for i in index_list.iter().rev() {
            parent_of_old_pov = &mut parent_of_old_pov.children[*i];
        }

        let new_idx = parent_of_old_pov
            .children
            .binary_search_by(|c| c.label.cmp(&old_pov.label))
            .unwrap_err();
        parent_of_old_pov.children.insert(new_idx, old_pov);

        index_list.push(new_idx);
        Some(index_list)
    }

    pub fn path_between<'a>(&'a mut self, from: &'a T, to: &'a T) -> Option<Vec<&'a T>> {
        if !self.pov_from(to) {
            return None;
        }
        self.path_from(from, to)
    }

    fn path_from<'a>(&'a self, from: &'a T, to: &'a T) -> Option<Vec<&'a T>> {
        if &self.label == from {
            return Some(vec![from]);
        }
        let mut path = self.children.iter().find_map(|c| c.path_from(from, to))?;
        path.push(&self.label);
        Some(path)
    }
}
