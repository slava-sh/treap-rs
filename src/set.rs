extern crate rand;

use map::TreapMap;

#[derive(Debug, Clone)]
pub struct TreapSet<T, Rng> {
    map: TreapMap<T, (), Rng>,
}

impl<T> TreapSet<T, rand::XorShiftRng> {
    pub fn new() -> Self {
        TreapSet { map: TreapMap::new() }
    }
}

impl<T> Default for TreapSet<T, rand::XorShiftRng> {
    fn default() -> Self {
        TreapSet::new()
    }
}

impl<T, Rng> TreapSet<T, Rng>
where
    T: Ord,
    Rng: rand::Rng,
{
    pub fn len(&self) -> usize {
        self.map.len()
    }

    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    pub fn clear(&mut self) {
        self.map.clear();
    }

    pub fn insert(&mut self, value: T) -> bool {
        self.map.insert(value, ()).is_none()
    }

    pub fn remove(&mut self, value: &T) -> bool {
        self.map.remove(value).is_some()
    }

    pub fn contains(&self, value: &T) -> bool {
        self.map.get(value).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let mut t = TreapSet::new();
        assert!(t.is_empty());
        assert_eq!(t.insert("a"), true);
        assert!(!t.is_empty());
        assert_eq!(t.len(), 1);
        assert_eq!(t.insert("a"), false);
        assert_eq!(t.len(), 1);
        assert_eq!(t.insert("b"), true);
        assert_eq!(t.len(), 2);
        assert_eq!(t.remove(&"a"), true);
        assert_eq!(t.len(), 1);
        assert_eq!(t.contains(&"a"), false);
        assert_eq!(t.remove(&"a"), false);
        assert_eq!(t.contains(&"b"), true);
        t.clear();
        assert!(t.is_empty());
    }
}
