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

pub struct PreviousValueCalculator<T, S, I>
where
    T: TrNum + Clone + 'static,
    S: BarSeries<T> + 'static,
    I: Indicator<Num = T, Output = T, Series = S>,
{
    n: usize,
    indicator: Arc<I>,
    _phantom: PhantomData<S>,
}

impl<T, S, I> Clone for PreviousValueCalculator<T, S, I>
where
    T: TrNum + Clone + 'static,
    S: BarSeries<T> + 'static,
    I: Indicator<Num = T, Output = T, Series = S>,
{
    fn clone(&self) -> Self {
        Self {
            n: self.n,
            indicator: Arc::clone(&self.indicator), // 引用直接复制
            _phantom: PhantomData,
        }
    }
}

// 不实现 Clone
impl<T, S, I> PreviousValueCalculator<T, S, I>
where
    T: TrNum + Clone + 'static,
    S: BarSeries<T> + 'static,
    I: Indicator<Num = T, Output = T, Series = S>,
{
    pub fn new(indicator: Arc<I>, n: usize) -> Self {
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

impl<T, S, I> IndicatorCalculator<T, S> for PreviousValueCalculator<T, S, I>
where
    T: TrNum + Clone + 'static,
    S: BarSeries<T> + 'static,
    I: Indicator<Num = T, Output = T, Series = S>,
{
    type Output = T;

    fn calculate(
        &self,
        _base: &BaseIndicator<T, S>,
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
pub struct PreviousValueIndicator<T, S, I>
where
    T: TrNum + Clone + 'static,
    S: BarSeries<T> + 'static,
    I: Indicator<Num = T, Output = T, Series = S>,
{
    cached: CachedIndicator<T, S, PreviousValueCalculator<T, S, I>>,
    n: usize,
}

impl<T, S, I> Clone for PreviousValueIndicator<T, S, I>
where
    T: TrNum + Clone + 'static,
    S: BarSeries<T> + 'static,
    I: Indicator<Num = T, Output = T, Series = S> + Clone,
    CachedIndicator<T, S, PreviousValueCalculator<T, S, I>>: Clone,
{
    fn clone(&self) -> Self {
        Self {
            cached: self.cached.clone(), // 手动 clone 内部缓存和计算器
            n: self.n,                   // usize 直接复制
        }
    }
}

// 不实现 Clone
impl<T, S, I> PreviousValueIndicator<T, S, I>
where
    T: TrNum + Clone + 'static,
    S: BarSeries<T> + 'static,
    I: Indicator<Num = T, Output = T, Series = S>,
{
    pub fn new(indicator: Arc<I>) -> Self {
        Self::with_n(indicator, 1)
    }

    pub fn with_n(indicator: Arc<I>, n: usize) -> Self {
        let calculator = PreviousValueCalculator::new(Arc::clone(&indicator), n);
        let cached = CachedIndicator::new_from_indicator(indicator, calculator);
        Self { cached, n }
    }

    pub fn get_n(&self) -> usize {
        self.n
    }
}

impl<T, S, I> Indicator for PreviousValueIndicator<T, S, I>
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
        self.n
    }
}
