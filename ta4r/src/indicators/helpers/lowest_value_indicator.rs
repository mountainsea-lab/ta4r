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
use std::marker::PhantomData;
use std::sync::Arc;

pub struct LowestValueCalculator<T, S, I>
where
    T: TrNum + Clone + 'static,
    S: BarSeries<T> + 'static,
    I: Indicator<Num = T, Output = T, Series = S>,
{
    indicator: Arc<I>,
    bar_count: usize,
    _phantom: PhantomData<S>,
}

impl<T, S, I> Clone for LowestValueCalculator<T, S, I>
where
    T: TrNum + Clone + 'static,
    S: BarSeries<T> + 'static,
    I: Indicator<Num = T, Output = T, Series = S>,
{
    fn clone(&self) -> Self {
        Self {
            indicator: Arc::clone(&self.indicator),
            bar_count: self.bar_count,
            _phantom: PhantomData,
        }
    }
}

impl<T, S, I> IndicatorCalculator<T, S> for LowestValueCalculator<T, S, I>
where
    T: TrNum + Clone + 'static,
    S: BarSeries<T> + 'static,
    I: Indicator<Num = T, Output = T, Series = S>,
{
    type Output = T;

    fn calculate(
        &self,
        base: &BaseIndicator<T, S>,
        index: usize,
    ) -> Result<Self::Output, IndicatorError> {
        let value = self.indicator.get_value(index)?;

        if value.is_nan() && self.bar_count > 1 {
            let tmp = LowestValueCalculator {
                indicator: Arc::clone(&self.indicator),
                bar_count: self.bar_count - 1,
                _phantom: PhantomData,
            };
            // 与 Java 一致：递归调用 barCount-1，并且 index-1
            return tmp.calculate(base, index - 1);
        }

        let end = index.saturating_sub(self.bar_count - 1); // Math.max(0, index - barCount + 1)
        let mut lowest = value;

        for i in (end..index).rev() {
            let v = self.indicator.get_value(i)?;
            if lowest.is_greater_than(&v) {
                lowest = v;
            }
        }

        Ok(lowest)
    }
}

pub struct LowestValueIndicator<T, S, I>
where
    T: TrNum + Clone + 'static,
    S: BarSeries<T> + 'static,
    I: Indicator<Num = T, Output = T, Series = S>,
{
    cached: CachedIndicator<T, S, LowestValueCalculator<T, S, I>>,
    bar_count: usize,
}

impl<T, S, I> LowestValueIndicator<T, S, I>
where
    T: TrNum + Clone + 'static,
    S: BarSeries<T> + 'static,
    I: Indicator<Num = T, Output = T, Series = S>,
{
    pub fn new(indicator: Arc<I>, bar_count: usize) -> Self {
        let calculator = LowestValueCalculator {
            indicator: Arc::clone(&indicator),
            bar_count,
            _phantom: PhantomData,
        };
        Self {
            cached: CachedIndicator::new_from_indicator(indicator, calculator),
            bar_count,
        }
    }
}

impl<T, S, I> Clone for LowestValueIndicator<T, S, I>
where
    T: TrNum + Clone + 'static,
    S: BarSeries<T> + 'static,
    I: Indicator<Num = T, Output = T, Series = S>,
{
    fn clone(&self) -> Self {
        Self {
            cached: self.cached.clone(),
            bar_count: self.bar_count,
        }
    }
}

impl<T, S, I> Indicator for LowestValueIndicator<T, S, I>
where
    T: TrNum + Clone + 'static,
    S: BarSeries<T> + 'static,
    I: Indicator<Num = T, Output = T, Series = S>,
{
    type Num = T;
    type Output = T;
    type Series = S;

    fn get_value(&self, index: usize) -> Result<Self::Output, IndicatorError> {
        self.cached.get_cached_value(index)
    }

    fn bar_series(&self) -> BarSeriesRef<Self::Series> {
        self.cached.bar_series()
    }

    fn count_of_unstable_bars(&self) -> usize {
        self.bar_count
    }
}
