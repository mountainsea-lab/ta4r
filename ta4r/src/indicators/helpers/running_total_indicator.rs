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
use crate::num::{NumFactory, TrNum};
use std::cell::{RefCell, RefMut};
use std::fmt;
use std::marker::PhantomData;
use std::sync::Arc;

pub struct RunningTotalCalculator<T, S, I>
where
    T: TrNum + Clone + 'static,
    S: BarSeries<T>,
    I: Indicator<Num = T, Output = T, Series = S>,
{
    indicator: Arc<I>,
    bar_count: usize,
    prev_index: RefCell<Option<usize>>,
    prev_sum: RefCell<T>,
    _phantom: PhantomData<(T, S)>,
}

impl<T, S, I> Clone for RunningTotalCalculator<T, S, I>
where
    T: TrNum + Clone + 'static,
    S: BarSeries<T>,
    I: Indicator<Num = T, Output = T, Series = S>,
{
    fn clone(&self) -> Self {
        Self {
            indicator: Arc::clone(&self.indicator),
            bar_count: self.bar_count,
            prev_index: RefCell::new(*self.prev_index.borrow()),
            prev_sum: RefCell::new(self.prev_sum.borrow().clone()),
            _phantom: PhantomData,
        }
    }
}

impl<T, S, I> fmt::Debug for RunningTotalCalculator<T, S, I>
where
    T: TrNum + Clone + fmt::Debug + 'static,
    S: BarSeries<T>,
    I: Indicator<Num = T, Output = T, Series = S>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RunningTotalCalculator")
            .field("bar_count", &self.bar_count)
            .field("prev_index", &self.prev_index.borrow())
            .field("prev_sum", &self.prev_sum.borrow())
            .finish()
    }
}

impl<T, S, I> RunningTotalCalculator<T, S, I>
where
    T: TrNum + Clone + 'static,
    S: BarSeries<T> + 'static,
    I: Indicator<Num = T, Output = T, Series = S>,
{
    pub fn new(indicator: Arc<I>, bar_count: usize) -> Self {
        let zero = indicator.bar_series().with_ref_or(T::zero(), |series| {
            series.num_factory().zero().as_ref().clone()
        });
        Self {
            indicator,
            bar_count,
            prev_index: RefCell::new(None),
            prev_sum: RefCell::new(zero),
            _phantom: PhantomData,
        }
    }

    /// 仅在 fast path 中调用：上一轮 sum + 当前 gain - 滑出值
    fn partial_sum(&self, index: usize, prev_sum: &T) -> Result<T, IndicatorError> {
        let mut sum = prev_sum.clone() + self.indicator.get_value(index)?;
        if index >= self.bar_count {
            let drop = self.indicator.get_value(index - self.bar_count)?;
            sum = sum - drop;
        }
        Ok(sum)
    }

    /// 更新快取状态
    fn update_partial_sum(
        &self,
        index: usize,
        new_sum: &T,
        prev_index: &mut RefMut<Option<usize>>,
        prev_sum: &mut RefMut<T>,
    ) {
        **prev_index = Some(index);
        **prev_sum = new_sum.clone();
    }
}

impl<T, S, I> IndicatorCalculator<T, S> for RunningTotalCalculator<T, S, I>
where
    T: TrNum + Clone + 'static,
    S: BarSeries<T> + 'static,
    I: Indicator<Num = T, Output = T, Series = S>,
{
    type Output = T;

    fn calculate(
        &self,
        _base: &BaseIndicator<T, S>,
        index: usize,
    ) -> Result<Self::Output, IndicatorError> {
        let mut prev_index = self.prev_index.borrow_mut();
        let mut prev_sum = self.prev_sum.borrow_mut();

        if let Some(last_index) = *prev_index {
            if index == last_index + 1 {
                // Fast path: reuse previous sum
                let sum = self.partial_sum(index, &prev_sum)?;
                self.update_partial_sum(index, &sum, &mut prev_index, &mut prev_sum);
                return Ok(sum);
            }
        }

        // Slow path: full recompute
        let zero = self
            .indicator
            .bar_series()
            .with_ref_or(T::zero(), |series| {
                series.num_factory().zero().as_ref().clone()
            });

        let start = index.saturating_sub(self.bar_count - 1);

        let mut sum = zero;
        for i in start..=index {
            sum = sum + self.indicator.get_value(i)?;
        }

        self.update_partial_sum(index, &sum, &mut prev_index, &mut prev_sum);
        Ok(sum)
    }
}

pub struct RunningTotalIndicator<T, S, I>
where
    T: TrNum + Clone + 'static,
    S: BarSeries<T> + 'static,
    I: Indicator<Num = T, Output = T, Series = S>,
{
    cached: CachedIndicator<T, S, RunningTotalCalculator<T, S, I>>,
}

impl<T, S, I> Clone for RunningTotalIndicator<T, S, I>
where
    T: TrNum + Clone + 'static,
    S: BarSeries<T> + 'static,
    I: Indicator<Num = T, Output = T, Series = S>,
{
    fn clone(&self) -> Self {
        Self {
            cached: self.cached.clone(),
        }
    }
}

impl<T, S, I> RunningTotalIndicator<T, S, I>
where
    T: TrNum + Clone + 'static,
    S: BarSeries<T> + 'static,
    I: Indicator<Num = T, Output = T, Series = S>,
{
    pub fn new(indicator: Arc<I>, bar_count: usize) -> Self {
        let calculator = RunningTotalCalculator::new(Arc::clone(&indicator), bar_count);
        let cached = CachedIndicator::new_from_indicator(indicator, calculator);
        Self { cached }
    }
}

impl<T, S, I> Indicator for RunningTotalIndicator<T, S, I>
where
    T: TrNum + Clone + 'static,
    S: BarSeries<T> + 'static,
    I: Indicator<Num = T, Output = T, Series = S>,
{
    type Num = T;
    type Output = T;
    type Series = S;

    fn get_value(&self, index: usize) -> Result<T, IndicatorError> {
        self.cached.get_cached_value(index)
    }

    fn bar_series(&self) -> BarSeriesRef<Self::Series> {
        self.cached.bar_series()
    }

    fn count_of_unstable_bars(&self) -> usize {
        self.cached.calculator().bar_count
    }
}

impl<T, S, I> fmt::Display for RunningTotalIndicator<T, S, I>
where
    T: TrNum + Clone + fmt::Debug + 'static,
    S: BarSeries<T> + 'static,
    I: Indicator<Num = T, Output = T, Series = S>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "RunningTotalIndicator barCount: {}",
            self.cached.calculator().bar_count
        )
    }
}

impl<T, S, I> fmt::Debug for RunningTotalIndicator<T, S, I>
where
    T: TrNum + Clone + fmt::Debug + 'static,
    S: BarSeries<T> + 'static,
    I: Indicator<Num = T, Output = T, Series = S>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RunningTotalIndicator")
            .field("bar_count", &self.cached.calculator().bar_count)
            .finish()
    }
}
