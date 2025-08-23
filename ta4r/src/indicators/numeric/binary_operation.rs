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
use crate::indicators::Indicator;
use crate::indicators::types::{BinaryOp, IndicatorError};
use crate::num::TrNum;
use std::sync::Arc;

// pub struct BinaryOperation<T, L, R>
// where
//     T: TrNum,
//     L: Indicator<Num = T, Output = T>,
//     R: Indicator<Num = T, Output = T>,
// {
//     left: L,
//     right: R,
//     operator: BinaryOp<T>,
//     _marker: PhantomData<T>, // 关联泛型，避免编译器报未使用泛型错误
// }
//
// impl<T, L, R> Clone for BinaryOperation<T, L, R>
// where
//     T: TrNum + Clone,
//     L: Indicator<Num = T, Output = T> + Clone,
//     R: Indicator<Num = T, Output = T> + Clone,
//     BinaryOp<T>: Clone,
// {
//     fn clone(&self) -> Self {
//         Self {
//             left: self.left.clone(),
//             right: self.right.clone(),
//             operator: self.operator.clone(),
//             _marker: PhantomData,
//         }
//     }
// }
//
// impl<T, L, R> BinaryOperation<T, L, R>
// where
//     T: TrNum + 'static,
//     L: Indicator<Num = T, Output = T>,
//     R: Indicator<Num = T, Output = T>,
// {
//     pub fn new_simple(left: L, right: R, op: fn(&T, &T) -> T) -> Self {
//         Self {
//             left,
//             right,
//             operator: BinaryOp::Simple(op),
//             _marker: PhantomData,
//         }
//     }
//
//     pub fn new_fallible(left: L, right: R, op: fn(&T, &T) -> Result<T, IndicatorError>) -> Self {
//         Self {
//             left,
//             right,
//             operator: BinaryOp::Fallible(op),
//             _marker: PhantomData,
//         }
//     }
//
//     /// 工厂方法模板代码,辅助方法，减少重复
//     fn from_simple_op<'a, S, I, LI, RI>(
//         left: &'a LI,
//         right: &'a RI,
//         op: fn(&T, &T) -> T,
//     ) -> Result<BinaryOperation<T, LI::IndicatorType, RI::IndicatorType>, IndicatorError>
//     where
//         T: TrNum + 'static,
//         S: BarSeries<T> + 'static,
//         I: Indicator<Num = T, Output = T> + Clone + 'a,
//         LI: IntoIndicator<T, S, I> + AsRef<I> + 'a,
//         RI: IntoIndicator<T, S, I> + AsRef<I> + 'a,
//     {
//         let first: &I = left.as_ref();
//         let l = left.as_indicator(first)?;
//         let r = right.as_indicator(first)?;
//         Ok(BinaryOperation::new_simple(l, r, op))
//     }
//
//     /// 工厂方法模板代码,辅助方法，减少重复
//     fn from_fallible_op<'a, S, I, LI, RI>(
//         left: &'a LI,
//         right: &'a RI,
//         op: fn(&T, &T) -> Result<T, IndicatorError>,
//     ) -> Result<BinaryOperation<T, LI::IndicatorType, RI::IndicatorType>, IndicatorError>
//     where
//         T: TrNum + 'static,
//         S: BarSeries<T> + 'static,
//         I: Indicator<Num = T, Output = T> + Clone + 'a,
//         LI: IntoIndicator<T, S, I> + AsRef<I> + 'a,
//         RI: IntoIndicator<T, S, I> + AsRef<I> + 'a,
//     {
//         let first: &I = left.as_ref();
//         let l = left.as_indicator(first)?;
//         let r = right.as_indicator(first)?;
//         Ok(BinaryOperation::new_fallible(l, r, op))
//     }
//
//     // 工厂方法：左右输入更灵活，自动转成指标
//     pub fn sum<'a, LI, RI, S, I>(
//         left: &'a LI,
//         right: &'a RI,
//     ) -> Result<BinaryOperation<T, LI::IndicatorType, RI::IndicatorType>, IndicatorError>
//     where
//         T: TrNum + 'static,
//         S: BarSeries<T> + 'static,
//         I: Indicator<Num = T, Output = T> + Clone + 'a,
//         LI: IntoIndicator<T, S, I> + AsRef<I> + 'a,
//         RI: IntoIndicator<T, S, I> + AsRef<I> + 'a,
//     {
//         BinaryOperation::<T, L, R>::from_simple_op(left, right, |a, b| a.plus(b))
//     }
//
//     /// 差值 left - right
//     pub fn difference<'a, LI, RI, S, I>(
//         left: &'a LI,
//         right: &'a RI,
//     ) -> Result<BinaryOperation<T, LI::IndicatorType, RI::IndicatorType>, IndicatorError>
//     where
//         T: TrNum + 'static,
//         S: BarSeries<T> + 'static,
//         I: Indicator<Num = T, Output = T> + Clone + 'a,
//         LI: IntoIndicator<T, S, I> + AsRef<I> + 'a,
//         RI: IntoIndicator<T, S, I> + AsRef<I> + 'a,
//     {
//         BinaryOperation::<T, L, R>::from_simple_op(left, right, |a, b| a.minus(b))
//     }
//
//     /// 乘积 left * right
//     pub fn product<'a, LI, RI, S, I>(
//         left: &'a LI,
//         right: &'a RI,
//     ) -> Result<BinaryOperation<T, LI::IndicatorType, RI::IndicatorType>, IndicatorError>
//     where
//         T: TrNum + 'static,
//         S: BarSeries<T> + 'static,
//         I: Indicator<Num = T, Output = T> + Clone + 'a,
//         LI: IntoIndicator<T, S, I> + AsRef<I> + 'a,
//         RI: IntoIndicator<T, S, I> + AsRef<I> + 'a,
//     {
//         BinaryOperation::<T, L, R>::from_simple_op(left, right, |a, b| a.multiplied_by(b))
//     }
//
//     /// 商 left / right
//     pub fn quotient<'a, LI, RI, S, I>(
//         left: &'a LI,
//         right: &'a RI,
//     ) -> Result<BinaryOperation<T, LI::IndicatorType, RI::IndicatorType>, IndicatorError>
//     where
//         T: TrNum + 'static,
//         S: BarSeries<T> + 'static,
//         I: Indicator<Num = T, Output = T> + Clone + 'a,
//         LI: IntoIndicator<T, S, I> + AsRef<I> + 'a,
//         RI: IntoIndicator<T, S, I> + AsRef<I> + 'a,
//     {
//         BinaryOperation::<T, L, R>::from_fallible_op(left, right, |a, b| {
//             a.divided_by(b).map_err(IndicatorError::NumError)
//         })
//     }
//
//     /// 最小值
//     pub fn min<'a, LI, RI, S, I>(
//         left: &'a LI,
//         right: &'a RI,
//     ) -> Result<BinaryOperation<T, LI::IndicatorType, RI::IndicatorType>, IndicatorError>
//     where
//         T: TrNum + 'static,
//         S: BarSeries<T> + 'static,
//         I: Indicator<Num = T, Output = T> + Clone + 'a,
//         LI: IntoIndicator<T, S, I> + AsRef<I> + 'a,
//         RI: IntoIndicator<T, S, I> + AsRef<I> + 'a,
//     {
//         BinaryOperation::<T, L, R>::from_simple_op(left, right, |a, b| a.min(b))
//     }
//
//     /// 最大值
//     pub fn max<'a, LI, RI, S, I>(
//         left: &'a LI,
//         right: &'a RI,
//     ) -> Result<BinaryOperation<T, LI::IndicatorType, RI::IndicatorType>, IndicatorError>
//     where
//         T: TrNum + 'static,
//         S: BarSeries<T> + 'static,
//         I: Indicator<Num = T, Output = T> + Clone + 'a,
//         LI: IntoIndicator<T, S, I> + AsRef<I> + 'a,
//         RI: IntoIndicator<T, S, I> + AsRef<I> + 'a,
//     {
//         BinaryOperation::<T, L, R>::from_simple_op(left, right, |a, b| a.max(b))
//     }
//
//     pub fn get_value(&self, index: usize) -> Result<T, IndicatorError> {
//         let left_val = self.left.get_value(index)?;
//         let right_val = self.right.get_value(index)?;
//         match self.operator {
//             BinaryOp::Simple(op) => Ok(op(&left_val, &right_val)),
//             BinaryOp::Fallible(op) => op(&left_val, &right_val),
//         }
//     }
//
//     pub fn count_of_unstable_bars(&self) -> usize {
//         usize::max(
//             self.left.count_of_unstable_bars(),
//             self.right.count_of_unstable_bars(),
//         )
//     }
// }
//
// impl<T, L, R> Indicator for BinaryOperation<T, L, R>
// where
//     T: TrNum + 'static,
//     L: Indicator<Num = T, Output = T>,
//     R: Indicator<Num = T, Output = T, Series = L::Series>,
// {
//     type Num = T;
//     type Output = T;
//     type Series = L::Series;
//
//     fn get_value(&self, index: usize) -> Result<Self::Num, IndicatorError> {
//         self.get_value(index)
//     }
//
//     fn bar_series(&self) -> BarSeriesRef<Self::Series> {
//         self.left.bar_series()
//     }
//
//     fn count_of_unstable_bars(&self) -> usize {
//         self.left
//             .count_of_unstable_bars()
//             .max(self.right.count_of_unstable_bars())
//     }
// }

