use crate::bar::types::BarSeries;
use crate::indicators::cached_indicator::CachedIndicator;
use crate::indicators::types::IndicatorError;
use crate::num::TrNum;

/// 递归缓存指标，基于 CachedIndicator 进一步限制递归阈值，防止递归深度过大
pub struct RecursiveCachedIndicator<'a, T, S>
where
    T: TrNum,
    S: BarSeries<'a, T>,
{
    cached_indicator: CachedIndicator<'a, T, S>,
    recursion_threshold: usize,
}

impl<'a, T, S> RecursiveCachedIndicator<'a, T, S>
where
    T: TrNum,
    S: BarSeries<'a, T>,
{
    /// 默认递归阈值
    pub const DEFAULT_RECURSION_THRESHOLD: usize = 100;

    pub fn new(bar_series: &'a S) -> Self {
        Self {
            cached_indicator: CachedIndicator::new(bar_series),
            recursion_threshold: Self::DEFAULT_RECURSION_THRESHOLD,
        }
    }

    pub fn with_threshold(bar_series: &'a S, threshold: usize) -> Self {
        Self {
            cached_indicator: CachedIndicator::new(bar_series),
            recursion_threshold: threshold,
        }
    }

    /// 访问缓存指标的 bar series
    pub fn bar_series(&self) -> &'a S {
        self.cached_indicator.bar_series()
    }

    /// 设置递归阈值（可选）
    pub fn set_recursion_threshold(&mut self, threshold: usize) {
        self.recursion_threshold = threshold;
    }

    /// 计算指标值的方法，具体计算逻辑仍由 CachedIndicator 或子类实现
    /// 这里示范调用 cached_indicator.calculate(index)
    pub fn calculate(&self, index: usize) -> T {
        // 具体实现由用户自己重写
        unimplemented!("Please implement calculate method");
    }

    /// get_value 的递归阈值控制实现
    pub fn get_value(&mut self, index: usize) -> Result<T, IndicatorError> {
        let series = self.bar_series();

        if series.get_bar_count() == 0 || index > series.get_end_index() {
            // 超出范围，退回 CachedIndicator 默认行为
            return self.cached_indicator.get_value(index);
        }

        let start_index = std::cmp::max(series.get_removed_bars_count(), self.cached_indicator.highest_result_index);

        if index > start_index && index - start_index > self.recursion_threshold {
            // 递归深度过大，改用迭代计算之前的值
            for prev_index in start_index..index {
                self.cached_indicator.get_value(prev_index)?;
            }
        }

        self.cached_indicator.get_value(index)
    }
}
