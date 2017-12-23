use std::cmp::Ordering;
use std::ops::Range;

#[derive(Debug, Clone)]
pub struct Node<K, V, S> {
    pub key: K,
    pub value: V,
    pub stats: S,
    priority: usize,
    left: Option<Box<Node<K, V, S>>>,
    right: Option<Box<Node<K, V, S>>>,
}

pub trait NodeStats<K, V>: Clone {
    fn compute(key: &K, value: &V, left: Option<&Self>, right: Option<&Self>) -> Self;
}

impl<K, V, S> Node<K, V, S>
where
    K: Ord,
    S: NodeStats<K, V>,
{
    pub fn new(key: K, value: V, priority: usize) -> Node<K, V, S> {
        let stats = NodeStats::compute(&key, &value, None, None);
        Node {
            key,
            value,
            stats,
            priority,
            left: None,
            right: None,
        }
    }

    fn update_stats(&mut self) {
        self.stats = NodeStats::compute(
            &self.key,
            &self.value,
            self.left.as_ref().map(|node| &node.stats),
            self.right.as_ref().map(|node| &node.stats),
        )
    }

    fn split3(
        node: Option<Box<Node<K, V, S>>>,
        key: &K,
    ) -> (Option<Box<Node<K, V, S>>>, Option<Box<Node<K, V, S>>>, Option<Box<Node<K, V, S>>>) {
        match node {
            None => (None, None, None),
            Some(mut node) => {
                match node.key.cmp(key) {
                    Ordering::Equal => {
                        let left = node.left.take();
                        let right = node.right.take();
                        node.update_stats();
                        (left, Some(node), right)
                    }
                    Ordering::Less => {
                        let (node_right, middle, right) = Node::split3(node.right.take(), key);
                        node.right = node_right;
                        node.update_stats();
                        (Some(node), middle, right)
                    }
                    Ordering::Greater => {
                        let (left, middle, node_left) = Node::split3(node.left.take(), key);
                        node.left = node_left;
                        node.update_stats();
                        (left, middle, Some(node))
                    }
                }
            }
        }
    }

    fn split2(
        node: Option<Box<Node<K, V, S>>>,
        key: &K,
    ) -> (Option<Box<Node<K, V, S>>>, Option<Box<Node<K, V, S>>>) {
        match node {
            None => (None, None),
            Some(mut node) => {
                if node.key < *key {
                    let (node_right, right) = Node::split2(node.right.take(), key);
                    node.right = node_right;
                    node.update_stats();
                    (Some(node), right)
                } else {
                    let (left, node_left) = Node::split2(node.left.take(), key);
                    node.left = node_left;
                    node.update_stats();
                    (left, Some(node))
                }
            }
        }
    }

    fn merge2(
        left: Option<Box<Node<K, V, S>>>,
        right: Option<Box<Node<K, V, S>>>,
    ) -> Option<Box<Node<K, V, S>>> {
        match (left, right) {
            (left, None) => left,
            (None, right) => right,
            (Some(mut left), Some(mut right)) => {
                let mut node = if left.priority <= right.priority {
                    left.right = Node::merge2(left.right.take(), Some(right));
                    left
                } else {
                    right.left = Node::merge2(Some(left), right.left.take());
                    right
                };
                node.update_stats();
                Some(node)
            }
        }
    }

    fn merge3(
        left: Option<Box<Node<K, V, S>>>,
        middle: Option<Box<Node<K, V, S>>>,
        right: Option<Box<Node<K, V, S>>>,
    ) -> Option<Box<Node<K, V, S>>> {
        Node::merge2(Node::merge2(left, middle), right)
    }

    pub fn insert_or_replace(
        root: &mut Option<Box<Node<K, V, S>>>,
        new: Node<K, V, S>,
    ) -> Option<Box<Node<K, V, S>>> {
        let (left, old, right) = Node::split3(root.take(), &new.key);
        *root = Node::merge3(left, Some(Box::new(new)), right);
        old
    }

    pub fn remove(root: &mut Option<Box<Node<K, V, S>>>, key: &K) -> Option<Box<Node<K, V, S>>> {
        let (left, node, right) = Node::split3(root.take(), key);
        *root = Node::merge2(left, right);
        node
    }

    pub fn get<'a>(root: &'a Option<Box<Node<K, V, S>>>, key: &K) -> Option<&'a Node<K, V, S>> {
        let mut next_node = root;
        while let Some(ref node) = *next_node {
            match node.key.cmp(key) {
                Ordering::Equal => return Some(node),
                Ordering::Less => next_node = &node.right,
                Ordering::Greater => next_node = &node.left,
            }
        }
        None
    }

    pub fn with_range<F, R>(
        root: &Option<Box<Node<K, V, S>>>,
        key_range: Range<&K>,
        f: F,
    ) -> Option<R>
    where
        F: FnOnce(&Node<K, V, S>) -> R,
    {
        let root = unsafe_mut(root);
        let (left, right) = Node::split2(root.take(), key_range.end);
        let (left, middle) = Node::split2(left, key_range.start);
        let result = middle.as_ref().map(|node_box| f(node_box.as_ref()));
        *root = Node::merge3(left, middle, right);
        result
    }
}

fn unsafe_mut<T>(value: &T) -> &mut T {
    unsafe { &mut *(value as *const T as *mut T) }
}

#[derive(Debug, Clone)]
pub struct EmptyStats;

impl<K, V> NodeStats<K, V> for EmptyStats {
    fn compute(_key: &K, _value: &V, _left: Option<&Self>, _right: Option<&Self>) -> Self {
        EmptyStats
    }
}