/// 二元运算指标，**仅持有对左右指标的引用**（零克隆、零分配）
// pub struct BinaryOperation<'a, T, L, R>
// where
//     T: TrNum,
//     L: Indicator<Num = T, Output = T>,
//     R: Indicator<Num = T, Output = T, Series = L::Series>,
// {
//     left:  &'a L,
//     right: &'a R,
//     operator: BinaryOp<T>,
// }
//
// impl<'a, T, L, R> Clone for BinaryOperation<'a, T, L, R>
// where
//     T: TrNum + Clone,
//     L: Indicator<Num = T, Output = T>,
//     R: Indicator<Num = T, Output = T, Series = L::Series>,
//     BinaryOp<T>: Clone,
// {
//     fn clone(&self) -> Self {
//         Self {
//             left:  self.left,
//             right: self.right,
//             operator: self.operator.clone(),
//         }
//     }
// }
//
// impl<'a, T, L, R> BinaryOperation<'a, T, L, R>
// where
//     T: TrNum + 'static,
//     L: Indicator<Num = T, Output = T>,
//     R: Indicator<Num = T, Output = T, Series = L::Series>,
// {
//     /// 基础构造：纯函数型二元操作
//     pub fn new_simple(left: &'a L, right: &'a R, op: fn(&T, &T) -> T) -> Self {
//         Self { left, right, operator: BinaryOp::Simple(op) }
//     }
//
//     /// 基础构造：可能失败的二元操作（例如除法需检查除数）
//     pub fn new_fallible(left: &'a L, right: &'a R, op: fn(&T, &T) -> Result<T, IndicatorError>) -> Self {
//         Self { left, right, operator: BinaryOp::Fallible(op) }
//     }
//
//     // ---------------- 工厂方法（按常用数学运算分类） ----------------
//
//     /// 和：left + right
//     pub fn sum(left: &'a L, right: &'a R) -> Self {
//         Self::new_simple(left, right, |a, b| a.plus(b))
//     }
//
//     /// 差：left - right
//     pub fn difference(left: &'a L, right: &'a R) -> Self {
//         Self::new_simple(left, right, |a, b| a.minus(b))
//     }
//
//     /// 积：left * right
//     pub fn product(left: &'a L, right: &'a R) -> Self {
//         Self::new_simple(left, right, |a, b| a.multiplied_by(b))
//     }
//
//     /// 商：left / right（可失败）
//     pub fn quotient(left: &'a L, right: &'a R) -> Self {
//         Self::new_fallible(left, right, |a, b| a.divided_by(b).map_err(IndicatorError::NumError))
//     }
//
//     /// 逐点最小值：min(left, right)
//     pub fn min(left: &'a L, right: &'a R) -> Self {
//         Self::new_simple(left, right, |a, b| a.min(b))
//     }
//
//     /// 逐点最大值：max(left, right)
//     pub fn max(left: &'a L, right: &'a R) -> Self {
//         Self::new_simple(left, right, |a, b| a.max(b))
//     }
//
//     /// 计算值
//     pub fn get_value(&self, index: usize) -> Result<T, IndicatorError> {
//         let lv = self.left.get_value(index)?;
//         let rv = self.right.get_value(index)?;
//         match self.operator {
//             BinaryOp::Simple(op)   => Ok(op(&lv, &rv)),
//             BinaryOp::Fallible(op) => op(&lv, &rv),
//         }
//     }
//
//     /// 非稳定条数（取左右中较大者）
//     pub fn count_of_unstable_bars(&self) -> usize {
//         usize::max(self.left.count_of_unstable_bars(), self.right.count_of_unstable_bars())
//     }
// }
//
// impl<'a, T, L, R> Indicator for BinaryOperation<'a, T, L, R>
// where
//     T: TrNum + 'static,
//     L: Indicator<Num = T, Output = T>,
//     R: Indicator<Num = T, Output = T, Series = L::Series>,
// {
//     type Num    = T;
//     type Output = T;
//     type Series = L::Series;
//
//     fn get_value(&self, index: usize) -> Result<Self::Num, IndicatorError> {
//         BinaryOperation::get_value(self, index)
//     }
//
//     fn bar_series(&self) -> BarSeriesRef<Self::Series> {
//         self.left.bar_series()
//     }
//
//     fn count_of_unstable_bars(&self) -> usize {
//         BinaryOperation::count_of_unstable_bars(self)
//     }
// }

