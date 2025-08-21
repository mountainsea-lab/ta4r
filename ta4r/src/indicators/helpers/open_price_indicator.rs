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
use crate::bar::types::{Bar, BarSeries};
use crate::indicators::abstract_indicator::BaseIndicator;
use crate::indicators::types::IndicatorError;
use crate::indicators::{Indicator, OptionExt};
use crate::num::TrNum;
use parking_lot::RwLock;
use std::cell::RefCell;
use std::sync::Arc;

/// An indicator that returns the open price of each bar.
pub struct OpenPriceIndicator<T, S>
where
    T: TrNum + 'static,
    S: BarSeries<T> + 'static,
{
    base: BaseIndicator<T, S>,
}

impl<T, S> Clone for OpenPriceIndicator<T, S>
where
    T: TrNum + 'static,
    S: BarSeries<T> + 'static,
{
    fn clone(&self) -> Self {
        Self {
            base: self.base.clone(),
        }
    }
}

impl<T, S> OpenPriceIndicator<T, S>
where
    T: TrNum + 'static,
    S: BarSeries<T> + 'static,
{
    ///  General construction Creates a new open price indicator based on the given bar series.
    pub fn new(series_ref: BarSeriesRef<S>) -> Self {
        Self {
            base: BaseIndicator::new(series_ref),
        }
    }
    /// 快捷方式：从 Arc<RwLock<S>> 构造
    pub fn from_shared(series: Arc<RwLock<S>>) -> Self {
        Self::new(BarSeriesRef::Shared(series))
    }

    /// 快捷方式：从 Rc<RefCell<S>> 构造
    pub fn from_mut(series: Arc<RefCell<S>>) -> Self {
        Self::new(BarSeriesRef::Mut(series))
    }
}

impl<T, S> Indicator for OpenPriceIndicator<T, S>
where
    T: TrNum + 'static,
    S: BarSeries<T> + 'static,
{
    type Num = T;
    type Output = T;
    type Series = S;

    #[inline]
    fn get_value(&self, index: usize) -> Result<Self::Num, IndicatorError> {
        self.base.bar_series().with_ref(|s| {
            let max = s.get_bar_count().saturating_sub(1);
            let bar = s.get_bar(index).or_invalid_index(index, max)?;
            bar.get_open_price()
                .ok_or_else(|| IndicatorError::CalculationError {
                    message: "Missing open price".to_string(),
                })
        })?
    }

    #[inline]
    fn bar_series(&self) -> BarSeriesRef<Self::Series> {
        self.base.bar_series()
    }

    #[inline]
    fn count_of_unstable_bars(&self) -> usize {
        0
    }
}
