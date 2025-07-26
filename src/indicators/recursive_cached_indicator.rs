use crate::bar::types::BarSeries;
use crate::indicators::Indicator;
use crate::indicators::cached_indicator::CachedIndicator;
use crate::indicators::types::IndicatorError;
use crate::num::TrNum;

const RECURSION_THRESHOLD: usize = 100;

/// 递归缓存指标
pub struct RecursiveCachedIndicator<'a, T, S, F>
where
    T: TrNum + 'static,
    S: BarSeries<'a, T>,
    F: Fn(&CachedIndicator<'a, T, S, F>, usize) -> Result<T, IndicatorError> + Clone,
{
    inner: CachedIndicator<'a, T, S, F>,
}

impl<'a, T, S, F> RecursiveCachedIndicator<'a, T, S, F>
where
    T: TrNum + 'static,
    S: BarSeries<'a, T>,
    F: Fn(&CachedIndicator<'a, T, S, F>, usize) -> Result<T, IndicatorError> + Clone,
{
    pub fn new(series: &'a S, calculate_fn: F) -> Self {
        Self {
            inner: CachedIndicator::new_from_series(series, calculate_fn),
        }
    }

    pub fn new_from_indicator<I>(indicator: &'a I, calculate_fn: F) -> Self
    where
        I: Indicator<Num = T, Series<'a> = S>,
    {
        Self {
            inner: CachedIndicator::new_from_indicator(indicator, calculate_fn),
        }
    }

    pub fn get_value(&self, index: usize) -> Result<T, IndicatorError> {
        let series = self.inner.base.get_bar_series();

        if let Some(end_index) = series.get_end_index() {
            if index <= end_index {
                let removed_bars_count = series.get_removed_bars_count();

                let highest_result_index = *self.inner.highest_result_index.borrow();

                let highest_result_index_usize = if highest_result_index < 0 {
                    0
                } else {
                    highest_result_index as usize
                };

                let start_index = std::cmp::max(removed_bars_count, highest_result_index_usize);

                if index > start_index + RECURSION_THRESHOLD {
                    for prev_index in start_index..index {
                        // 这里调用 inner.get_value(&self, usize) 也是不可变借用，
                        // 内部用 RefCell 管理可变性，没问题
                        self.inner.get_value(prev_index)?;
                    }
                }
            }
        }

        self.inner.get_value(index)
    }

    pub fn get_bar_series(&self) -> &'a S {
        self.inner.base.get_bar_series()
    }

    pub fn into_inner(self) -> CachedIndicator<'a, T, S, F> {
        self.inner
    }
}
