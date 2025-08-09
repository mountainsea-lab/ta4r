/*!
 * MIT License
 *
 * Copyright (c) 2025 Mountainsea
 * Based on ta4j (c) 2017–2025 Ta4j Organization & respective authors (see AUTHORS)
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */
use crate::bar::types::BarSeries;
use crate::indicators::Indicator;
use crate::indicators::abstract_indicator::BaseIndicator;
use crate::indicators::types::{IndicatorCalculator, IndicatorError};
use crate::num::TrNum;
use std::cell::RefCell;

pub struct CachedIndicator<'a, T, S, C>
where
    T: TrNum + 'static,
    S: BarSeries<'a, T>,
    C: IndicatorCalculator<'a, T, S> + Clone,
{
    pub(crate) base: BaseIndicator<'a, T, S>,
    results: RefCell<Vec<Option<T>>>,
    pub(crate) highest_result_index: RefCell<isize>,
    pub(crate) calculator: C,
}

impl<'a, T, S, C> Clone for CachedIndicator<'a, T, S, C>
where
    T: TrNum + Clone + 'static,
    S: BarSeries<'a, T>,
    C: IndicatorCalculator<'a, T, S> + Clone,
{
    fn clone(&self) -> Self {
        CachedIndicator {
            base: self.base.clone(), // 现在只拷贝引用，不要求 S: Clone
            results: RefCell::new(self.results.borrow().clone()),
            highest_result_index: RefCell::new(*self.highest_result_index.borrow()),
            calculator: self.calculator.clone(),
        }
    }
}

impl<'a, T, S, C> CachedIndicator<'a, T, S, C>
where
    T: TrNum + Clone + 'static,
    S: BarSeries<'a, T>,
    C: IndicatorCalculator<'a, T, S> + Clone,
{
    /// 根据序列容量创建 CachedIndicator，缓存容量初始化为 max_count 大小，元素初始化为 None
    pub fn new_from_series(series: &'a S, calculator: C) -> Self {
        let max_count = series.get_maximum_bar_count();
        let capacity = if max_count == usize::MAX {
            0
        } else {
            max_count
        };

        CachedIndicator {
            base: BaseIndicator::new(series),
            results: RefCell::new(vec![None; capacity]),
            highest_result_index: RefCell::new(-1),
            calculator,
        }
    }

    /// 通过已有指标构造，复用其 BarSeries
    pub fn new_from_indicator<I>(indicator: &'a I, calculator: C) -> Self
    where
        I: Indicator<Num = T, Series<'a> = S>,
    {
        Self::new_from_series(indicator.get_bar_series(), calculator)
    }

    pub fn calculator(&self) -> &C {
        &self.calculator
    }

    /// 调用计算函数，计算指定索引的指标值
    fn calculate(&self, index: usize) -> Result<T, IndicatorError> {
        self.calculator.calculate(&self.base, index)
    }

    /// 获取指定索引的指标值，自动缓存和扩容
    pub fn get_cached_value(&self, index: usize) -> Result<T, IndicatorError> {
        let series = self.base.get_bar_series();

        if series.get_bar_count() == 0 {
            // 无序列，直接计算不缓存
            return self.calculate(index);
        }

        let removed_bars_count = series.get_removed_bars_count();
        let maximum_result_count = series.get_maximum_bar_count();

        if index < removed_bars_count {
            // 请求的索引在被移除的范围，返回第0个缓存或计算
            self.increase_length_to(removed_bars_count, maximum_result_count);
            *self.highest_result_index.borrow_mut() = removed_bars_count as isize;

            let mut results_ref = self.results.borrow_mut();
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
                    // 最新bar不缓存，直接计算
                    return self.calculate(index);
                }
            }

            self.increase_length_to(index, maximum_result_count);

            let mut highest_result_index_ref = self.highest_result_index.borrow_mut();
            let mut results_ref = self.results.borrow_mut();

            if (index as isize) > *highest_result_index_ref {
                // 新索引超过缓存最高索引，计算并追加缓存
                *highest_result_index_ref = index as isize;
                let val = self.calculate(index)?;
                if let Some(last) = results_ref.last_mut() {
                    *last = Some(val.clone());
                } else {
                    results_ref.push(Some(val.clone()));
                }
                Ok(val)
            } else {
                // 索引已缓存，从缓存中取值
                let result_inner_index =
                    results_ref.len() - 1 - ((*highest_result_index_ref as usize) - index);

                if let Some(Some(value)) = results_ref.get(result_inner_index) {
                    Ok(value.clone())
                } else {
                    // 缓存中为空，计算后写入
                    let val = self.calculate(index)?;
                    if let Some(slot) = results_ref.get_mut(result_inner_index) {
                        *slot = Some(val.clone());
                    }
                    Ok(val)
                }
            }
        }
    }

    /// 根据索引扩容缓存，确保缓存容量不超过最大限制
    fn increase_length_to(&self, index: usize, max_length: usize) {
        let mut results = self.results.borrow_mut();
        let highest_result_index = self.highest_result_index.borrow_mut();

        if *highest_result_index >= 0 {
            let highest_index_usize = *highest_result_index as usize;

            if index > highest_index_usize {
                // 计算需要新增多少缓存槽
                let needed = index - highest_index_usize;
                let current_len = results.len();

                // 限制新增数量，不超过 max_length - current_len
                let available_space = max_length.saturating_sub(current_len);
                let add_count = std::cmp::min(needed, available_space);

                if add_count == max_length {
                    // 需要清空缓存，重新分配 max_length 个 None
                    results.clear();
                    results.resize(max_length, None);
                } else {
                    // 正常追加 None
                    results.extend(std::iter::repeat(None).take(add_count));
                    drop(results); // 提前释放可变借用

                    // 移除多余缓存，保持容量上限
                    self.remove_exceeding_results(max_length);
                }
            }
            // index <= highest_result_index，无需扩容
        } else {
            // 首次缓存，缓存应为空(因为指标初始化会提前分配内存)
            assert!(
                results.iter().all(|x| x.is_none()),
                "Cache should be empty on first use"
            );

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

impl<'a, T, S, C> Indicator for CachedIndicator<'a, T, S, C>
where
    T: TrNum + 'static,
    S: for<'any> BarSeries<'any, T>,
    C: IndicatorCalculator<'a, T, S> + Clone,
{
    type Num = T;
    type Series<'b>
        = S
    where
        Self: 'b;

    fn get_value(&self, index: usize) -> Result<T, IndicatorError> {
        self.get_cached_value(index)
    }

    fn get_bar_series(&self) -> &Self::Series<'_> {
        self.base.get_bar_series()
    }

    fn get_count_of_unstable_bars(&self) -> usize {
        0
    }
}
