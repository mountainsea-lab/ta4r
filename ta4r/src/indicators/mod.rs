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
use crate::indicators::helpers::constant_indicator::ConstantIndicator;
use crate::indicators::types::{IndicatorError, IndicatorIterator, NumConst};
use crate::num::TrNum;
use crate::num::types::NumError;

pub mod abstract_indicator;
pub mod averages;
pub mod cached_indicator;
pub mod helpers;
pub mod numeric;
mod recursive_cached_indicator;
pub mod types;

pub trait Indicator: Clone {
    type Num: TrNum + 'static;

    /// GAT 返回绑定生命周期的系列
    type Series<'a>: BarSeries<'a, Self::Num>
    where
        Self: 'a;

    /// 获取指定 index 处的指标值
    fn get_value(&self, index: usize) -> Result<Self::Num, IndicatorError>;

    /// 返回该指标依赖的 BarSeries 引用
    fn get_bar_series(&self) -> &Self::Series<'_>;

    /// 返回在多少根 bar 之前该指标是不稳定的（计算值不可靠）
    fn get_count_of_unstable_bars(&self) -> usize;

    /// 判断 index 处是否稳定
    fn is_stable_at(&self, index: usize) -> bool {
        index >= self.get_count_of_unstable_bars()
    }

    /// 当前 series 是否已达到稳定计算条件
    fn is_stable(&self) -> bool {
        self.get_bar_series().get_bar_count() >= self.get_count_of_unstable_bars()
    }

    /// 提供一个便捷迭代器（模拟 Java 的 stream()）
    fn iter(&self) -> IndicatorIterator<Self>
    where
        Self: Sized,
    {
        match (
            self.get_bar_series().get_begin_index(),
            self.get_bar_series().get_end_index(),
        ) {
            (Some(begin), Some(end)) => IndicatorIterator {
                indicator: self,
                index: begin,
                end,
            },
            _ => IndicatorIterator {
                indicator: self,
                index: 1, // 空迭代器起始大于结束
                end: 0,
            },
        }
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
pub trait IntoIndicator<'a, T, S, I>
where
    T: TrNum + 'static,
    S: for<'any> BarSeries<'any, T> + 'a,
    I: Indicator<Num = T> + Clone + 'a,
{
    type IndicatorType: Indicator<Num = T> + Clone + 'a;

    /// 传入第一个指标，用于获取 BarSeries 以构造 ConstantIndicator
    fn as_indicator(&self, first: &'a I) -> Result<Self::IndicatorType, IndicatorError>;
}

/// 对数字常量NumConstant实现，通过第一个指标获取 BarSeries 构造 ConstantIndicator
impl<'a, T, S, I, N> IntoIndicator<'a, T, S, I> for NumConst<N>
where
    T: TrNum + Clone + 'static,
    S: for<'any> BarSeries<'any, T> + 'a,
    I: Indicator<Num = T, Series<'a> = S> + Clone + 'a,
    N: ToNumber<T> + Clone,
{
    type IndicatorType = ConstantIndicator<'a, T, S>;

    fn as_indicator(&self, first: &'a I) -> Result<Self::IndicatorType, IndicatorError> {
        let series = first.get_bar_series(); // &S
        let factory_ref: &T::Factory = series.factory_ref(); // 封装了解 deref

        let value = self
            .0
            .to_number(factory_ref)
            .map_err(IndicatorError::NumError)?;

        Ok(ConstantIndicator::new(series, value))
    }
}

/// 对于已经是指标的，直接返回自己
impl<'a, T, S, I> IntoIndicator<'a, T, S, I> for I
where
    T: TrNum + 'static,
    S: for<'any> BarSeries<'any, T> + 'a,
    I: Indicator<Num = T> + Clone + 'a,
{
    type IndicatorType = I;

    fn as_indicator(&self, _first: &'a I) -> Result<Self::IndicatorType, IndicatorError> {
        Ok(self.clone())
    }
}

pub trait OptionExt<T> {
    fn or_invalid_index(self, index: usize, max: usize) -> Result<T, IndicatorError>;
}
