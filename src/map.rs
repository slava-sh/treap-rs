extern crate rand;

use node::Node;

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

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        let new_node = Node::new(key, value, self.rng.gen());
        match Node::insert_or_replace(&mut self.root, new_node) {
            Some(old_node) => Some(old_node.value),
            None => {
                self.len += 1;
                None
            }
        }
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
    }
}
