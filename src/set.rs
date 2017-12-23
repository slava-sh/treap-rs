extern crate rand;

use set_stats::{SetStats, EmptyStats, SetStatsToMapStats};
use map::TreapMap;
use std::ops::Range;

#[derive(Debug, Clone)]
pub struct TreapSet<T, S = EmptyStats, Rng = rand::XorShiftRng> {
    map: TreapMap<T, (), SetStatsToMapStats<S>, Rng>,
}

impl<T> TreapSet<T> {
    pub fn new() -> Self {
        Default::default()
    }
}

impl<T, S> Default for TreapSet<T, S> {
    fn default() -> Self {
        TreapSet { map: Default::default() }
    }
}

impl<T, S, Rng> TreapSet<T, S, Rng>
where
    T: Ord,
    S: SetStats<T>,
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

    pub fn stats(&self, range: Range<&T>) -> Option<S> {
        self.map.stats(range).map(|map_stats| map_stats.set_stats)
    }

    pub fn stats_full(&self) -> Option<S> {
        self.map.stats_full().map(|map_stats| map_stats.set_stats)
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

    #[test]
    fn stats() {
        let mut t: TreapSet<_, Stats> = TreapSet::default();
        for i in 0..10 {
            assert!(t.insert(i));
        }
        assert_eq!(t.len(), 10);
        assert_eq!(t.stats(&3..&6), Some(Stats { sum: 3 + 4 + 5 }));
        assert_eq!(t.stats_full(), Some(Stats { sum: 9 * 10 / 2 }));
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Stats {
        pub sum: isize,
    }

    impl SetStats<isize> for Stats {
        fn compute(value: &isize, left: Option<&Self>, right: Option<&Self>) -> Self {
            let mut sum = *value;
            if let Some(left) = left {
                sum += left.sum;
            }
            if let Some(right) = right {
                sum += right.sum;
            }
            Stats { sum }
        }
    }
}
