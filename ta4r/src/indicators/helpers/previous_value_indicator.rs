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
use crate::indicators::types::{IndicatorCalculator, IndicatorError};
use crate::num::TrNum;
use std::marker::PhantomData;

pub struct PreviousValueCalculator<'a, T, S, I>
where
    T: TrNum + Clone + 'static,
    S: for<'any> BarSeries<'any, T>,
    I: Indicator<Num = T, Output = T, Series<'a> = S>,
{
    n: usize,
    indicator: &'a I,
    _phantom: PhantomData<S>,
}

impl<'a, T, S, I> Clone for PreviousValueCalculator<'a, T, S, I>
where
    T: TrNum + Clone + 'static,
    S: for<'any> BarSeries<'any, T>,
    I: Indicator<Num = T, Output = T, Series<'a> = S>,
{
    fn clone(&self) -> Self {
        Self {
            n: self.n,
            indicator: self.indicator, // 引用直接复制
            _phantom: PhantomData,
        }
    }
}

// 不实现 Clone
impl<'a, T, S, I> PreviousValueCalculator<'a, T, S, I>
where
    T: TrNum + Clone + 'static,
    S: for<'any> BarSeries<'any, T>,
    I: Indicator<Num = T, Output = T, Series<'a> = S>,
{
    pub fn new(indicator: &'a I, n: usize) -> Self {
        if n < 1 {
            panic!("n must be positive, but was: {}", n);
        }
        Self {
            n,
            indicator,
            _phantom: PhantomData,
        }
    }
}

impl<'a, T, S, I> IndicatorCalculator<'a, T, S> for PreviousValueCalculator<'a, T, S, I>
where
    T: TrNum + Clone + 'static,
    S: for<'any> BarSeries<'any, T>,
    I: Indicator<Num = T, Output = T, Series<'a> = S>,
{
    type Output = T;

    fn calculate(
        &self,
        _base: &BaseIndicator<'a, T, S>,
        index: usize,
    ) -> Result<Self::Output, IndicatorError> {
        if index < self.n {
            Ok(T::nan())
        } else {
            self.indicator.get_value(index - self.n)
        }
    }
}

// ------------------- PreviousValueIndicator -------------------
pub struct PreviousValueIndicator<'a, T, S, I>
where
    T: TrNum + Clone + 'static,
    S: for<'any> BarSeries<'any, T>,
    I: Indicator<Num = T, Output = T, Series<'a> = S>,
{
    cached: CachedIndicator<'a, T, S, PreviousValueCalculator<'a, T, S, I>>,
    n: usize,
}

impl<'a, T, S, I> Clone for PreviousValueIndicator<'a, T, S, I>
where
    T: TrNum + Clone + 'static,
    S: for<'b> BarSeries<'b, T>,
    I: Indicator<Num = T, Output = T, Series<'a> = S> + Clone,
    CachedIndicator<'a, T, S, PreviousValueCalculator<'a, T, S, I>>: Clone,
{
    fn clone(&self) -> Self {
        Self {
            cached: self.cached.clone(), // 手动 clone 内部缓存和计算器
            n: self.n,                   // usize 直接复制
        }
    }
}

// 不实现 Clone
impl<'a, T, S, I> PreviousValueIndicator<'a, T, S, I>
where
    T: TrNum + Clone + 'static,
    S: for<'any> BarSeries<'any, T>,
    I: Indicator<Num = T, Output = T, Series<'a> = S>,
{
    pub fn new(indicator: &'a I) -> Self {
        Self::with_n(indicator, 1)
    }

    pub fn with_n(indicator: &'a I, n: usize) -> Self {
        let calculator = PreviousValueCalculator::new(indicator, n);
        let cached = CachedIndicator::new_from_indicator(indicator, calculator);
        Self { cached, n }
    }

    pub fn get_n(&self) -> usize {
        self.n
    }
}

impl<'a, T, S, I> Indicator for PreviousValueIndicator<'a, T, S, I>
where
    T: TrNum + Clone + 'static,
    S: for<'any> BarSeries<'any, T>,
    I: Indicator<Num = T, Output = T, Series<'a> = S>,
{
    type Num = T;
    type Output = T;
    type Series<'b>
        = S
    where
        Self: 'b;
    fn get_value(&self, index: usize) -> Result<Self::Output, IndicatorError> {
        self.cached.get_cached_value(index)
    }

    fn get_bar_series(&self) -> &Self::Series<'_> {
        self.cached.get_bar_series()
    }

    fn get_count_of_unstable_bars(&self) -> usize {
        self.n
    }
}
