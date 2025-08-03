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

pub struct IndicatorIterator<'a, I: Indicator> {
    pub(crate) indicator: &'a I,
    pub(crate) index: usize,
    pub(crate) end: usize,
}

impl<'a, I: Indicator> Iterator for IndicatorIterator<'a, I> {
    type Item = Result<I::Num, IndicatorError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index > self.end {
            None
        } else {
            let result = self.indicator.get_value(self.index);
            self.index += 1;
            Some(result)
        }
    }
}

impl<T> OptionExt<T> for Option<T> {
    fn or_invalid_index(self, index: usize, max: usize) -> Result<T, IndicatorError> {
        self.ok_or(IndicatorError::InvalidIndex { index, max })
    }
}

/// IndicatorCalculator trait —— 不再引用 `CachedIndicator`，改为 `BaseIndicator`
pub trait IndicatorCalculator<'a, T, S>
where
    T: TrNum + Clone + 'static,
    S: BarSeries<'a, T>,
{
    fn calculate(&self, base: &BaseIndicator<'a, T, S>, index: usize) -> Result<T, IndicatorError>;
}
