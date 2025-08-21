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

/// Volume计算器，实现 IndicatorCalculator trait
pub struct VolumeCalculator<T, S> {
    bar_count: usize,
    _phantom: PhantomData<(T, S)>,
}

impl<T, S> Clone for VolumeCalculator<T, S> {
    fn clone(&self) -> Self {
        Self {
            bar_count: self.bar_count,
            _phantom: PhantomData,
        }
    }
}

impl<T, S> VolumeCalculator<T, S> {
    pub fn new(bar_count: usize) -> Self {
        Self {
            bar_count: bar_count.max(1),
            _phantom: PhantomData,
        }
    }
}

impl<T, S> IndicatorCalculator<T, S> for VolumeCalculator<T, S>
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
        let start_index = index.saturating_sub(self.bar_count - 1);

        series_ref
            .with_ref(|s| {
                let factory_ref = s.factory_ref(); // 获取 NumFactory
                let mut sum = factory_ref.zero().as_ref().clone(); // sum 是 T 类型

                for i in start_index..=index {
                    let volume = s.get_bar(i).map_or_else(
                        || factory_ref.zero().as_ref().clone(),
                        |bar| bar.get_volume(),
                    );
                    sum = sum.plus(&volume);
                }

                sum // 返回 T，类型安全
            })
            .map_err(|e| IndicatorError::Other { message: e })
    }
}

/// 基于 VolumeCalculator 的 VolumeIndicator
pub struct VolumeIndicator<T, S>
where
    T: TrNum + Clone + 'static,
    S: BarSeries<T> + 'static,
{
    cached: CachedIndicator<T, S, VolumeCalculator<T, S>>,
}

impl<T, S> Clone for VolumeIndicator<T, S>
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

impl<T, S> VolumeIndicator<T, S>
where
    T: TrNum + Clone + 'static,
    S: BarSeries<T> + 'static,
{
    /// 默认构造器，bar_count = 1
    pub fn new(series: Arc<RwLock<S>>) -> Self {
        Self::from_shared_with_bar_count(series, 1)
    }

    /// 快捷方式：从 Arc<RwLock<S>> 构造
    pub fn from_shared_with_bar_count(series: Arc<RwLock<S>>, bar_count: usize) -> Self {
        let calculator = VolumeCalculator::new(bar_count);
        let cached = CachedIndicator::new_from_series(BarSeriesRef::Shared(series), calculator);
        Self { cached }
    }

    /// 快捷方式：从 Rc<RefCell<S>> 构造
    pub fn from_mut_with_bar_count(series: Arc<RefCell<S>>, bar_count: usize) -> Self {
        let calculator = VolumeCalculator::new(bar_count);
        let cached = CachedIndicator::new_from_series(BarSeriesRef::Mut(series), calculator);
        Self { cached }
    }
}

impl<T, S> Indicator for VolumeIndicator<T, S>
where
    T: TrNum + Clone + 'static,
    S: BarSeries<T> + 'static,
{
    type Num = T;
    type Output = T;
    type Series = S;

    #[inline]
    fn get_value(&self, index: usize) -> Result<T, IndicatorError> {
        self.cached.get_cached_value(index)
    }

    #[inline]
    fn bar_series(&self) -> BarSeriesRef<Self::Series> {
        self.cached.bar_series()
    }

    #[inline]
    fn count_of_unstable_bars(&self) -> usize {
        self.cached.calculator.bar_count
    }
}
