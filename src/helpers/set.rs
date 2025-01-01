use std::{collections::HashSet, hash::Hash};

pub trait SetItem: PartialOrd + Ord + Clone + Hash {}
impl<T> SetItem for T where T: PartialOrd + Ord + Clone + Hash {}

#[derive(Debug, Clone)]
pub struct Set<T: SetItem> {
    set: HashSet<Vec<T>>
}

impl<T: SetItem> Set<T> {
    pub fn new() -> Set<T> {
        Set { set: HashSet::new() }
    }

    pub fn insert(&mut self, item: Vec<T>) -> bool {
        let mut cloned = item.clone();
        cloned.sort();
        self.set.insert(cloned)
    }

    pub fn remove(&mut self, item: Vec<T>) -> bool {
        let mut cloned = item.clone();
        cloned.sort();
        self.set.remove(&cloned)
    }

    pub fn contains(&self, item: Vec<&T>) -> bool {
        let mut cloned: Vec<T> = item.into_iter().cloned().collect();
        cloned.sort();
        self.set.contains(&cloned)
    }

    pub fn vec_snapshot(&self) -> Vec<Vec<T>> {
        self.set.iter().cloned().collect()
    }
}

impl<T: SetItem> IntoIterator for Set<T> {
    type Item = Vec<T>;
    type IntoIter = std::collections::hash_set::IntoIter<Vec<T>>;
    
    fn into_iter(self) -> Self::IntoIter {
        self.set.into_iter()
    }
}