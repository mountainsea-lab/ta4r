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
use crate::num::TrNum;
use parking_lot::RwLock;
use std::cell::RefCell;
use std::marker::PhantomData;
use std::sync::Arc;
use time::OffsetDateTime;

// ------------------- Calculator -------------------
pub struct DateTimeCalculator<S, F> {
    action: F,
    _phantom: PhantomData<S>,
}

impl<S, F: Copy> Clone for DateTimeCalculator<S, F> {
    fn clone(&self) -> Self {
        Self {
            action: self.action,
            _phantom: PhantomData,
        }
    }
}

impl<S, F> DateTimeCalculator<S, F> {
    pub fn new(action: F) -> Self {
        Self {
            action,
            _phantom: PhantomData,
        }
    }
}

// 默认函数：返回 Bar 的 begin_time
pub fn default_get_begin_time<T: TrNum + 'static, B: Bar<T>>(bar: &B) -> OffsetDateTime {
    bar.get_begin_time()
}

impl<T, S, F> IndicatorCalculator<T, S> for DateTimeCalculator<S, F>
where
    T: TrNum + Clone + 'static,
    S: BarSeries<T> + 'static,
    F: Fn(&<S as BarSeries<T>>::Bar) -> OffsetDateTime + Copy,
{
    type Output = OffsetDateTime;

    fn calculate(
        &self,
        base: &BaseIndicator<T, S>,
        index: usize,
    ) -> Result<Self::Output, IndicatorError> {
        let series_ref = base.bar_series();

        series_ref
            .with_ref(|s| match s.get_bar(index) {
                Some(bar) => Ok((self.action)(bar)),
                None => Err(IndicatorError::OutOfBounds { index }),
            })
            .map_err(|e| IndicatorError::Other { message: e })?
    }
}

// ------------------- Indicator -------------------
pub struct DateTimeIndicator<T, S, F>
where
    T: TrNum + Clone + 'static,
    S: BarSeries<T> + 'static,
    F: Fn(&<S as BarSeries<T>>::Bar) -> OffsetDateTime + Copy,
{
    cached: CachedIndicator<T, S, DateTimeCalculator<S, F>>,
}

impl<T, S, F> Clone for DateTimeIndicator<T, S, F>
where
    T: TrNum + Clone + 'static,
    S: BarSeries<T> + 'static,
    F: Fn(&<S as BarSeries<T>>::Bar) -> OffsetDateTime + Copy,
    CachedIndicator<T, S, DateTimeCalculator<S, F>>: Clone,
{
    fn clone(&self) -> Self {
        Self {
            cached: self.cached.clone(),
        }
    }
}

impl<T, S, F> DateTimeIndicator<T, S, F>
where
    T: TrNum + Clone + 'static,
    S: BarSeries<T> + 'static,
    F: Fn(&<S as BarSeries<T>>::Bar) -> OffsetDateTime + Copy,
{
    /// General construction Creates DateTimeIndicator indicator based on the given bar series and function.
    pub fn with_func(series: BarSeriesRef<S>, f: F) -> Self {
        let calculator = DateTimeCalculator::new(f);
        let cached = CachedIndicator::new_from_series(series, calculator);
        Self { cached }
    }
    /// 快捷方式：从 Arc<RwLock<S>> 构造
    pub fn from_shared(series: Arc<RwLock<S>>, f: F) -> Self {
        let calculator = DateTimeCalculator::new(f);
        let cached = CachedIndicator::new_from_series(BarSeriesRef::Shared(series), calculator);
        Self { cached }
    }

    /// 快捷方式：从 Rc<RefCell<S>> 构造
    pub fn from_mut(series: Arc<RefCell<S>>, f: F) -> Self {
        let calculator = DateTimeCalculator::new(f);
        let cached = CachedIndicator::new_from_series(BarSeriesRef::Mut(series), calculator);
        Self { cached }
    }
}

impl<T, S> DateTimeIndicator<T, S, fn(&<S as BarSeries<T>>::Bar) -> OffsetDateTime>
where
    T: TrNum + Clone + 'static,
    S: BarSeries<T> + 'static,
{
    /// 默认构造函数
    pub fn new(series: BarSeriesRef<S>) -> Self {
        Self::with_func(
            series,
            default_get_begin_time::<T, <S as BarSeries<T>>::Bar>,
        )
    }
}

impl<T, S, F> Indicator for DateTimeIndicator<T, S, F>
where
    T: TrNum + Clone + 'static,
    S: BarSeries<T> + 'static,
    F: Fn(&<S as BarSeries<T>>::Bar) -> OffsetDateTime + Copy,
{
    type Num = T;
    type Output = OffsetDateTime;
    type Series = S;

    fn get_value(&self, index: usize) -> Result<Self::Output, IndicatorError> {
        self.cached.get_cached_value(index)
    }

    fn bar_series(&self) -> BarSeriesRef<Self::Series> {
        self.cached.bar_series()
    }

    fn count_of_unstable_bars(&self) -> usize {
        0
    }
}
