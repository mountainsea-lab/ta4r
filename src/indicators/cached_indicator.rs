use crate::bar::types::BarSeries;
use crate::num::TrNum;

pub struct CachedIndicator<'a, T, S>
where
    T: TrNum + 'static,
    S: BarSeries<'a, T>,
{
    /// 关联的 BarSeries 引用，带生命周期
    bar_series: &'a S,

    /// 缓存结果，按 index 存储，使用 Option<T> 方便懒加载
    cache: Vec<Option<T>>,

    /// 当前缓存的最大索引，-1 表示空缓存
    highest_cached_index: isize,

    /// 最大缓存容量，一般与 BarSeries 最大 bar 数量保持一致
    max_cache_size: usize,
}
