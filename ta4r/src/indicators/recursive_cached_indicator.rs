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
use crate::indicators::cached_indicator::CachedIndicator;
use crate::indicators::types::{IndicatorCalculator, IndicatorError};
use crate::num::TrNum;
use parking_lot::RwLock;
use std::cell::RefCell;
use std::sync::Arc;

const RECURSION_THRESHOLD: usize = 100;

pub struct RecursiveCalcWrapper<C> {
    pub(crate) inner: C,
    pub(crate) threshold: usize,
}

impl<C> Clone for RecursiveCalcWrapper<C>
where
    C: Clone,
{
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            threshold: self.threshold,
        }
    }
}

impl<'a, T, S, C> IndicatorCalculator<T, S> for RecursiveCalcWrapper<C>
where
    T: TrNum + Clone + 'static,
    S: BarSeries<T> + 'static,
    C: IndicatorCalculator<T, S> + Clone,
{
    type Output = C::Output;

    fn calculate(
        &self,
        base: &BaseIndicator<T, S>,
        index: usize,
    ) -> Result<Self::Output, IndicatorError> {
        // 不负责递归预计算，直接调用内层计算器
        self.inner.calculate(base, index)
    }
}

pub struct RecursiveCachedIndicator<T, S, C>
where
    T: TrNum + Clone + 'static,
    S: BarSeries<T> + 'static,
    C: IndicatorCalculator<T, S> + Clone,
{
    pub(crate) cached: CachedIndicator<T, S, RecursiveCalcWrapper<C>>,
}

impl<T, S, C> Clone for RecursiveCachedIndicator<T, S, C>
where
    T: TrNum + Clone + 'static,
    S: BarSeries<T> + 'static,
    C: IndicatorCalculator<T, S> + Clone,
{
    fn clone(&self) -> Self {
        Self {
            cached: self.cached.clone(),
        }
    }
}

impl<T, S, C> RecursiveCachedIndicator<T, S, C>
where
    T: TrNum + Clone + 'static,
    S: BarSeries<T> + 'static,
    C: IndicatorCalculator<T, S, Output = T> + Clone,
{
    /// General construction Creates indicator based on the given bar series.
    pub fn new(series_ref: BarSeriesRef<S>, calculator: C) -> Self {
        Self::new_with_threshold(series_ref, calculator, RECURSION_THRESHOLD)
    }

    pub fn new_with_threshold(
        series_ref: BarSeriesRef<S>,
        calculator: C,
        threshold: usize,
    ) -> Self {
        let wrapper = RecursiveCalcWrapper {
            inner: calculator,
            threshold,
        };
        Self {
            cached: CachedIndicator::new_from_series(series_ref, wrapper),
        }
    }

    /// 快捷方式：从 Arc<RwLock<S>> 构造
    pub fn from_shared(series: Arc<RwLock<S>>, calculator: C) -> Self {
        Self::new(BarSeriesRef::Shared(series), calculator)
    }

    /// 快捷方式：从 Rc<RefCell<S>> 构造
    pub fn from_mut(series: Arc<RefCell<S>>, calculator: C) -> Self {
        Self::new(BarSeriesRef::Mut(series), calculator)
    }

    /// 从现有 Indicator 构造，使用默认阈值
    pub fn from_indicator<I>(indicator: Arc<I>, calculator: C) -> Self
    where
        I: Indicator<Num = T, Output = T, Series = S>,
    {
        Self::from_indicator_with_threshold(indicator, calculator, RECURSION_THRESHOLD)
    }

    /// 从现有 Indicator 构造，自定义阈值
    pub fn from_indicator_with_threshold<I>(
        indicator: Arc<I>,
        calculator: C,
        threshold: usize,
    ) -> Self
    where
        I: Indicator<Num = T, Output = T, Series = S>,
    {
        let wrapper = RecursiveCalcWrapper {
            inner: calculator,
            threshold,
        };
        let cached = CachedIndicator::new_from_indicator(indicator, wrapper);
        Self { cached }
    }

    pub fn get_value(&self, index: usize) -> Result<C::Output, IndicatorError> {
        let series_ref = self.cached.base.bar_series();

        series_ref.with_ref(|s| {
            if s.get_bar_count() == 0 || index > s.get_end_index().unwrap_or(usize::MAX) {
                // 超出范围，直接计算
                return self.cached.get_cached_value(index);
            }

            let removed = s.get_removed_bars_count();
            let highest = *self.cached.highest_result_index.borrow();

            let start = std::cmp::max(removed, if highest < 0 { 0 } else { highest as usize });

            if index > start && (index - start) > self.cached.calculator.threshold {
                // 迭代计算避免深递归
                for i in start..index {
                    self.cached.get_cached_value(i)?;
                }
            }

            self.cached.get_cached_value(index)
        })?
    }
}

impl<T, S, C> Indicator for RecursiveCachedIndicator<T, S, C>
where
    T: TrNum + Clone + 'static,
    S: BarSeries<T> + 'static,
    C: IndicatorCalculator<T, S, Output = T> + Clone,
{
    type Num = T;
    type Output = T;
    type Series = S;

    fn get_value(&self, index: usize) -> Result<C::Output, IndicatorError> {
        self.get_value(index)
    }

    fn bar_series(&self) -> BarSeriesRef<Self::Series> {
        self.cached.bar_series()
    }

    fn count_of_unstable_bars(&self) -> usize {
        0
    }
}
