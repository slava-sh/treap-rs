use map_stats::MapStats;
pub use map_stats::EmptyStats;

pub trait SetStats<T>: Clone {
    fn compute(value: &T, left: Option<&Self>, right: Option<&Self>) -> Self;
}

#[derive(Debug, Clone)]
pub struct SetStatsToMapStats<SS> {
    pub set_stats: SS,
}

impl<T, SS> MapStats<T, ()> for SetStatsToMapStats<SS>
where
    SS: SetStats<T> + Clone,
{
    fn compute(key: &T, _value: &(), left: Option<&Self>, right: Option<&Self>) -> Self {
        SetStatsToMapStats {
            set_stats: SS::compute(
                key,
                left.as_ref().map(|map_stats| &map_stats.set_stats),
                right.as_ref().map(|map_stats| &map_stats.set_stats),
            ),
        }
    }
}

impl<T> SetStats<T> for EmptyStats {
    fn compute(_value: &T, _left: Option<&Self>, _right: Option<&Self>) -> Self {
        EmptyStats
    }
}
