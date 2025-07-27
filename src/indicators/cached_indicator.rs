use crate::bar::types::BarSeries;
use crate::indicators::Indicator;
use crate::indicators::abstract_indicator::BaseIndicator;
use crate::indicators::types::IndicatorError;
use crate::num::TrNum;
use std::cell::RefCell;

/// CachedIndicator 状态和缓存
pub struct CachedIndicator<'a, T, S, F>
where
    T: TrNum + 'static,
    S: BarSeries<'a, T>,
    F: Fn(&Self, usize) -> Result<T, IndicatorError> + Clone,
{
    pub(crate) base: BaseIndicator<'a, T, S>,
    results: RefCell<Vec<Option<T>>>, // 用 RefCell 实现内部可变性
    pub(crate) highest_result_index: RefCell<isize>, // 同理
    calculate_fn: F,
}

impl<'a, T, S, F> Clone for CachedIndicator<'a, T, S, F>
where
    T: TrNum + Clone + 'static,
    S: BarSeries<'a, T>,
    F: Fn(&Self, usize) -> Result<T, IndicatorError> + Clone,
{
    fn clone(&self) -> Self {
        Self {
            base: self.base.clone(),
            results: RefCell::new(self.results.borrow().clone()),
            highest_result_index: RefCell::new(*self.highest_result_index.borrow()),
            calculate_fn: self.calculate_fn.clone(),
        }
    }
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
            results: RefCell::new(results),
            highest_result_index: RefCell::new(-1),
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
    pub fn get_value(&self, index: usize) -> Result<T, IndicatorError> {
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
            *self.highest_result_index.borrow_mut() = removed_bars_count as isize;

            let results_ref = &mut *self.results.borrow_mut();
            if let Some(Some(value)) = results_ref.get(0) {
                Ok(value.clone())
            } else {
                let val = self.calculate(0)?;
                if let Some(slot) = results_ref.get_mut(0) {
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

            let mut highest_result_index_ref = self.highest_result_index.borrow_mut();
            let mut results_ref = self.results.borrow_mut();

            if (index as isize) > *highest_result_index_ref {
                *highest_result_index_ref = index as isize;
                let val = self.calculate(index)?;
                if let Some(last) = results_ref.last_mut() {
                    *last = Some(val.clone());
                } else {
                    results_ref.push(Some(val.clone()));
                }
                Ok(val)
            } else {
                let result_inner_index =
                    results_ref.len() - 1 - ((*highest_result_index_ref as usize) - index);

                if let Some(Some(value)) = results_ref.get(result_inner_index) {
                    Ok(value.clone())
                } else {
                    let val = self.calculate(index)?;
                    if let Some(slot) = results_ref.get_mut(result_inner_index) {
                        *slot = Some(val.clone());
                    }
                    Ok(val)
                }
            }
        }
    }

    fn increase_length_to(&self, index: usize, max_length: usize) {
        let mut results = self.results.borrow_mut();
        let mut highest_result_index = self.highest_result_index.borrow_mut();

        if *highest_result_index >= 0 {
            let highest_index_usize = *highest_result_index as usize;

            if index > highest_index_usize {
                // 计算需要新增多少缓存槽
                let needed = index - highest_index_usize;

                // 这里的扩容数量不应超过 max_length
                // 但也不应该直接用 max_length，防止一次扩容过大
                let add_count = std::cmp::min(needed, max_length);

                // 当新增槽数等于 max_length 时，说明缓存要整体清理重置
                if add_count == max_length {
                    // 清空缓存，重新分配 max_length 个 None
                    results.clear();
                    results.resize(max_length, None);
                } else {
                    // 正常追加 None 作为缓存空间
                    results.extend(std::iter::repeat(None).take(add_count));
                    drop(results); // 释放借用

                    // 移除多余缓存，保持容量上限
                    self.remove_exceeding_results(max_length);
                }
            }
            // else index <= highest_result_index，不需要扩容
        } else {
            // 首次缓存使用，缓存应该为空
            assert!(results.is_empty(), "Cache should be empty on first use");

            // 初始化缓存长度：index + 1，但不超过最大容量 max_length
            let init_len = std::cmp::min(index + 1, max_length);
            results.resize(init_len, None);
        }
    }

    /// 删除多余缓存，保持最近 max_length 条
    fn remove_exceeding_results(&self, max_length: usize) {
        let mut results = self.results.borrow_mut();
        let result_count = results.len();

        if result_count > max_length {
            let nb_results_to_remove = result_count - max_length;
            if nb_results_to_remove == 1 {
                results.remove(0);
            } else {
                results.drain(0..nb_results_to_remove);
            }
        }
    }
}

impl<'a, T, S, F> Indicator for CachedIndicator<'a, T, S, F>
where
    T: TrNum + 'static,
    S: for<'any> BarSeries<'any, T>,
    F: Fn(&Self, usize) -> Result<T, IndicatorError> + Clone,
{
    type Num = T;
    type Series<'b>
        = S
    where
        Self: 'b;

    fn get_value(&self, index: usize) -> Result<T, IndicatorError> {
        self.get_value(index)
    }

    fn get_bar_series(&self) -> &Self::Series<'_> {
        self.base.get_bar_series()
    }

    fn get_count_of_unstable_bars(&self) -> usize {
        0
    }
}
