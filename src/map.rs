extern crate rand;

use node;
use node::Node;
use std::ops::Range;

#[derive(Debug, Clone)]
pub struct TreapMap<K, V, S = node::EmptyStats, Rng = rand::XorShiftRng> {
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
    S: node::NodeStats<K, V>,
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
    fn state() {
        let mut t: TreapMap<_, _, ValueSum> = TreapMap::default();
        for i in 0..10 {
            assert_eq!(t.insert(i, i), None);
        }
        assert_eq!(t.len(), 10);
        assert_eq!(t.stats(&3..&6), Some(ValueSum { sum: 3 + 4 + 5 }));
        assert_eq!(t.stats_full(), Some(ValueSum { sum: 9 * 10 / 2 }));
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct ValueSum {
        pub sum: i32,
    }

    impl<K> node::NodeStats<K, i32> for ValueSum {
        fn compute(_key: &K, value: &i32, left: Option<&Self>, right: Option<&Self>) -> Self {
            let mut sum = *value;
            if let Some(left) = left {
                sum += left.sum;
            }
            if let Some(right) = right {
                sum += right.sum;
            }
            ValueSum { sum }
        }
    }
}
