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
use crate::indicators::recursive_cached_indicator::RecursiveCachedIndicator;
use crate::indicators::types::{IndicatorCalculator, IndicatorError};
use crate::num::TrNum;
use std::marker::PhantomData;

/// BaseEmaCalculator 持有对 indicator 的引用
pub struct BaseEmaCalculator<'a, T, S, I>
where
    T: TrNum + Clone + 'static,
    S: for<'any> BarSeries<'any, T>,
    I: Indicator<Num = T, Output = T, Series<'a> = S> + 'a,
{
    pub(crate) indicator: &'a I,
    pub(crate) multiplier: T,
    pub(crate) _phantom: PhantomData<&'a S>,
}

impl<'a, T, S, I> Clone for BaseEmaCalculator<'a, T, S, I>
where
    T: TrNum + Clone + 'static,
    S: for<'any> BarSeries<'any, T>,
    I: Indicator<Num = T, Output = T, Series<'a> = S> + 'a,
{
    fn clone(&self) -> Self {
        BaseEmaCalculator {
            indicator: self.indicator, // 复制引用即可
            multiplier: self.multiplier.clone(),
            _phantom: PhantomData,
        }
    }
}

impl<'a, T, S, I> IndicatorCalculator<'a, T, S> for BaseEmaCalculator<'a, T, S, I>
where
    T: TrNum + Clone + 'static,
    S: for<'any> BarSeries<'any, T>,
    I: Indicator<Num = T, Output = T, Series<'a> = S> + 'a,
{
    type Output = T;

    fn calculate(
        &self,
        base: &BaseIndicator<'a, T, S>,
        index: usize,
    ) -> Result<Self::Output, IndicatorError> {
        if index == 0 {
            return self.indicator.get_value(0);
        }

        let prev = base.get_value(index - 1)?;
        let current = self.indicator.get_value(index)?;
        let diff = current.clone() - prev.clone();
        Ok(diff * self.multiplier.clone() + prev)
    }
}

/// BaseEmaIndicator 也持有 indicator 的引用
pub struct BaseEmaIndicator<'a, T, S, I>
where
    T: TrNum + Clone + 'static,
    S: for<'any> BarSeries<'any, T>,
    I: Indicator<Num = T, Output = T, Series<'a> = S> + 'a,
{
    pub(crate) indicator: &'a I,
    pub(crate) bar_count: usize,
    pub(crate) multiplier: T,
    pub(crate) inner: RecursiveCachedIndicator<'a, T, S, BaseEmaCalculator<'a, T, S, I>>,
}

impl<'a, T, S, I> Clone for BaseEmaIndicator<'a, T, S, I>
where
    T: TrNum + Clone + 'static,
    S: for<'any> BarSeries<'any, T>,
    I: Indicator<Num = T, Output = T, Series<'a> = S> + 'a,
{
    fn clone(&self) -> Self {
        BaseEmaIndicator {
            indicator: self.indicator, // 复制引用
            bar_count: self.bar_count,
            multiplier: self.multiplier.clone(),
            inner: self.inner.clone(),
        }
    }
}

impl<'a, T, S, I> BaseEmaIndicator<'a, T, S, I>
where
    T: TrNum + Clone + 'static,
    S: for<'any> BarSeries<'any, T>,
    I: Indicator<Num = T, Output = T, Series<'a> = S> + 'a,
{
    /// 标准构造器：传入 T 类型 multiplier（等价 Java 中 Num 类型）
    pub fn new(indicator: &'a I, bar_count: usize, multiplier: T) -> Self {
        let calculator = BaseEmaCalculator {
            indicator,
            multiplier: multiplier.clone(),
            _phantom: PhantomData,
        };

        let inner = RecursiveCachedIndicator::from_indicator(indicator, calculator);

        Self {
            indicator,
            bar_count,
            multiplier,
            inner,
        }
    }

    pub fn bar_count(&self) -> usize {
        self.bar_count
    }
}

impl<'a, T, S, I> Indicator for BaseEmaIndicator<'a, T, S, I>
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
        self.indicator.get_bar_series()
    }

    fn get_count_of_unstable_bars(&self) -> usize {
        self.bar_count()
    }
}
