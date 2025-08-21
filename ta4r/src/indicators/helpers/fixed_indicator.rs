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
use crate::indicators::types::IndicatorError;
use crate::num::TrNum;
use parking_lot::RwLock;
use std::cell::RefCell;
use std::sync::Arc;

pub struct FixedIndicator<T, S>
where
    T: TrNum + Clone + 'static,
    S: BarSeries<T> + 'static,
{
    base: BaseIndicator<T, S>,
    values: Vec<T>,
}

impl<'a, T, S> Clone for FixedIndicator<T, S>
where
    T: TrNum + Clone + 'static,
    S: BarSeries<T> + 'static,
{
    fn clone(&self) -> Self {
        Self {
            base: self.base.clone(),
            values: self.values.clone(), // Vec<T> 需要 T: Clone
        }
    }
}

impl<T, S> FixedIndicator<T, S>
where
    T: TrNum + Clone + 'static,
    S: BarSeries<T> + 'static,
{
    pub fn new(series_ref: BarSeriesRef<S>, values: Vec<T>) -> Self {
        Self {
            base: BaseIndicator::new(series_ref),
            values,
        }
    }
    /// 快捷方式：从 Arc<RwLock<S>> 构造
    pub fn from_shared(series: Arc<RwLock<S>>, values: Vec<T>) -> Self {
        Self::new(BarSeriesRef::Shared(series), values)
    }

    /// 快捷方式：从 Rc<RefCell<S>> 构造
    pub fn from_mut(series: Arc<RefCell<S>>, values: Vec<T>) -> Self {
        Self::new(BarSeriesRef::Mut(series), values)
    }

    pub fn add_value(&mut self, value: T) {
        self.values.push(value);
    }

    pub fn get_base(&self) -> &BaseIndicator<T, S> {
        &self.base
    }
}

impl<T, S> Indicator for FixedIndicator<T, S>
where
    T: TrNum + Clone + 'static,
    S: BarSeries<T> + 'static,
{
    type Num = T;
    type Output = T;

    type Series = S;

    fn get_value(&self, index: usize) -> Result<T, IndicatorError> {
        self.values
            .get(index)
            .cloned()
            .ok_or_else(|| IndicatorError::OutOfBounds { index })
    }

    fn bar_series(&self) -> BarSeriesRef<Self::Series> {
        self.base.bar_series()
    }

    fn count_of_unstable_bars(&self) -> usize {
        0
    }
}
