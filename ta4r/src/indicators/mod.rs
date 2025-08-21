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
use crate::indicators::helpers::constant_indicator::ConstantIndicator;
use crate::indicators::types::{IndicatorError, IndicatorIterator, NumConst};
use crate::num::TrNum;
use crate::num::types::NumError;

pub mod abstract_indicator;
// mod atr_indicator;
pub mod averages;
pub mod cached_indicator;
pub mod helpers;
pub mod numeric;
pub mod recursive_cached_indicator;
pub mod types;
// pub mod atr_indicator;

pub trait Indicator: Clone {
    type Num: TrNum + 'static;
    type Output: Clone + 'static;
    type Series: BarSeries<Self::Num> + 'static;

    /// 获取指标值
    fn get_value(&self, index: usize) -> Result<Self::Output, IndicatorError>;

    /// 获取绑定的 BarSeriesRef
    fn bar_series(&self) -> BarSeriesRef<Self::Series>;

    /// 不稳定 bar 的数量
    fn count_of_unstable_bars(&self) -> usize;

    /// 判断 index 是否稳定
    fn is_stable_at(&self, index: usize) -> bool {
        index >= self.count_of_unstable_bars()
    }

    /// 当前 series 是否稳定
    fn is_stable(&self) -> bool {
        let series = self.bar_series();
        let bar_count = series.with_ref_or(0, |s| s.get_bar_count());
        bar_count >= self.count_of_unstable_bars()
    }

    /// 默认迭代器（实盘增量模式）
    fn iter(&self) -> IndicatorIterator<'_, Self, Self::Num, Self::Series>
    where
        Self: Clone,
        Self::Num: Clone + From<Self::Output>,
    {
        let series_ref = self.bar_series();
        IndicatorIterator::incremental(self, series_ref)
    }

    /// 快照迭代器（历史数据遍历，高性能）
    fn iter_snapshot(&self) -> IndicatorIterator<'_, Self, Self::Num, Self::Series>
    where
        Self: Clone,
        Self::Num: Clone + From<Self::Output>,
    {
        IndicatorIterator::snapshot(self)
    }
}

/// 转换为数字类型 trait定义
pub trait ToNumber<T>
where
    T: TrNum + Clone + 'static,
{
    fn to_number(&self, factory: &T::Factory) -> Result<T, NumError>;
}

/// 类型转换为指标统一约束 trait 主要作用数字类型自动转换为指标类型
pub trait IntoIndicator<T, S, I>
where
    T: TrNum + 'static,
    S: BarSeries<T>,
    I: Indicator<Num = T, Output = T> + Clone,
{
    type IndicatorType: Indicator<Num = T, Output = T> + Clone;

    /// 传入第一个指标，用于获取 BarSeries 以构造 ConstantIndicator
    fn as_indicator(&self, first: &I) -> Result<Self::IndicatorType, IndicatorError>;
}

/// 对数字常量NumConstant实现，通过第一个指标获取 BarSeries 构造 ConstantIndicator
impl<T, S, I, N> IntoIndicator<T, S, I> for NumConst<N>
where
    T: TrNum + Clone + 'static,
    S: BarSeries<T> + 'static,
    I: Indicator<Num = T, Output = T, Series = S> + Clone,
    N: ToNumber<T> + Clone,
{
    type IndicatorType = ConstantIndicator<T, S>;

    fn as_indicator(&self, first: &I) -> Result<Self::IndicatorType, IndicatorError> {
        first
            .bar_series()
            .with_ref(|series| {
                self.0
                    .to_number(series.factory_ref())
                    .map(|num| ConstantIndicator::new(first.bar_series(), num))
                    .map_err(IndicatorError::NumError)
            })
            .map_err(|e| IndicatorError::Other { message: e })?
    }
}

/// 对于已经是指标的，直接返回自己
impl<T, S, I> IntoIndicator<T, S, I> for I
where
    T: TrNum + 'static,
    S: BarSeries<T>,
    I: Indicator<Num = T, Output = T> + Clone,
{
    type IndicatorType = I;

    fn as_indicator(&self, _first: &I) -> Result<Self::IndicatorType, IndicatorError> {
        Ok(self.clone())
    }
}

pub trait OptionExt<T> {
    fn or_invalid_index(self, index: usize, max: usize) -> Result<T, IndicatorError>;
}
