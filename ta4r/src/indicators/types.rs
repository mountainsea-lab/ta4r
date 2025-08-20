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
use crate::indicators::abstract_indicator::BaseIndicator;
use crate::indicators::{Indicator, OptionExt, ToNumber};
use crate::num::types::NumError;
use crate::num::{NumFactory, TrNum};
use parking_lot::RwLock;
use std::sync::Arc;
use thiserror::Error;
use crate::bar::builder::types::BarSeriesRef;

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


// pub enum IterMode<'a, S: 'a> {
//     Snapshot(parking_lot::RwLockReadGuard<'a, S>),
//     Incremental(&'a Arc<RwLock<S>>),
// }
//
// pub struct IndicatorIterator<'a, I>
// where
//     I: 'a + Indicator,
// {
//     indicator: &'a I,
//     index: usize,
//     end: usize,
//     mode: IterMode<'a, I::Series>,
// }
//
// impl<'a, I> IndicatorIterator<'a, I>
// where
//     I: Indicator,
// {
//     /// 快照模式迭代器
//     pub fn snapshot(indicator: &'a I) -> Self {
//         let series = indicator.bar_series();
//         let series_read = series.read();
//         let begin = series_read.get_begin_index().unwrap_or(0);
//         let end = series_read.get_end_index().unwrap_or(0);
//
//         Self {
//             indicator,
//             index: begin,
//             end,
//             mode: IterMode::Snapshot(series_read),
//         }
//     }
//
//     /// 增量模式迭代器
//     pub fn incremental(indicator: &'a I) -> Self {
//         let series = indicator.bar_series();
//         let begin = series.read().get_begin_index().unwrap_or(0);
//         let end = series.read().get_end_index().unwrap_or(0);
//
//         Self {
//             indicator,
//             index: begin,
//             end,
//             mode: IterMode::Incremental(&series),
//         }
//     }
// }
//
// impl<'a, I> Iterator for IndicatorIterator<'a, I>
// where
//     I: Indicator,
// {
//     type Item = Result<I::Output, IndicatorError>;
//
//     fn next(&mut self) -> Option<Self::Item> {
//         if self.index > self.end {
//             return None;
//         }
//
//         let value = match &self.mode {
//             IterMode::Snapshot(_series_read) => {
//                 // snapshot 模式直接读
//                 self.indicator.get_value(self.index)
//             }
//             IterMode::Incremental(series_arc) => {
//                 // incremental 模式每次临时读锁
//                 let series_read = series_arc.read();
//                 // 更新 end，实时感知追加 bar
//                 self.end = series_read.get_end_index().unwrap_or(self.end);
//                 self.indicator.get_value(self.index)
//             }
//         };
//
//         self.index += 1;
//         Some(value)
//     }
// }

pub enum IterMode<T, S> {
    /// Snapshot 模式，持有 Vec<T> 拷贝
    Snapshot(Vec<T>),
    /// Incremental 模式，持有 BarSeriesRef
    Incremental(BarSeriesRef<S>),
}

pub struct IndicatorIterator<'a, I, T, S>
where
    I: Indicator<Num = T, Series = S>,
    T: Clone,
{
    indicator: &'a I,
    index: usize,
    end: usize,
    mode: IterMode<T, S>,
}

impl<'a, I, T, S> IndicatorIterator<'a, I, T, S>
where
    I: Indicator<Num = T, Series = S>,
    T: Clone,
{
    /// Snapshot 模式
    pub fn snapshot(indicator: &'a I) -> Self {
        let series = indicator.bar_series();
        let begin = series.get_begin_index().unwrap_or(0);
        let end = series.get_end_index().unwrap_or(0);

        let bars: Vec<_> = (begin..=end)
            .map(|i| indicator.get_value(i).unwrap()) // 或 clone
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
        let begin = match &series_ref {
            BarSeriesRef::Mut(rc) => rc.borrow().get_begin_index().unwrap_or(0),
            BarSeriesRef::Shared(mutex) => mutex.lock().unwrap().get_begin_index().unwrap_or(0),
            BarSeriesRef::RawMut(ptr) => unsafe { (**ptr).get_begin_index().unwrap_or(0) },
            BarSeriesRef::None => 0,
        };

        let end = match &series_ref {
            BarSeriesRef::Mut(rc) => rc.borrow().get_end_index().unwrap_or(0),
            BarSeriesRef::Shared(mutex) => mutex.lock().unwrap().get_end_index().unwrap_or(0),
            BarSeriesRef::RawMut(ptr) => unsafe { (**ptr).get_end_index().unwrap_or(0) },
            BarSeriesRef::None => 0,
        };

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
    I: Indicator<Num = T, Series = S>,
    T: Clone,
{
    type Item = Result<I::Output, IndicatorError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.end {
            return None;
        }

        let value = match &mut self.mode {
            IterMode::Snapshot(bars) => {
                self.indicator.get_value(self.index)
            }
            IterMode::Incremental(series_ref) => match series_ref {
                BarSeriesRef::Mut(rc) => {
                    let series = rc.borrow();
                    self.end = series.get_end_index().unwrap_or(self.end);
                    self.indicator.get_value(self.index)
                }
                BarSeriesRef::Shared(mutex) => {
                    let series = mutex.lock().unwrap();
                    self.end = series.get_end_index().unwrap_or(self.end);
                    self.indicator.get_value(self.index)
                }
                BarSeriesRef::RawMut(ptr) => unsafe {
                    let series = &**ptr;
                    self.end = series.get_end_index().unwrap_or(self.end);
                    self.indicator.get_value(self.index)
                }
                BarSeriesRef::None => return None,
            },
        };

        self.index += 1;
        Some(value)
    }
}

