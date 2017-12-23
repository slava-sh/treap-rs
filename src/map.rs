extern crate rand;

use map_stats::{MapStats, EmptyStats};
use node::Node;
use std::ops::Range;

#[derive(Debug, Clone)]
pub struct TreapMap<K, V, S = EmptyStats, Rng = rand::XorShiftRng> {
    root: Option<Box<Node<K, V, S>>>,
    len: usize,
    rng: Rng,
}

impl<K, V> TreapMap<K, V> {
    pub fn new() -> Self {
        Default::default()
    }
}

impl<K, V, S> Default for TreapMap<K, V, S> {
    fn default() -> Self {
        TreapMap {
            root: None,
            len: 0,
            rng: rand::weak_rng(),
        }
    }
}

impl<K, V, S, Rng> TreapMap<K, V, S, Rng>
where
    K: Ord,
    S: MapStats<K, V>,
    Rng: rand::Rng,
{
    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn clear(&mut self) {
        self.root.take();
        self.len = 0;
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        let new_node = Node::new(key, value, self.rng.gen());
        Node::insert_or_replace(&mut self.root, new_node)
            .map(|old_node| old_node.value)
            .or_else(|| {
                self.len += 1;
                None
            })
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        Node::remove(&mut self.root, key).map(|node| {
            self.len -= 1;
            node.value
        })
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        Node::get(&self.root, key).as_ref().map(|node| &node.value)
    }

    pub fn stats(&self, key_range: Range<&K>) -> Option<S> {
        Node::with_range(&self.root, key_range, |node| node.stats.clone())
    }

    pub fn stats_full(&self) -> Option<S> {
        self.root.as_ref().map(|node| node.stats.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let mut t = TreapMap::new();
        assert!(t.is_empty());
        assert_eq!(t.insert("a", 0), None);
        assert!(!t.is_empty());
        assert_eq!(t.len(), 1);
        assert_eq!(t.insert("a", 1), Some(0));
        assert_eq!(t.len(), 1);
        assert_eq!(t.insert("b", 4), None);
        assert_eq!(t.len(), 2);
        assert_eq!(t.remove(&"a"), Some(1));
        assert_eq!(t.len(), 1);
        assert_eq!(t.get(&"a"), None);
        assert_eq!(t.remove(&"a"), None);
        assert_eq!(t.get(&"b"), Some(&4));
        t.clear();
        assert!(t.is_empty());
    }

    #[test]
    fn stats() {
        let mut t: TreapMap<_, _, Stats> = TreapMap::default();
        for i in 0..10 {
            assert_eq!(t.insert(i, i * 10), None);
        }
        assert_eq!(t.len(), 10);
        assert_eq!(
            t.stats(&3..&6),
            Some(Stats {
                key_sum: (3 + 4 + 5),
                value_sum: (3 + 4 + 5) * 10,
            })
        );
        assert_eq!(
            t.stats_full(),
            Some(Stats {
                key_sum: 9 * (9 + 1) / 2,
                value_sum: 9 * (9 + 1) / 2 * 10,
            })
        );
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Stats {
        pub key_sum: isize,
        pub value_sum: isize,
    }

    impl MapStats<isize, isize> for Stats {
        fn compute(key: &isize, value: &isize, left: Option<&Self>, right: Option<&Self>) -> Self {
            let mut key_sum = *key;
            let mut value_sum = *value;
            if let Some(left) = left {
                key_sum += left.key_sum;;
                value_sum += left.value_sum;
            }
            if let Some(right) = right {
                key_sum += right.key_sum;;
                value_sum += right.value_sum;
            }
            Stats { key_sum, value_sum }
        }
    }
}
