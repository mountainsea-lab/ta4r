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
use crate::indicators::abstract_indicator::BaseIndicator;
use crate::indicators::{Indicator, OptionExt, ToNumber};
use crate::num::types::NumError;
use crate::num::{NumFactory, TrNum};
use std::sync::Arc;
use thiserror::Error;

///===========================base sturct types======================
#[derive(Debug, Clone, Error)]
pub enum IndicatorError {
    #[error("Invalid index: {index} (max allowed: {max})")]
    InvalidIndex { index: usize, max: usize },

    #[error("OutOfBounds index: {index}")]
    OutOfBounds { index: usize },

    #[error("Calculation error: {message}")]
    CalculationError { message: String },

    #[error("Number error: {0}")]
    NumError(#[from] NumError),

    #[error("Other error: {message}")]
    Other { message: String },
}

// 二元运算符定义
#[derive(Clone, Copy)]
pub enum BinaryOp<T: TrNum> {
    Simple(fn(&T, &T) -> T),
    Fallible(fn(&T, &T) -> Result<T, IndicatorError>),
}

pub enum UnaryOp<T: TrNum> {
    Simple(fn(&T) -> T),
    Fallible(fn(&T) -> Result<T, IndicatorError>),
    ClosureFallible(Arc<dyn Fn(&T) -> Result<T, IndicatorError> + Send + Sync>),
}

impl<T: TrNum> Clone for UnaryOp<T> {
    fn clone(&self) -> Self {
        match self {
            UnaryOp::Simple(f) => UnaryOp::Simple(*f),
            UnaryOp::Fallible(f) => UnaryOp::Fallible(*f),
            UnaryOp::ClosureFallible(arc_fn) => UnaryOp::ClosureFallible(arc_fn.clone()),
        }
    }
}

/// 数字包装类型 NumConst<T>
#[derive(Clone, Debug)]
pub struct NumConst<T>(pub T);

/// 转换数字类型的实现，针对包装类型内部具体数字类型的转换
impl<T> ToNumber<T> for NumConst<i64>
where
    T: TrNum + Clone + 'static,
{
    fn to_number(&self, factory: &T::Factory) -> Result<T, NumError> {
        Ok(factory.num_of_i64(self.0))
    }
}

/// 针对 &str 的实现
impl<T> ToNumber<T> for NumConst<&str>
where
    T: TrNum + Clone + 'static,
{
    fn to_number(&self, factory: &T::Factory) -> Result<T, NumError> {
        factory.num_of_str(self.0)
    }
}

/// ========================tools=============================
pub type IndicatorResult<T> = Result<T, IndicatorError>;

impl<T> OptionExt<T> for Option<T> {
    fn or_invalid_index(self, index: usize, max: usize) -> Result<T, IndicatorError> {
        self.ok_or(IndicatorError::InvalidIndex { index, max })
    }
}

/// IndicatorCalculator trait —— 不再引用 `CachedIndicator`，改为 `BaseIndicator`
pub trait IndicatorCalculator<T, S>
where
    T: TrNum + Clone + 'static,
    S: BarSeries<T>,
{
    /// 计算结果类型（数值指标 = T，其他指标 = 特定类型）
    type Output: Clone + 'static;
    fn calculate(
        &self,
        base: &BaseIndicator<T, S>,
        index: usize,
    ) -> Result<Self::Output, IndicatorError>;
}

pub enum IterMode<T, S> {
    /// Snapshot 模式，持有 Vec<T> 拷贝
    Snapshot(Vec<T>), // parking_lot::RwLockReadGuard<'a, S>
    /// Incremental 模式，持有 BarSeriesRef
    Incremental(BarSeriesRef<S>),
}

pub struct IndicatorIterator<'a, I, T, S>
where
    T: TrNum + 'static,
    S: BarSeries<T> + 'static,
    I: Indicator<Series = S>,
{
    indicator: &'a I,
    index: usize,
    end: usize,
    mode: IterMode<T, S>,
}

impl<'a, I, T, S> IndicatorIterator<'a, I, T, S>
where
    T: TrNum + 'static,
    S: BarSeries<T> + 'static,
    I: Indicator<Series = S>,
    T: From<I::Output>, // 支持 Output 转 Num
{
    /// Snapshot 模式
    pub fn snapshot(indicator: &'a I) -> Self {
        let series_ref = indicator.bar_series();

        let begin = series_ref.with_ref_or(0, |s| s.get_begin_index().unwrap_or(0));
        let end = series_ref.with_ref_or(0, |s| s.get_end_index().unwrap_or(0));

        // 拷贝 Snapshot
        let bars: Vec<T> = (begin..=end)
            .map(|i| indicator.get_value(i).unwrap().into())
            .collect();

        Self {
            indicator,
            index: 0,
            end: bars.len(),
            mode: IterMode::Snapshot(bars),
        }
    }

    /// Incremental 模式
    pub fn incremental(indicator: &'a I, series_ref: BarSeriesRef<S>) -> Self {
        let begin = series_ref.with_ref_or(0, |s| s.get_begin_index().unwrap_or(0));
        let end = series_ref.with_ref_or(0, |s| s.get_end_index().unwrap_or(0));

        Self {
            indicator,
            index: begin,
            end,
            mode: IterMode::Incremental(series_ref),
        }
    }
}

impl<'a, I, T, S> Iterator for IndicatorIterator<'a, I, T, S>
where
    T: TrNum + 'static,
    S: BarSeries<T> + 'static,
    I: Indicator<Series = S>,
    T: From<I::Output>, // 支持 Output 转 Num
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.end {
            return None;
        }

        let value = match &self.mode {
            IterMode::Snapshot(bars) => bars.get(self.index).cloned(),
            IterMode::Incremental(_series_ref) => {
                // 安全处理 Result -> Option
                self.indicator
                    .get_value(self.index)
                    .ok() // 将 Result<T, E> 转为 Option<T>
                    .map(|v| v.into())
            }
        };

        self.index += 1;
        value
    }
}
