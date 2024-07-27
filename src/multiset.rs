use indexmap::{map::Entry, IndexMap};
use std::{fmt, hash::Hash};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MultiSet<T: fmt::Debug + Clone + Copy + PartialEq + Eq + PartialOrd + Ord + Hash> {
    pub(crate) bag: IndexMap<T, usize>,
    pub(crate) sum: usize,
}

impl<T: fmt::Debug + Clone + Copy + PartialEq + Eq + PartialOrd + Ord + Hash> MultiSet<T> {
    pub fn with_capacity(n: usize) -> Self {
        Self {
            bag: IndexMap::with_capacity(n),
            sum: 0,
        }
    }
    pub fn new() -> Self {
        Self::with_capacity(0)
    }
    pub fn is_empty(&self) -> bool {
        self.sum == 0
    }
    pub fn clear(&mut self) {
        self.sum = 0;
        self.bag.clear();
    }

    pub fn pop(&mut self, item: T) -> Option<T> {
        match self.bag.entry(item) {
            Entry::Occupied(mut v) => {
                if *v.get() > 0 {
                    self.sum -= 1;
                    *v.get_mut() -= 1;
                    Some(item)
                } else {
                    None
                }
            }
            Entry::Vacant(_) => None,
        }
    }
    pub fn pop_multiple(&mut self, item: T, n: usize) -> Option<(T, usize)> {
        match self.bag.entry(item) {
            Entry::Occupied(mut v) => match *v.get() {
                0 => None,
                u if u >= n => {
                    self.sum -= n;
                    *v.get_mut() -= n;
                    Some((item, n))
                }
                u => {
                    self.sum -= u;
                    *v.get_mut() = 0;
                    Some((item, n))
                }
            },
            Entry::Vacant(_) => None,
        }
    }
    pub fn remove_multiple(&mut self, item: T, n: usize) {
        self.pop_multiple(item, n);
    }
    pub fn remove(&mut self, item: T) {
        self.pop(item);
    }
    pub fn push_multiple(&mut self, item: T, count: usize) {
        self.sum += count;
        match self.bag.entry(item) {
            Entry::Occupied(mut v) => {
                *v.get_mut() += count;
            }
            Entry::Vacant(e) => {
                e.insert(count);
            }
        }
    }
    pub fn push(&mut self, item: T) {
        self.push_multiple(item, 1)
    }
    pub fn n_available(&self, item: &T) -> usize {
        self.bag.get(item).map(Clone::clone).unwrap_or(0)
    }
    pub fn n_total(&self) -> usize {
        self.sum
    }
}
impl<T: fmt::Debug + Clone + Copy + PartialEq + Eq + PartialOrd + Ord + Hash>
    FromIterator<(T, usize)> for MultiSet<T>
{
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = (T, usize)>,
    {
        let mut inv = Self::new();
        for (item, count) in iter {
            inv.push_multiple(item, count);
        }
        inv
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example() -> MultiSet<char> {
        [('a', 1), ('b', 2), ('c', 3)].into_iter().collect()
    }

    #[test]
    fn pop() {
        let mut s = example();
        s.pop('b');
        s.pop('b');
        assert_eq!(s.n_total(), 4);
        assert_eq!(s.n_available(&'b'), 0);
    }

    #[test]
    fn push() {
        let mut s = example();
        s.push('a');
        assert_eq!(s.n_total(), 7);
        assert_eq!(s.n_available(&'a'), 2);
    }

    #[test]
    fn remove() {
        let mut s = example();
        s.remove('b');
        assert_eq!(s.n_total(), 5);
        assert_eq!(s.n_available(&'b'), 1);
    }

    #[test]
    fn pop_multiple() {
        let mut s = example();
        s.pop_multiple('b', 2);
        assert_eq!(s.n_total(), 4);
        assert_eq!(s.n_available(&'b'), 0);
    }

    #[test]
    fn push_multiple() {
        let mut s = example();
        s.push_multiple('a', 3);
        assert_eq!(s.n_total(), 9);
        assert_eq!(s.n_available(&'a'), 4);
    }

    #[test]
    fn remove_multiple() {
        let mut s = example();
        s.remove_multiple('b', 2);
        assert_eq!(s.n_total(), 4);
        assert_eq!(s.n_available(&'b'), 0);
    }

    #[test]
    fn clear() {
        let mut s = example();
        s.clear();
        assert_eq!(s.n_total(), 0);
        assert_eq!(s.n_available(&'b'), 0);
    }
}
