extern crate rand;

use node::Node;

#[derive(Debug, Clone)]
pub struct TreapMap<K, V, Rng> {
    root: Option<Box<Node<K, V>>>,
    len: usize,
    rng: Rng,
}

impl<K, V> TreapMap<K, V, rand::XorShiftRng> {
    pub fn new() -> Self {
        TreapMap {
            root: None,
            len: 0,
            rng: rand::weak_rng(),
        }
    }
}

impl<K, V> Default for TreapMap<K, V, rand::XorShiftRng> {
    fn default() -> Self {
        TreapMap::new()
    }
}

impl<K, V, Rng> TreapMap<K, V, Rng>
where
    K: Ord,
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
}
