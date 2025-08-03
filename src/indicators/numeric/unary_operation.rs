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
use crate::indicators::types::{IndicatorError, UnaryOp};
use crate::indicators::{Indicator, IntoIndicator};
use crate::num::decimal_num::DecimalNum;
use crate::num::decimal_num_factory::DecimalNumFactory;
use crate::num::types::NumError;
use crate::num::{DecimalFactory, NumFactory, TrNum};
use num_traits::{FromPrimitive, ToPrimitive};
use once_cell::sync::Lazy;
use rust_decimal::Decimal;
use std::marker::PhantomData;
use std::sync::Arc;

/// TODO 如果你对性能要求极高且调用链长，建议方案泛型闭包封装。改造UnaryOperation<T, I, F> 增加对F的支持,后续有时间再优化了
pub struct UnaryOperation<T, I>
where
    T: TrNum,
    I: Indicator<Num = T>,
{
    operand: I,
    operator: UnaryOp<T>,
    _marker: PhantomData<T>,
}

impl<T, I> Clone for UnaryOperation<T, I>
where
    T: TrNum + Clone,
    I: Indicator<Num = T> + Clone,
{
    fn clone(&self) -> Self {
        Self {
            operand: self.operand.clone(),
            operator: self.operator.clone(),
            _marker: PhantomData,
        }
    }
}

impl<T, I> UnaryOperation<T, I>
where
    T: TrNum + 'static,
    I: Indicator<Num = T>,
{
    pub fn new_simple(operand: I, op: fn(&T) -> T) -> Self {
        Self {
            operand,
            operator: UnaryOp::Simple(op),
            _marker: PhantomData,
        }
    }

    pub fn new_fallible(operand: I, op: fn(&T) -> Result<T, IndicatorError>) -> Self {
        Self {
            operand,
            operator: UnaryOp::Fallible(op),
            _marker: PhantomData,
        }
    }

    pub fn new_closure_fallible<F>(operand: I, op: F) -> Self
    where
        F: Fn(&T) -> Result<T, IndicatorError> + Send + Sync + 'static,
    {
        Self {
            operand,
            operator: UnaryOp::ClosureFallible(Arc::new(Box::new(op))),
            _marker: PhantomData,
        }
    }

    pub fn from_simple_op<'a, OI, S>(
        operand: &'a OI,
        op: fn(&T) -> T,
    ) -> Result<UnaryOperation<T, OI::IndicatorType>, IndicatorError>
    where
        S: for<'any> BarSeries<'any, T> + 'a,
        I: Indicator<Num = T> + Clone + 'a,
        OI: IntoIndicator<'a, T, S, I> + AsRef<I> + 'a,
    {
        let base = operand.as_ref();
        let op_ind = operand.as_indicator(base)?;
        Ok(UnaryOperation::new_simple(op_ind, op))
    }

    pub fn from_fallible_op<'a, IO, S>(
        operand: &'a IO,
        op: fn(&T) -> Result<T, IndicatorError>,
    ) -> Result<UnaryOperation<T, IO::IndicatorType>, IndicatorError>
    where
        S: for<'any> BarSeries<'any, T> + 'a,
        I: Indicator<Num = T> + Clone + 'a,
        IO: IntoIndicator<'a, T, S, I> + AsRef<I> + 'a,
        // F: Fn(&T) -> Result<T, IndicatorError> + Send + Sync + 'static,
    {
        let base = operand.as_ref();
        let op_ind = operand.as_indicator(base)?;
        Ok(UnaryOperation::new_fallible(op_ind, op))
    }

    // 内置函数：sqrt（使用 Fallible 版本，因为 sqrt 可能失败）
    pub fn sqrt<'a, OI, S>(
        operand: &'a OI,
    ) -> Result<UnaryOperation<T, OI::IndicatorType>, IndicatorError>
    where
        T: TrNum + 'static,
        S: for<'any> BarSeries<'any, T> + 'a,
        I: Indicator<Num = T> + Clone + 'a,
        OI: IntoIndicator<'a, T, S, I> + AsRef<I> + 'a,
    {
        UnaryOperation::<T, I>::from_fallible_op(operand, |v| {
            v.sqrt().map_err(IndicatorError::NumError)
        })
    }

    pub fn abs<'a, OI, S>(
        operand: &'a OI,
    ) -> Result<UnaryOperation<T, OI::IndicatorType>, IndicatorError>
    where
        T: TrNum + 'static,
        S: for<'any> BarSeries<'any, T> + 'a,
        I: Indicator<Num = T> + Clone + 'a,
        OI: IntoIndicator<'a, T, S, I> + AsRef<I> + 'a,
    {
        UnaryOperation::<T, I>::from_simple_op(operand, |v| v.abs())
    }

    // pub fn pow<'a, OI, S>(
    //     operand: &'a OI,
    //     exponent: impl Into<i64>,
    // ) -> Result<UnaryOperation<T, OI::IndicatorType>, IndicatorError>
    // where
    //     T: TrNum + Clone + 'static,
    //     S: for<'any> BarSeries<'any, T> + 'a,
    //     I: Indicator<Num = T> + Clone + 'a,
    //     OI: IntoIndicator<'a, T, S, I> + AsRef<I> + 'a,
    // {
    //     let series = operand.as_ref().get_bar_series();
    //     let factory_ref: &T::Factory = series.factory_ref();
    //     let num_exponent = factory_ref.num_of_i64(exponent.into());
    //
    //     let pow_op = move |base: &T| base.pow_num(&num_exponent).map_err(IndicatorError::NumError);
    //
    //     let operand_ref = operand.as_ref();
    //     let op_ind = operand.as_indicator(operand_ref)?;
    //
    //     Ok(UnaryOperation::<T, I>::new_closure_fallible(op_ind, pow_op))
    // }
    pub fn pow<'a, OI, S>(
        operand: &'a OI,
        exponent: impl Into<i64>,
    ) -> Result<UnaryOperation<T, OI::IndicatorType>, IndicatorError>
    where
        T: TrNum + Clone + 'static,
        S: for<'any> BarSeries<'any, T> + 'a,
        I: Indicator<Num = T> + Clone + 'a,
        OI: IntoIndicator<'a, T, S, I> + AsRef<I> + 'a,
    {
        // 1. 获取BarSeries和工厂
        let series = operand.as_ref().get_bar_series();
        let factory_ref: &T::Factory = series.factory_ref();

        // 2. 将普通整数转自定义数字类型
        let num_exponent = factory_ref.num_of_i64(exponent.into());

        // 3. 构造闭包函数
        let pow_op = move |base: &T| {
            base.pow_num(&num_exponent)
                .map_err(IndicatorError::NumError)
        };

        // 4. 通过 as_indicator 获取指标实例（类型 I）
        let op_ind = operand.as_indicator(operand.as_ref())?;

        // 5. 构造 UnaryOperation
        Ok(UnaryOperation::<T, OI::IndicatorType>::new_closure_fallible(op_ind, pow_op))
    }

    pub fn get_value(&self, index: usize) -> Result<T, IndicatorError> {
        let operand = self.operand.get_value(index)?;
        match &self.operator {
            UnaryOp::Simple(op) => Ok(op(&operand)),
            UnaryOp::Fallible(op) => op(&operand),
            UnaryOp::ClosureFallible(op) => op(&operand),
        }
    }
}

