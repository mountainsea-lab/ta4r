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
use crate::bar::types::{Bar, BarSeries};
use crate::indicators::abstract_indicator::BaseIndicator;
use crate::indicators::types::IndicatorError;
use crate::indicators::{Indicator, OptionExt};
use crate::num::TrNum;

/// An indicator that returns the close price of each bar.
pub struct ClosePriceIndicator<'a, T, S>
where
    T: TrNum + 'static,
    S: BarSeries<'a, T>,
{
    base: BaseIndicator<'a, T, S>,
}

impl<'a, T, S> Clone for ClosePriceIndicator<'a, T, S>
where
    T: TrNum + 'static,
    S: BarSeries<'a, T>,
{
    fn clone(&self) -> Self {
        Self {
            base: self.base.clone(),
        }
    }
}

impl<'a, T, S> ClosePriceIndicator<'a, T, S>
where
    T: TrNum + 'static,
    S: BarSeries<'a, T>,
{
    /// Creates a new close price indicator based on the given bar series.
    pub fn new(series: &'a S) -> Self {
        Self {
            base: BaseIndicator::new(series),
        }
    }
}

impl<'a, T, S> Indicator for ClosePriceIndicator<'a, T, S>
where
    T: TrNum + 'static,
    S: for<'any> BarSeries<'any, T>,
{
    type Num = T;
    type Output = T;
    type Series<'b>
        = S
    where
        Self: 'b;

    #[inline]
    fn get_value(&self, index: usize) -> Result<Self::Num, IndicatorError> {
        let series = self.base.get_bar_series();
        let max = series.get_bar_count().saturating_sub(1);
        series
            .get_bar(index)
            .or_invalid_index(index, max)?
            .get_close_price()
            .ok_or_else(|| IndicatorError::CalculationError {
                message: "Missing close price".to_string(),
            })
    }

    #[inline]
    fn get_bar_series(&self) -> &Self::Series<'_> {
        self.base.get_bar_series()
    }

    #[inline]
    fn get_count_of_unstable_bars(&self) -> usize {
        0
    }
}
