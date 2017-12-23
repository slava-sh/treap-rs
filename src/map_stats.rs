pub trait MapStats<K, V>: Clone {
    fn compute(key: &K, value: &V, left: Option<&Self>, right: Option<&Self>) -> Self;
}

#[derive(Debug, Clone)]
pub struct EmptyStats;

impl<K, V> MapStats<K, V> for EmptyStats {
    fn compute(_key: &K, _value: &V, _left: Option<&Self>, _right: Option<&Self>) -> Self {
        EmptyStats
    }
}