/// 二元运算指标，
/// **通过 `Arc` 持有左右指标引用**（避免生命周期传染，轻量 clone）
pub struct BinaryOperation<T, L, R>
where
    T: TrNum,
    L: Indicator<Num = T, Output = T>,
    R: Indicator<Num = T, Output = T, Series = L::Series>,
{
    left: Arc<L>,
    right: Arc<R>,
    operator: BinaryOp<T>,
}

impl<T, L, R> Clone for BinaryOperation<T, L, R>
where
    T: TrNum + Clone,
    L: Indicator<Num = T, Output = T>,
    R: Indicator<Num = T, Output = T, Series = L::Series>,
    BinaryOp<T>: Clone,
{
    fn clone(&self) -> Self {
        Self {
            left: Arc::clone(&self.left),
            right: Arc::clone(&self.right),
            operator: self.operator.clone(),
        }
    }
}

impl<T, L, R> BinaryOperation<T, L, R>
where
    T: TrNum + 'static,
    L: Indicator<Num = T, Output = T> + 'static,
    R: Indicator<Num = T, Output = T, Series = L::Series> + 'static,
{
    /// 基础构造：纯函数型二元操作
    pub fn new_simple(left: Arc<L>, right: Arc<R>, op: fn(&T, &T) -> T) -> Self {
        Self {
            left,
            right,
            operator: BinaryOp::Simple(op),
        }
    }

    /// 基础构造：可能失败的二元操作（例如除法需检查除数）
    pub fn new_fallible(
        left: Arc<L>,
        right: Arc<R>,
        op: fn(&T, &T) -> Result<T, IndicatorError>,
    ) -> Self {
        Self {
            left,
            right,
            operator: BinaryOp::Fallible(op),
        }
    }

    // ---------------- 工厂方法（按常用数学运算分类） ----------------

    /// 和：left + right
    pub fn sum(left: Arc<L>, right: Arc<R>) -> Self {
        Self::new_simple(left, right, |a, b| a.plus(b))
    }

    /// 差：left - right
    pub fn difference(left: Arc<L>, right: Arc<R>) -> Self {
        Self::new_simple(left, right, |a, b| a.minus(b))
    }

    /// 积：left * right
    pub fn product(left: Arc<L>, right: Arc<R>) -> Self {
        Self::new_simple(left, right, |a, b| a.multiplied_by(b))
    }

    /// 商：left / right（可失败）
    pub fn quotient(left: Arc<L>, right: Arc<R>) -> Self {
        Self::new_fallible(left, right, |a, b| {
            a.divided_by(b).map_err(IndicatorError::NumError)
        })
    }

    /// 逐点最小值：min(left, right)
    pub fn min(left: Arc<L>, right: Arc<R>) -> Self {
        Self::new_simple(left, right, |a, b| a.min(b))
    }

    /// 逐点最大值：max(left, right)
    pub fn max(left: Arc<L>, right: Arc<R>) -> Self {
        Self::new_simple(left, right, |a, b| a.max(b))
    }

    /// 计算值
    pub fn get_value(&self, index: usize) -> Result<T, IndicatorError> {
        let lv = self.left.get_value(index)?;
        let rv = self.right.get_value(index)?;
        match self.operator {
            BinaryOp::Simple(op) => Ok(op(&lv, &rv)),
            BinaryOp::Fallible(op) => op(&lv, &rv),
        }
    }

    /// 非稳定条数（取左右中较大者）
    pub fn count_of_unstable_bars(&self) -> usize {
        usize::max(
            self.left.count_of_unstable_bars(),
            self.right.count_of_unstable_bars(),
        )
    }
}

impl<T, L, R> Indicator for BinaryOperation<T, L, R>
where
    T: TrNum + 'static,
    L: Indicator<Num = T, Output = T> + 'static,
    R: Indicator<Num = T, Output = T, Series = L::Series> + 'static,
{
    type Num = T;
    type Output = T;
    type Series = L::Series;

    fn get_value(&self, index: usize) -> Result<Self::Num, IndicatorError> {
        BinaryOperation::get_value(self, index)
    }

    fn bar_series(&self) -> BarSeriesRef<Self::Series> {
        self.left.bar_series()
    }

    fn count_of_unstable_bars(&self) -> usize {
        BinaryOperation::count_of_unstable_bars(self)
    }
}
