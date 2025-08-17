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
use crate::indicators::cached_indicator::CachedIndicator;
use crate::indicators::types::{IndicatorCalculator, IndicatorError};
use crate::num::TrNum;

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

impl<'a, T, S, C> IndicatorCalculator<'a, T, S> for RecursiveCalcWrapper<C>
where
    T: TrNum + Clone + 'static,
    S: BarSeries<'a, T>,
    C: IndicatorCalculator<'a, T, S> + Clone,
{
    type Output = C::Output;

    fn calculate(
        &self,
        base: &BaseIndicator<'a, T, S>,
        index: usize,
    ) -> Result<Self::Output, IndicatorError> {
        // 不负责递归预计算，直接调用内层计算器
        self.inner.calculate(base, index)
    }
}

pub struct RecursiveCachedIndicator<'a, T, S, C>
where
    T: TrNum + Clone + 'static,
    S: BarSeries<'a, T>,
    C: IndicatorCalculator<'a, T, S> + Clone,
{
    pub(crate) cached: CachedIndicator<'a, T, S, RecursiveCalcWrapper<C>>,
}

impl<'a, T, S, C> Clone for RecursiveCachedIndicator<'a, T, S, C>
where
    T: TrNum + Clone + 'static,
    S: BarSeries<'a, T>,
    C: IndicatorCalculator<'a, T, S> + Clone,
{
    fn clone(&self) -> Self {
        Self {
            cached: self.cached.clone(),
        }
    }
}

impl<'a, T, S, C> RecursiveCachedIndicator<'a, T, S, C>
where
    T: TrNum + Clone + 'static,
    S: BarSeries<'a, T>,
    C: IndicatorCalculator<'a, T, S, Output = T> + Clone,
{
    pub fn new(series: &'a S, calculator: C) -> Self {
        Self::new_with_threshold(series, calculator, RECURSION_THRESHOLD)
    }

    pub fn new_with_threshold(series: &'a S, calculator: C, threshold: usize) -> Self {
        let wrapper = RecursiveCalcWrapper {
            inner: calculator,
            threshold,
        };
        Self {
            cached: CachedIndicator::new_from_series(series, wrapper),
        }
    }

    /// 从现有 Indicator 构造，使用默认阈值
    pub fn from_indicator<I>(indicator: &'a I, calculator: C) -> Self
    where
        I: Indicator<Num = T, Output = T, Series<'a> = S>,
    {
        Self::from_indicator_with_threshold(indicator, calculator, RECURSION_THRESHOLD)
    }

    /// 从现有 Indicator 构造，自定义阈值
    pub fn from_indicator_with_threshold<I>(
        indicator: &'a I,
        calculator: C,
        threshold: usize,
    ) -> Self
    where
        I: Indicator<Num = T, Output = T, Series<'a> = S>,
    {
        let wrapper = RecursiveCalcWrapper {
            inner: calculator,
            threshold,
        };
        let cached = CachedIndicator::new_from_indicator(indicator, wrapper);
        Self { cached }
    }

    pub fn get_value(&self, index: usize) -> Result<C::Output, IndicatorError> {
        let series = self.cached.base.get_bar_series();

        if series.get_bar_count() == 0 || index > series.get_end_index().unwrap_or(usize::MAX) {
            // 超出范围，直接计算
            return self.cached.get_cached_value(index);
        }

        let removed = series.get_removed_bars_count();
        let highest = *self.cached.highest_result_index.borrow();

        let start = std::cmp::max(removed, if highest < 0 { 0 } else { highest as usize });

        if index > start && (index - start) > self.cached.calculator.threshold {
            // 迭代计算避免深递归
            for i in start..index {
                self.cached.get_cached_value(i)?;
            }
        }

        self.cached.get_cached_value(index)
    }
}

impl<'a, T, S, C> Indicator for RecursiveCachedIndicator<'a, T, S, C>
where
    T: TrNum + Clone + 'static,
    S: for<'any> BarSeries<'any, T>,
    C: IndicatorCalculator<'a, T, S, Output = T> + Clone,
{
    type Num = T;
    type Output = T;
    type Series<'b>
        = S
    where
        Self: 'b;

    fn get_value(&self, index: usize) -> Result<C::Output, IndicatorError> {
        self.get_value(index)
    }

    fn get_bar_series(&self) -> &Self::Series<'_> {
        self.cached.get_bar_series()
    }

    fn get_count_of_unstable_bars(&self) -> usize {
        0
    }
}
