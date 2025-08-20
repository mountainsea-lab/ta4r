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

use std::marker::PhantomData;
use std::sync::Arc;
use parking_lot::RwLock;
use crate::bar::types::{Bar, BarSeries};
use crate::indicators::Indicator;
use crate::indicators::types::IndicatorError;
use crate::num::TrNum;


pub struct BaseIndicator<T, S>
where
    T: TrNum + 'static,
    S: BarSeries<T>,
{
    series: Arc<RwLock<S>>,
    _marker: PhantomData<T>,
}

impl<T, S> Clone for BaseIndicator<T, S>
where
    T: TrNum + 'static,
    S: BarSeries<T> + 'static,
{
    fn clone(&self) -> Self {
        Self {
            series: self.series.clone(),
            _marker: PhantomData,
        }
    }
}

impl<T, S> BaseIndicator<T, S>
where
    T: TrNum + 'static,
    S: BarSeries<T> + 'static,
{
    pub fn new(series: Arc<RwLock<S>>) -> Self {
        Self {
            series,
            _marker: PhantomData,
        }
    }

    /// 指标在 index 是否稳定
    pub fn is_stable_at(&self, index: usize, unstable_count: usize) -> bool {
        index >= unstable_count
    }

    /// 整个指标是否稳定
    pub fn is_stable(&self, unstable_count: usize) -> bool {
        self.series.read().get_bar_count() >= unstable_count
    }
}

impl<T, S> Indicator for BaseIndicator<T, S>
where
    T: TrNum + Clone + 'static,
    S: BarSeries<T> + 'static,
{
    type Num = T;
    type Output = T;
    type Series = S;

    fn get_value(&self, index: usize) -> Result<T, IndicatorError> {
        let series = self.series.read();
        let bar = series
            .get_bar(index)
            .ok_or(IndicatorError::OutOfBounds { index })?;
        bar.get_close_price()
            .ok_or(IndicatorError::OutOfBounds { index })
    }

    fn bar_series(&self) -> Arc<RwLock<Self::Series>> {
        self.series.clone()
    }

    fn count_of_unstable_bars(&self) -> usize {
        0
    }
}
