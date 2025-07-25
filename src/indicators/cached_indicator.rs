use crate::bar::types::BarSeries;
use crate::indicators::Indicator;
use crate::indicators::abstract_indicator::BaseIndicator;
use crate::indicators::types::IndicatorError;
use crate::num::TrNum;

/// CachedIndicator 状态和缓存
#[derive(Clone)]
pub struct CachedIndicator<'a, T, S, F>
where
    T: TrNum + 'static,
    S: BarSeries<'a, T>,
    F: Fn(&Self, usize) -> Result<T, IndicatorError> + Clone,
{
    pub(crate) base: BaseIndicator<'a, T, S>,

    results: Vec<Option<T>>,
    pub highest_result_index: isize, // -1 表示空缓存

    calculate_fn: F,
}

impl<'a, T, S, F> CachedIndicator<'a, T, S, F>
where
    T: TrNum + 'static,
    S: BarSeries<'a, T>,
    F: Fn(&Self, usize) -> Result<T, IndicatorError> + Clone,
{
    /// 对应 CachedIndicator(BarSeries series)
    pub fn new_from_series(series: &'a S, calculate_fn: F) -> Self {
        let max_count = series.get_maximum_bar_count();
        let capacity = if max_count == usize::MAX {
            0
        } else {
            max_count
        };
        let results = Vec::with_capacity(capacity);

        CachedIndicator {
            base: BaseIndicator::new(series),
            results,
            highest_result_index: -1,
            calculate_fn,
        }
    }

    /// 对应 CachedIndicator(Indicator<?> indicator)
    pub fn new_from_indicator<I>(indicator: &'a I, calculate_fn: F) -> Self
    where
        I: Indicator<Num = T, Series<'a> = S>,
    {
        Self::new_from_series(indicator.get_bar_series(), calculate_fn)
    }

    /// 计算具体指标值（调用传入的闭包）
    fn calculate(&self, index: usize) -> Result<T, IndicatorError> {
        (self.calculate_fn)(self, index)
    }

    /// 关键同步缓存逻辑，类似 Java getValue()
    pub fn get_value(&mut self, index: usize) -> Result<T, IndicatorError> {
        let series = self.base.get_bar_series();

        if series.get_bar_count() == 0 {
            // 无序列，直接计算不缓存
            return self.calculate(index);
        }

        let removed_bars_count = series.get_removed_bars_count();
        let maximum_result_count = series.get_maximum_bar_count();

        if index < removed_bars_count {
            // 结果已被移除缓存，返回最近缓存的第0个结果
            self.increase_length_to(removed_bars_count, maximum_result_count);
            self.highest_result_index = removed_bars_count as isize;

            if let Some(Some(value)) = self.results.get(0) {
                Ok(value.clone())
            } else {
                let val = self.calculate(0)?;
                if let Some(slot) = self.results.get_mut(0) {
                    *slot = Some(val.clone());
                }
                Ok(val)
            }
        } else {
            if let Some(end_index) = series.get_end_index() {
                if index == end_index {
                    // 最后一个bar，不缓存，直接计算
                    return self.calculate(index);
                }
            }

            self.increase_length_to(index, maximum_result_count);

            if (index as isize) > self.highest_result_index {
                self.highest_result_index = index as isize;
                let val = self.calculate(index)?;
                if let Some(last) = self.results.last_mut() {
                    *last = Some(val.clone());
                } else {
                    self.results.push(Some(val.clone()));
                }
                Ok(val)
            } else {
                let result_inner_index =
                    self.results.len() - 1 - ((self.highest_result_index as usize) - index);

                if let Some(Some(value)) = self.results.get(result_inner_index) {
                    Ok(value.clone())
                } else {
                    let val = self.calculate(index)?;
                    if let Some(slot) = self.results.get_mut(result_inner_index) {
                        *slot = Some(val.clone());
                    }
                    Ok(val)
                }
            }
        }
    }

    /// 扩容缓存，增加到index所需大小
    fn increase_length_to(&mut self, index: usize, max_length: usize) {
        if self.highest_result_index > -1 {
            let diff = index as isize - self.highest_result_index;
            let new_results_count = std::cmp::min(diff as usize, max_length);
            if new_results_count == max_length {
                self.results.clear();
                self.results.resize(max_length, None);
            } else if new_results_count > 0 {
                self.results
                    .extend(std::iter::repeat(None).take(new_results_count));
                self.remove_exceeding_results(max_length);
            }
        } else {
            // 第一次使用缓存
            assert!(self.results.is_empty(), "Cache should be empty");
            self.results
                .resize(std::cmp::min(index + 1, max_length), None);
        }
    }

    /// 移除超过最大缓存长度的旧数据
    fn remove_exceeding_results(&mut self, max_length: usize) {
        if self.results.len() > max_length {
            let excess = self.results.len() - max_length;
            self.results.drain(0..excess);
        }
    }
}
