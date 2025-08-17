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
use crate::indicators::averages::base_ema_indicator::BaseEmaIndicator;
use crate::indicators::types::IndicatorError;
use crate::num::{NumFactory, TrNum};

/// Modified moving average indicator (MMA).
///
/// Similar to exponential moving average but smooths more slowly.
/// Used in Welles Wilder's indicators like ADX, RSI.
///
/// Formula: multiplier = 1 / bar_count
pub struct MMAIndicator<'a, T, S, I>
where
    T: TrNum + Clone + 'static,
    S: for<'any> BarSeries<'any, T>,
    I: Indicator<Num = T, Output = T, Series<'a> = S> + 'a,
{
    inner: BaseEmaIndicator<'a, T, S, I>,
}

impl<'a, T, S, I> Clone for MMAIndicator<'a, T, S, I>
where
    T: TrNum + Clone + 'static,
    S: for<'b> BarSeries<'b, T>,
    I: Indicator<Num = T, Output = T, Series<'a> = S> + 'a,
{
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<'a, T, S, I> MMAIndicator<'a, T, S, I>
where
    T: TrNum + Clone + 'static,
    S: for<'any> BarSeries<'any, T>,
    I: Indicator<Num = T, Output = T, Series<'a> = S> + 'a,
{
    /// Constructs a new MMAIndicator
    ///
    /// # Arguments
    /// * `indicator` - the base indicator
    /// * `bar_count` - the MMA time frame
    pub fn new(indicator: &'a I, bar_count: usize) -> Result<Self, IndicatorError> {
        let num_factory = indicator.get_bar_series().num_factory();

        let one = num_factory.one().as_ref().clone();

        let bar_count_t = num_factory.num_of_usize(bar_count).clone();

        let multiplier = one.divided_by(&bar_count_t)?;
        let inner = BaseEmaIndicator::new(indicator, bar_count, multiplier);

        Ok(Self { inner })
    }

    /// Returns the number of unstable bars, equal to the bar count
    pub fn get_count_of_unstable_bars(&self) -> usize {
        self.inner.bar_count()
    }
}

impl<'a, T, S, I> Indicator for MMAIndicator<'a, T, S, I>
where
    T: TrNum + Clone + 'static,
    S: for<'any> BarSeries<'any, T>,
    I: Indicator<Num = T, Output = T, Series<'a> = S> + 'a,
{
    type Num = T;
    type Output = T;
    type Series<'b>
        = S
    where
        Self: 'b;

    fn get_value(&self, index: usize) -> Result<T, IndicatorError> {
        self.inner.get_value(index)
    }

    fn get_bar_series(&self) -> &Self::Series<'_> {
        self.inner.get_bar_series()
    }

    fn get_count_of_unstable_bars(&self) -> usize {
        self.inner.get_count_of_unstable_bars()
    }
}
