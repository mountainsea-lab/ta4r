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
use crate::num::{NumFactory, TrNum};
use std::marker::PhantomData;
use std::sync::Arc;

/// GainCalculator 调用被包装的 indicator 计算 gain
pub struct GainCalculator<T, S, I>
where
    T: TrNum + Clone + 'static,
    S: BarSeries<T> + 'static,
    I: Indicator<Num = T, Output = T, Series = S>,
{
    indicator: Arc<I>,
    _phantom: PhantomData<(T, S)>,
}

impl<T, S, I> Clone for GainCalculator<T, S, I>
where
    T: TrNum + Clone + 'static,
    S: BarSeries<T> + 'static,
    I: Indicator<Num = T, Output = T, Series = S>,
{
    fn clone(&self) -> Self {
        Self {
            indicator: Arc::clone(&self.indicator),
            _phantom: PhantomData,
        }
    }
}

impl<T, S, I> GainCalculator<T, S, I>
where
    T: TrNum + Clone + 'static,
    S: BarSeries<T> + 'static,
    I: Indicator<Num = T, Output = T, Series = S>,
{
    pub fn new(indicator: Arc<I>) -> Self {
        Self {
            indicator,
            _phantom: PhantomData,
        }
    }
}

impl<T, S, I> IndicatorCalculator<T, S> for GainCalculator<T, S, I>
where
    T: TrNum + Clone + 'static,
    S: BarSeries<T> + 'static,
    I: Indicator<Num = T, Output = T, Series = S>,
{
    type Output = T;

    fn calculate(
        &self,
        base: &BaseIndicator<T, S>,
        index: usize,
    ) -> Result<Self::Output, IndicatorError> {
        // 通过 BarSeriesRef 获取 zero
        let zero = base.bar_series().with_ref_or(T::zero(), |series| {
            series.num_factory().zero().as_ref().clone()
        });

        if index == 0 {
            return Ok(zero);
        }

        let actual_value = self.indicator.get_value(index)?;
        let previous_value = self.indicator.get_value(index - 1)?;

        // 保持原来的计算逻辑
        if actual_value.is_greater_than(&previous_value) {
            Ok(actual_value.minus(&previous_value))
        } else {
            Ok(zero)
        }
    }
}

/// GainIndicator 组合 CachedIndicator，持有泛型 Indicator
pub struct GainIndicator<T, S, I>
where
    T: TrNum + Clone + 'static,
    S: BarSeries<T> + 'static,
    I: Indicator<Num = T, Output = T, Series = S>,
{
    cached: CachedIndicator<T, S, GainCalculator<T, S, I>>,
}

impl<T, S, I> Clone for GainIndicator<T, S, I>
where
    T: TrNum + Clone + 'static,
    S: BarSeries<T> + 'static,
    I: Indicator<Num = T, Output = T, Series = S>,
{
    fn clone(&self) -> Self {
        Self {
            cached: self.cached.clone(),
        }
    }
}

impl<T, S, I> GainIndicator<T, S, I>
where
    T: TrNum + Clone + 'static,
    S: BarSeries<T> + 'static,
    I: Indicator<Num = T, Output = T, Series = S>,
{
    /// 构造函数，传入被包装的 indicator 引用
    pub fn new(indicator: Arc<I>) -> Self {
        let calculator = GainCalculator::new(Arc::clone(&indicator));
        let cached = CachedIndicator::new_from_indicator(indicator, calculator);
        Self { cached }
    }
}

impl<T, S, I> Indicator for GainIndicator<T, S, I>
where
    T: TrNum + Clone + 'static,
    S: BarSeries<T> + 'static,
    I: Indicator<Num = T, Output = T, Series = S>,
{
    type Num = T;
    type Output = T;
    type Series = S;

    fn get_value(&self, index: usize) -> Result<T, IndicatorError> {
        self.cached.get_cached_value(index)
    }

    fn bar_series(&self) -> BarSeriesRef<Self::Series> {
        self.cached.bar_series()
    }

    fn count_of_unstable_bars(&self) -> usize {
        1
    }
}
