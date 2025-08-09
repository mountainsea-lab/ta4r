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
use crate::indicators::helpers::running_total_indicator::RunningTotalIndicator;
use crate::indicators::types::{IndicatorCalculator, IndicatorError};
use crate::num::{NumFactory, TrNum};
use std::marker::PhantomData;

pub struct SmaCalculator<'a, T, S, I>
where
    T: TrNum + Clone + 'static,
    S: for<'any> BarSeries<'any, T>,
    I: Indicator<Num = T, Series<'a> = S>,
{
    running_total: RunningTotalIndicator<'a, T, S, I>,
    bar_count: usize,
    _phantom: PhantomData<(T, S)>,
}

impl<'a, T, S, I> Clone for SmaCalculator<'a, T, S, I>
where
    T: TrNum + Clone + 'static,
    S: for<'any> BarSeries<'any, T>,
    I: Indicator<Num = T, Series<'a> = S>,
{
    fn clone(&self) -> Self {
        Self {
            running_total: self.running_total.clone(),
            bar_count: self.bar_count,
            _phantom: PhantomData,
        }
    }
}

impl<'a, T, S, I> SmaCalculator<'a, T, S, I>
where
    T: TrNum + Clone + 'static,
    S: for<'any> BarSeries<'any, T>,
    I: Indicator<Num = T, Series<'a> = S>,
{
    pub fn new(indicator: &'a I, bar_count: usize) -> Self {
        Self {
            running_total: RunningTotalIndicator::new(indicator, bar_count),
            bar_count,
            _phantom: PhantomData,
        }
    }
}

impl<'a, T, S, I> IndicatorCalculator<'a, T, S> for SmaCalculator<'a, T, S, I>
where
    T: TrNum + Clone + 'static,
    S: for<'any> BarSeries<'any, T>,
    I: Indicator<Num = T, Series<'a> = S>,
{
    fn calculate(
        &self,
        _base: &BaseIndicator<'a, T, S>,
        index: usize,
    ) -> Result<T, IndicatorError> {
        let real_bar_count = (index + 1).min(self.bar_count);

        let sum = self.running_total.get_value(index)?;

        let denom = self
            .running_total
            .get_bar_series()
            .num_factory()
            .num_of_i64(real_bar_count as i64);
        sum.divided_by(&denom).map_err(IndicatorError::NumError)
    }
}

pub struct SmaIndicator<'a, T, S, I>
where
    T: TrNum + Clone + 'static,
    S: for<'any> BarSeries<'any, T>,
    I: Indicator<Num = T, Series<'a> = S>,
{
    cached: CachedIndicator<'a, T, S, SmaCalculator<'a, T, S, I>>,
}

impl<'a, T, S, I> Clone for SmaIndicator<'a, T, S, I>
where
    T: TrNum + Clone + 'static,
    S: for<'any> BarSeries<'any, T>,
    I: Indicator<Num = T, Series<'a> = S>,
{
    fn clone(&self) -> Self {
        Self {
            cached: self.cached.clone(),
        }
    }
}

impl<'a, T, S, I> SmaIndicator<'a, T, S, I>
where
    T: TrNum + Clone + 'static,
    S: for<'any> BarSeries<'any, T>,
    I: Indicator<Num = T, Series<'a> = S>,
{
    pub fn new(indicator: &'a I, bar_count: usize) -> Self {
        let calculator = SmaCalculator::new(indicator, bar_count);
        let cached = CachedIndicator::new_from_indicator(indicator, calculator);
        Self { cached }
    }
}

impl<'a, T, S, I> Indicator for SmaIndicator<'a, T, S, I>
where
    T: TrNum + Clone + 'static,
    S: for<'any> BarSeries<'any, T>,
    I: Indicator<Num = T, Series<'a> = S>,
{
    type Num = T;
    type Series<'b>
        = S
    where
        Self: 'b;

    fn get_value(&self, index: usize) -> Result<T, IndicatorError> {
        self.cached.get_cached_value(index)
    }

    fn get_bar_series(&self) -> &Self::Series<'_> {
        self.cached.get_bar_series()
    }

    fn get_count_of_unstable_bars(&self) -> usize {
        self.cached.calculator().bar_count
    }
}

impl<'a, T, S, I> std::fmt::Display for SmaIndicator<'a, T, S, I>
where
    T: TrNum + Clone + 'static,
    S: for<'any> BarSeries<'any, T> + std::fmt::Debug,
    I: Indicator<Num = T, Series<'a> = S>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SMA(bar_count={})", self.get_count_of_unstable_bars())
    }
}
