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

/// 等价于 Java 的 EMAIndicator，封装标准 multiplier 的构造
pub struct EmaIndicator<'a, T, S, I>
where
    T: TrNum + Clone + 'static,
    S: for<'b> BarSeries<'b, T>,
    I: Indicator<Num = T, Output = T, Series<'a> = S> + 'a,
{
    inner: BaseEmaIndicator<'a, T, S, I>,
}

impl<'a, T, S, I> Clone for EmaIndicator<'a, T, S, I>
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

impl<'a, T, S, I> EmaIndicator<'a, T, S, I>
where
    T: TrNum + Clone + 'static,
    S: for<'b> BarSeries<'b, T>,
    I: Indicator<Num = T, Output = T, Series<'a> = S> + 'a,
{
    pub fn new(indicator: &'a I, bar_count: usize) -> Self {
        let num_factory = indicator.get_bar_series().num_factory();
        let multiplier: T = num_factory.num_of_f64(2.0 / (bar_count as f64 + 1.0));
        let inner = BaseEmaIndicator::new(indicator, bar_count, multiplier);
        Self { inner }
    }

    pub fn bar_count(&self) -> usize {
        self.inner.bar_count()
    }
}

impl<'a, T, S, I> Indicator for EmaIndicator<'a, T, S, I>
where
    T: TrNum + Clone + 'static,
    S: for<'b> BarSeries<'b, T>,
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
        self.bar_count()
    }
}
