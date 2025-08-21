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
use crate::indicators::Indicator;
use crate::indicators::abstract_indicator::BaseIndicator;
use crate::indicators::cached_indicator::CachedIndicator;
use crate::indicators::types::{IndicatorCalculator, IndicatorError};
use crate::num::{NumFactory, TrNum};
use parking_lot::RwLock;
use std::cell::RefCell;
use std::marker::PhantomData;
use std::sync::Arc;

/// 典型价格计算器：(high + low + close) / 3
pub struct TypicalPriceCalculator<T, S> {
    _phantom: PhantomData<(T, S)>,
}

impl<T, S> Clone for TypicalPriceCalculator<T, S> {
    fn clone(&self) -> Self {
        Self {
            _phantom: PhantomData,
        }
    }
}

impl<T, S> TypicalPriceCalculator<T, S> {
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData,
        }
    }
}

impl<'a, T, S> IndicatorCalculator<T, S> for TypicalPriceCalculator<T, S>
where
    T: TrNum + Clone + 'static,
    S: BarSeries<T> + 'static,
{
    type Output = T;

    fn calculate(
        &self,
        base: &BaseIndicator<T, S>,
        index: usize,
    ) -> Result<Self::Output, IndicatorError> {
        let series_ref = base.bar_series();

        series_ref.with_ref(|s| {
            let bar = s
                .get_bar(index)
                .ok_or_else(|| IndicatorError::OutOfBounds { index })?;

            let high = bar.get_high_price().ok_or_else(|| IndicatorError::Other {
                message: format!("Missing high price at index {}", index),
            })?;
            let low = bar.get_low_price().ok_or_else(|| IndicatorError::Other {
                message: format!("Missing low price at index {}", index),
            })?;
            let close = bar.get_close_price().ok_or_else(|| IndicatorError::Other {
                message: format!("Missing close price at index {}", index),
            })?;

            let sum = high.plus(&low).plus(&close);

            // 方案1：绑定临时值
            let three_value = s.num_factory().three();
            let three = three_value.as_ref();

            // 方案2：或者直接用已拥有所有权的 T
            // let three = series.num_factory().three_as_num();

            let result = sum.divided_by(three)?;

            Ok(result)
        })?
    }
}

pub struct TypicalPriceIndicator<T, S>
where
    T: TrNum + Clone + 'static,
    S: BarSeries<T> + 'static,
{
    cached: CachedIndicator<T, S, TypicalPriceCalculator<T, S>>,
}

impl<T, S> Clone for TypicalPriceIndicator<T, S>
where
    T: TrNum + Clone + 'static,
    S: BarSeries<T> + 'static,
{
    fn clone(&self) -> Self {
        Self {
            cached: self.cached.clone(),
        }
    }
}

impl<T, S> TypicalPriceIndicator<T, S>
where
    T: TrNum + Clone + 'static,
    S: BarSeries<T> + 'static,
{
    pub fn new(series_ref: BarSeriesRef<S>) -> Self {
        let calculator = TypicalPriceCalculator::new();
        let cached = CachedIndicator::new_from_series(series_ref, calculator);
        Self { cached }
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

impl<T, S> Indicator for TypicalPriceIndicator<T, S>
where
    T: TrNum + Clone + 'static,
    S: BarSeries<T> + 'static,
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
        0
    }
}
