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
use crate::bar::builder::types::BarSeriesRef;
use crate::bar::types::BarSeries;
use crate::indicators::Indicator;
use crate::indicators::abstract_indicator::BaseIndicator;
use crate::indicators::types::{IndicatorCalculator, IndicatorError};
use crate::num::TrNum;
use std::cell::RefCell;

pub struct CachedIndicator<T, S, C>
where
    T: TrNum + 'static,
    S: BarSeries<T>,
    C: IndicatorCalculator<T, S> + Clone,
{
    pub(crate) base: BaseIndicator<T, S>,
    results: RefCell<Vec<Option<C::Output>>>,
    pub(crate) highest_result_index: RefCell<isize>,
    pub(crate) calculator: C,
}

impl<'a, T, S, C> Clone for CachedIndicator<T, S, C>
where
    T: TrNum + Clone + 'static,
    S: BarSeries<T> + 'static,
    C: IndicatorCalculator<T, S> + Clone,
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

impl<T, S, C> CachedIndicator<T, S, C>
where
    T: TrNum + Clone + 'static,
    S: BarSeries<T> + 'static,
    C: IndicatorCalculator<T, S> + Clone,
{
    /// 根据序列容量创建 CachedIndicator，缓存容量初始化为 max_count 大小，元素初始化为 None
    pub fn new_from_series(series: BarSeriesRef<S>, calculator: C) -> Self {
        let max_count = series.with_ref_or(0, |s| s.get_maximum_bar_count());

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
    pub fn new_from_indicator<I>(indicator: &I, calculator: C) -> Self
    where
        I: Indicator<Num = T, Output = C::Output, Series = S>,
    {
        Self::new_from_series(indicator.bar_series(), calculator)
    }

    pub fn calculator(&self) -> &C {
        &self.calculator
    }

    /// 调用计算函数，计算指定索引的指标值
    fn calculate(&self, index: usize) -> Result<C::Output, IndicatorError> {
        self.calculator.calculate(&self.base, index)
    }

    /// 获取指定索引的指标值，自动缓存和扩容
    pub fn get_cached_value(&self, index: usize) -> Result<C::Output, IndicatorError> {
        let series_guard = self.base.bar_series();

        // 获取 bar_count，空序列直接计算
        let bar_count = series_guard.with_ref_or(0, |s| s.get_bar_count());
        if bar_count == 0 {
            return self.calculate(index);
        }

        let removed_count = series_guard.with_ref_or(0, |s| s.get_removed_bars_count());
        let max_count = series_guard.with_ref_or(usize::MAX, |s| s.get_maximum_bar_count());
        let end_index = series_guard.with_ref_or(None, |s| s.get_end_index());

        // 请求索引在被移除的范围
        if index < removed_count {
            self.increase_length_to(removed_count, max_count);
            *self.highest_result_index.borrow_mut() = removed_count as isize;

            let mut results = self.results.borrow_mut();
            if results.is_empty() {
                results.push(None);
            }

            if let Some(Some(value)) = results.get(0) {
                return Ok(value.clone());
            }

            let val = self.calculate(0)?;
            results[0] = Some(val.clone());
            return Ok(val);
        }

        // 最新 bar 不缓存
        if end_index == Some(index) {
            return self.calculate(index);
        }

        self.increase_length_to(index, max_count);

        let mut highest_index = self.highest_result_index.borrow_mut();
        let mut results = self.results.borrow_mut();

        if (index as isize) > *highest_index {
            // 新索引超过缓存最高索引
            *highest_index = index as isize;
            let val = self.calculate(index)?;

            if results.len() < max_count {
                results.push(Some(val.clone()));
            } else {
                // 循环缓冲写入
                let write_pos = results.len() % max_count;
                results[write_pos] = Some(val.clone());
            }

            return Ok(val);
        }

        // 索引已缓存，从缓存取值
        let offset = (*highest_index as usize) - index;
        let inner_index = (results.len() + results.len() - 1 - offset) % results.len();

        if let Some(Some(value)) = results.get(inner_index) {
            Ok(value.clone())
        } else {
            let val = self.calculate(index)?;
            if let Some(slot) = results.get_mut(inner_index) {
                *slot = Some(val.clone());
            }
            Ok(val)
        }
    }

    // pub fn get_cached_value(&self, index: usize) -> Result<C::Output, IndicatorError> {
    //     let series_guard = self.base.bar_series();
    //     // 参考这个获取方式 修改相关代码
    //     let bar_count = series_guard.with_ref_or(0, |s| s.get_bar_count());
    //
    //     // 空序列，直接计算
    //     if bar_count == 0 {
    //         return self.calculate(index);
    //     }
    //
    //     let removed_count = series_guard.get_removed_bars_count();
    //     let max_count = series_guard.get_maximum_bar_count();
    //
    //     // 请求索引在被移除的范围
    //     if index < removed_count {
    //         self.increase_length_to(removed_count, max_count);
    //         *self.highest_result_index.borrow_mut() = removed_count as isize;
    //
    //         let mut results = self.results.borrow_mut();
    //         if results.is_empty() {
    //             results.push(None);
    //         }
    //
    //         if let Some(Some(value)) = results.get(0) {
    //             return Ok(value.clone());
    //         }
    //
    //         let val = self.calculate(0)?;
    //         results[0] = Some(val.clone());
    //         return Ok(val);
    //     }
    //
    //     // 最新 bar 不缓存
    //     if series_guard.get_end_index() == Some(index) {
    //         return self.calculate(index);
    //     }
    //
    //     self.increase_length_to(index, max_count);
    //
    //     let mut highest_index = self.highest_result_index.borrow_mut();
    //     let mut results = self.results.borrow_mut();
    //
    //     if (index as isize) > *highest_index {
    //         // 新索引超过缓存最高索引
    //         *highest_index = index as isize;
    //         let val = self.calculate(index)?;
    //
    //         // 循环缓冲写入
    //         if results.len() < max_count {
    //             results.push(Some(val.clone()));
    //         } else {
    //             // 使用 modulo 替代 remove(0)
    //             let write_pos = results.len() % max_count;
    //             results[write_pos] = Some(val.clone());
    //         }
    //
    //         return Ok(val);
    //     }
    //
    //     // 索引已缓存，从缓存取值
    //     let offset = (*highest_index as usize) - index;
    //     let inner_index = (results.len() + results.len() - 1 - offset) % results.len();
    //
    //     if let Some(Some(value)) = results.get(inner_index) {
    //         Ok(value.clone())
    //     } else {
    //         let val = self.calculate(index)?;
    //         if let Some(slot) = results.get_mut(inner_index) {
    //             *slot = Some(val.clone());
    //         }
    //         Ok(val)
    //     }
    // }

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

impl<T, S, C> Indicator for CachedIndicator<T, S, C>
where
    T: TrNum + 'static,
    S: BarSeries<T> + 'static,
    C: IndicatorCalculator<T, S> + Clone,
{
    type Num = T;
    type Output = C::Output;
    type Series = S;

    fn get_value(&self, index: usize) -> Result<Self::Output, IndicatorError> {
        self.get_cached_value(index)
    }

    fn bar_series(&self) -> BarSeriesRef<S> {
        self.base.bar_series()
    }

    fn count_of_unstable_bars(&self) -> usize {
        0
    }
}
