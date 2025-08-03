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
use crate::indicators::types::{BinaryOp, IndicatorError};
use crate::indicators::{Indicator, IntoIndicator};
use crate::num::TrNum;
use std::marker::PhantomData;

pub struct BinaryOperation<T, L, R>
where
    T: TrNum,
    L: Indicator<Num = T>,
    R: Indicator<Num = T>,
{
    left: L,
    right: R,
    operator: BinaryOp<T>,
    _marker: PhantomData<T>, // 关联泛型，避免编译器报未使用泛型错误
}

impl<T, L, R> Clone for BinaryOperation<T, L, R>
where
    T: TrNum + Clone,
    L: Indicator<Num = T> + Clone,
    R: Indicator<Num = T> + Clone,
    BinaryOp<T>: Clone,
{
    fn clone(&self) -> Self {
        Self {
            left: self.left.clone(),
            right: self.right.clone(),
            operator: self.operator.clone(),
            _marker: PhantomData,
        }
    }
}

impl<T, L, R> BinaryOperation<T, L, R>
where
    T: TrNum + 'static,
    L: Indicator<Num = T>,
    R: Indicator<Num = T>,
{
    pub fn new_simple(left: L, right: R, op: fn(&T, &T) -> T) -> Self {
        Self {
            left,
            right,
            operator: BinaryOp::Simple(op),
            _marker: PhantomData,
        }
    }

    pub fn new_fallible(left: L, right: R, op: fn(&T, &T) -> Result<T, IndicatorError>) -> Self {
        Self {
            left,
            right,
            operator: BinaryOp::Fallible(op),
            _marker: PhantomData,
        }
    }

    /// 工厂方法模板代码,辅助方法，减少重复
    fn from_simple_op<'a, S, I, LI, RI>(
        left: &'a LI,
        right: &'a RI,
        op: fn(&T, &T) -> T,
    ) -> Result<BinaryOperation<T, LI::IndicatorType, RI::IndicatorType>, IndicatorError>
    where
        T: TrNum + 'static,
        S: for<'any> BarSeries<'any, T> + 'a,
        I: Indicator<Num = T> + Clone + 'a,
        LI: IntoIndicator<'a, T, S, I> + AsRef<I> + 'a,
        RI: IntoIndicator<'a, T, S, I> + AsRef<I> + 'a,
    {
        let first: &I = left.as_ref();
        let l = left.as_indicator(first)?;
        let r = right.as_indicator(first)?;
        Ok(BinaryOperation::new_simple(l, r, op))
    }

    /// 工厂方法模板代码,辅助方法，减少重复
    fn from_fallible_op<'a, S, I, LI, RI>(
        left: &'a LI,
        right: &'a RI,
        op: fn(&T, &T) -> Result<T, IndicatorError>,
    ) -> Result<BinaryOperation<T, LI::IndicatorType, RI::IndicatorType>, IndicatorError>
    where
        T: TrNum + 'static,
        S: for<'any> BarSeries<'any, T> + 'a,
        I: Indicator<Num = T> + Clone + 'a,
        LI: IntoIndicator<'a, T, S, I> + AsRef<I> + 'a,
        RI: IntoIndicator<'a, T, S, I> + AsRef<I> + 'a,
    {
        let first: &I = left.as_ref();
        let l = left.as_indicator(first)?;
        let r = right.as_indicator(first)?;
        Ok(BinaryOperation::new_fallible(l, r, op))
    }

    // 工厂方法：左右输入更灵活，自动转成指标
    pub fn sum<'a, LI, RI, S, I>(
        left: &'a LI,
        right: &'a RI,
    ) -> Result<BinaryOperation<T, LI::IndicatorType, RI::IndicatorType>, IndicatorError>
    where
        T: TrNum + 'static,
        S: for<'any> BarSeries<'any, T> + 'a,
        I: Indicator<Num = T> + Clone + 'a,
        LI: IntoIndicator<'a, T, S, I> + AsRef<I> + 'a,
        RI: IntoIndicator<'a, T, S, I> + AsRef<I> + 'a,
    {
        BinaryOperation::<T, L, R>::from_simple_op(left, right, |a, b| a.plus(b))
    }

    /// 差值 left - right
    pub fn difference<'a, LI, RI, S, I>(
        left: &'a LI,
        right: &'a RI,
    ) -> Result<BinaryOperation<T, LI::IndicatorType, RI::IndicatorType>, IndicatorError>
    where
        T: TrNum + 'static,
        S: for<'any> BarSeries<'any, T> + 'a,
        I: Indicator<Num = T> + Clone + 'a,
        LI: IntoIndicator<'a, T, S, I> + AsRef<I> + 'a,
        RI: IntoIndicator<'a, T, S, I> + AsRef<I> + 'a,
    {
        BinaryOperation::<T, L, R>::from_simple_op(left, right, |a, b| a.minus(b))
    }

    /// 乘积 left * right
    pub fn product<'a, LI, RI, S, I>(
        left: &'a LI,
        right: &'a RI,
    ) -> Result<BinaryOperation<T, LI::IndicatorType, RI::IndicatorType>, IndicatorError>
    where
        T: TrNum + 'static,
        S: for<'any> BarSeries<'any, T> + 'a,
        I: Indicator<Num = T> + Clone + 'a,
        LI: IntoIndicator<'a, T, S, I> + AsRef<I> + 'a,
        RI: IntoIndicator<'a, T, S, I> + AsRef<I> + 'a,
    {
        BinaryOperation::<T, L, R>::from_simple_op(left, right, |a, b| a.multiplied_by(b))
    }

    /// 商 left / right
    pub fn quotient<'a, LI, RI, S, I>(
        left: &'a LI,
        right: &'a RI,
    ) -> Result<BinaryOperation<T, LI::IndicatorType, RI::IndicatorType>, IndicatorError>
    where
        T: TrNum + 'static,
        S: for<'any> BarSeries<'any, T> + 'a,
        I: Indicator<Num = T> + Clone + 'a,
        LI: IntoIndicator<'a, T, S, I> + AsRef<I> + 'a,
        RI: IntoIndicator<'a, T, S, I> + AsRef<I> + 'a,
    {
        BinaryOperation::<T, L, R>::from_fallible_op(left, right, |a, b| {
            a.divided_by(b).map_err(IndicatorError::NumError)
        })
    }

    /// 最小值
    pub fn min<'a, LI, RI, S, I>(
        left: &'a LI,
        right: &'a RI,
    ) -> Result<BinaryOperation<T, LI::IndicatorType, RI::IndicatorType>, IndicatorError>
    where
        T: TrNum + 'static,
        S: for<'any> BarSeries<'any, T> + 'a,
        I: Indicator<Num = T> + Clone + 'a,
        LI: IntoIndicator<'a, T, S, I> + AsRef<I> + 'a,
        RI: IntoIndicator<'a, T, S, I> + AsRef<I> + 'a,
    {
        BinaryOperation::<T, L, R>::from_simple_op(left, right, |a, b| a.min(b))
    }

    /// 最大值
    pub fn max<'a, LI, RI, S, I>(
        left: &'a LI,
        right: &'a RI,
    ) -> Result<BinaryOperation<T, LI::IndicatorType, RI::IndicatorType>, IndicatorError>
    where
        T: TrNum + 'static,
        S: for<'any> BarSeries<'any, T> + 'a,
        I: Indicator<Num = T> + Clone + 'a,
        LI: IntoIndicator<'a, T, S, I> + AsRef<I> + 'a,
        RI: IntoIndicator<'a, T, S, I> + AsRef<I> + 'a,
    {
        BinaryOperation::<T, L, R>::from_simple_op(left, right, |a, b| a.max(b))
    }

    pub fn get_value(&self, index: usize) -> Result<T, IndicatorError> {
        let left_val = self.left.get_value(index)?;
        let right_val = self.right.get_value(index)?;
        match self.operator {
            BinaryOp::Simple(op) => Ok(op(&left_val, &right_val)),
            BinaryOp::Fallible(op) => op(&left_val, &right_val),
        }
    }

    pub fn get_count_of_unstable_bars(&self) -> usize {
        usize::max(
            self.left.get_count_of_unstable_bars(),
            self.right.get_count_of_unstable_bars(),
        )
    }
}

impl<T, L, R> Indicator for BinaryOperation<T, L, R>
where
    T: TrNum + 'static,
    L: Indicator<Num = T>,
    R: Indicator<Num = T>,
{
    type Num = T;

    type Series<'s>
        = L::Series<'s>
    where
        Self: 's;

    fn get_value(&self, index: usize) -> Result<Self::Num, IndicatorError> {
        self.get_value(index)
    }

    fn get_bar_series(&self) -> &Self::Series<'_> {
        self.left.get_bar_series()
    }

    fn get_count_of_unstable_bars(&self) -> usize {
        self.left
            .get_count_of_unstable_bars()
            .max(self.right.get_count_of_unstable_bars())
    }
}
