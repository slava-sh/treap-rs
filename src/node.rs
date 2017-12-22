use std::cmp::Ordering;

pub struct Node<K, V> {
    pub key: K,
    pub value: V,
    priority: usize,
    left: Option<Box<Node<K, V>>>,
    right: Option<Box<Node<K, V>>>,
}

impl<K, V> Node<K, V>
where
    K: Ord,
{
    pub fn new(key: K, value: V, priority: usize) -> Node<K, V> {
        Node {
            key,
            value,
            priority,
            left: None,
            right: None,
        }
    }

    fn update(&mut self) {}

    fn split3(
        node: Option<Box<Node<K, V>>>,
        key: &K,
    ) -> (Option<Box<Node<K, V>>>, Option<Box<Node<K, V>>>, Option<Box<Node<K, V>>>) {
        match node {
            None => (None, None, None),
            Some(mut node) => {
                match node.key.cmp(key) {
                    Ordering::Equal => {
                        let left = node.left.take();
                        let right = node.right.take();
                        node.update();
                        (left, Some(node), right)
                    }
                    Ordering::Less => {
                        let (node_right, middle, right) = Node::split3(node.right.take(), key);
                        node.right = node_right;
                        node.update();
                        (Some(node), middle, right)
                    }
                    Ordering::Greater => {
                        let (left, middle, node_left) = Node::split3(node.left.take(), key);
                        node.left = node_left;
                        node.update();
                        (left, middle, Some(node))
                    }
                }
            }
        }
    }

    fn merge2(
        left: Option<Box<Node<K, V>>>,
        right: Option<Box<Node<K, V>>>,
    ) -> Option<Box<Node<K, V>>> {
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
                node.update();
                Some(node)
            }
        }
    }

    fn merge3(
        left: Option<Box<Node<K, V>>>,
        middle: Option<Box<Node<K, V>>>,
        right: Option<Box<Node<K, V>>>,
    ) -> Option<Box<Node<K, V>>> {
        Node::merge2(Node::merge2(left, middle), right)
    }

    pub fn insert_or_replace(
        root: &mut Option<Box<Node<K, V>>>,
        new: Node<K, V>,
    ) -> Option<Box<Node<K, V>>> {
        let (left, old, right) = Node::split3(root.take(), &new.key);
        *root = Node::merge3(left, Some(Box::new(new)), right);
        old
    }
}