static DECIMAL_NUM_FACTORY: Lazy<Arc<DecimalNumFactory>> =
    Lazy::new(|| Arc::new(DecimalNumFactory::instance()));

impl<I> UnaryOperation<DecimalNum, I>
where
    I: Indicator<Num = DecimalNum> + Clone + 'static,
{
    fn decimal_log_fn(v: &DecimalNum) -> Result<DecimalNum, IndicatorError> {
        let factory = &*DECIMAL_NUM_FACTORY;
        let f = v.to_f64().ok_or(NumError::InvalidOperation)?;

        if f <= 0.0 || !f.is_finite() {
            Err(IndicatorError::NumError(NumError::DivisionByZero))
        } else {
            let ln = f.ln();
            let decimal = Decimal::from_f64(ln)
                .ok_or(IndicatorError::NumError(NumError::InvalidOperation))?;
            factory
                .num_of_decimal(decimal)
                .map_err(IndicatorError::NumError)
        }
    }

    pub fn log<'a, OI, S>(
        operand: &'a OI,
    ) -> Result<UnaryOperation<DecimalNum, OI::IndicatorType>, IndicatorError>
    where
        S: for<'any> BarSeries<'any, DecimalNum> + 'a,
        OI: IntoIndicator<'a, DecimalNum, S, I> + AsRef<I> + 'a,
    {
        UnaryOperation::<DecimalNum, I>::from_fallible_op(operand, Self::decimal_log_fn)
    }
}

impl<T, I> Indicator for UnaryOperation<T, I>
where
    T: TrNum + 'static,
    I: Indicator<Num = T>,
{
    type Num = T;

    type Series<'s>
        = I::Series<'s>
    where
        Self: 's;

    fn get_value(&self, index: usize) -> Result<Self::Num, IndicatorError> {
        self.get_value(index)
    }

    fn get_bar_series(&self) -> &Self::Series<'_> {
        self.operand.get_bar_series()
    }

    fn get_count_of_unstable_bars(&self) -> usize {
        0
    }
}
