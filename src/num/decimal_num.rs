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

use crate::num::TrNum;
use crate::num::decimal_num_factory::DecimalNumFactory;
use crate::num::types::{MathContext, NumError, NumberDelegate};
use num_traits::Num;
use rust_decimal::prelude::*;
use std::fmt;
use std::ops::{Add, Div, Mul, Neg, Rem, Sub};
use std::str::FromStr;

/// 高精度数值类型，对应ta4j的DecimalNum
#[derive(Clone)]
pub struct DecimalNum {
    delegate: Decimal,
    math_context: MathContext,
}

impl DecimalNum {
    /// 默认构造器，使用默认 MathContext
    pub fn new<T: Into<Decimal>>(val: T) -> Self {
        let ctx = MathContext::default();
        Self::with_context(val, ctx)
    }

    /// 带精度上下文构造器
    pub fn with_context<T: Into<Decimal>>(val: T, math_context: MathContext) -> Self {
        let raw = val.into();
        let delegate = Self::apply_math_context(raw, &math_context);
        Self {
            delegate,
            math_context,
        }
    }

    /// 从字符串构造（返回 Result）
    pub fn from_str_with_context(s: &str, math_context: MathContext) -> Result<Self, NumError> {
        if s.eq_ignore_ascii_case("NaN") {
            return Err(NumError::ParseError("NaN is not a valid number".into()));
        }
        let raw = Decimal::from_str(s)
            .map_err(|_| NumError::ParseError(format!("字符串无法解析为数字 '{}'", s)))?;
        let delegate = Self::apply_math_context(raw, &math_context);

        Ok(Self::with_context(delegate, math_context))
    }

    /// 从字符串构造，默认上下文
    pub fn from_str(s: &str) -> Result<Self, NumError> {
        Self::from_str_with_context(s, MathContext::default())
    }

    /// 获取底层 Decimal
    pub fn delegate(&self) -> Decimal {
        self.delegate
    }

    /// 获取 MathContext
    pub fn math_context(&self) -> &MathContext {
        &self.math_context
    }

    /// 选择两个 DecimalNum 中精度更大的 MathContext
    pub fn choose_math_context_with_greater_precision(&self, other: &Self) -> MathContext {
        if self.math_context.precision > other.math_context.precision {
            self.math_context.clone()
        } else {
            other.math_context.clone()
        }
    }

    /// 应用精度和舍入策略到 Decimal 值
    pub fn apply_math_context(val: Decimal, ctx: &MathContext) -> Decimal {
        val.round_dp_with_strategy(ctx.precision, ctx.rounding_mode)
    }

    pub fn mul_ref(&self, rhs: &Self) -> Self {
        let math_context = self.choose_math_context_with_greater_precision(rhs);
        Self::with_context(self.delegate * rhs.delegate, math_context)
    }

    // /// 类似连分数的展开，核心目标是：避免直接使用 Math.log，而是基于 BigDecimal 实现严格控制精度的 ln(x)
    // pub fn ln(&self) -> Result<Self, NumError> {
    //     // 1. 处理非法输入（ln(x<=0) = NaN）
    //     if self.delegate <= Decimal::ZERO {
    //         return Err(NumError::NaN);
    //     }
    //
    //     // 2. ln(1) = 0
    //     if self.delegate == Decimal::ONE {
    //         return Ok(Self::with_context(Decimal::ZERO, self.math_context.clone()));
    //     }
    //
    //     // 3. 设置迭代次数
    //     let iter: u32 = 1000;
    //
    //     // 4. x = self - 1
    //     let one = DecimalNum::with_context(Decimal::ONE, self.math_context.clone());
    //     let x = self.clone().sub(one);
    //
    //     // 5. 初始化 ret = iter + 1
    //     let mut ret = DecimalNum::with_context(Decimal::from(iter + 1), self.math_context.clone());
    //
    //     // 6. 迭代计算
    //     for i in (0..=iter).rev() {
    //         // N = ((i/2 + 1)^2) * x
    //         let half_plus_one = DecimalNum::with_context(
    //             Decimal::from((i / 2 + 1) as u64),
    //             self.math_context.clone(),
    //         );
    //         let pow_result = half_plus_one.pow(2)?; // 注意传播 Result
    //
    //         let mut N = pow_result.mul_ref(&x); // 引用乘法避免 move
    //
    //         // ret = N / ret
    //         ret = N.div(ret);
    //
    //         // N = i + 1
    //         N = DecimalNum::with_context(Decimal::from(i + 1), self.math_context.clone());
    //
    //         // ret = ret + N
    //         ret = ret.add(N);
    //     }
    //
    //     // ret = x / ret
    //     let result = x.div(ret);
    //     Ok(result)
    // }
}

impl PartialEq for DecimalNum {
    fn eq(&self, other: &Self) -> bool {
        self.delegate == other.delegate
    }
}

impl PartialOrd for DecimalNum {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.delegate.partial_cmp(&other.delegate)
    }
}

// impl AsRef<DecimalNum> for Arc<DecimalNum> {
//     fn as_ref(&self) -> &DecimalNum {
//         self.as_ref()
//     }
// }

// --- From Trait 实现 ---
// 直接用默认构造器 new 调用 with_context

impl From<Decimal> for DecimalNum {
    fn from(val: Decimal) -> Self {
        DecimalNum::new(val)
    }
}

impl From<i32> for DecimalNum {
    fn from(val: i32) -> Self {
        DecimalNum::new(Decimal::from(val))
    }
}

impl From<i64> for DecimalNum {
    fn from(val: i64) -> Self {
        DecimalNum::new(Decimal::from(val))
    }
}

impl From<u32> for DecimalNum {
    fn from(val: u32) -> Self {
        DecimalNum::new(Decimal::from(val))
    }
}

impl From<u64> for DecimalNum {
    fn from(val: u64) -> Self {
        DecimalNum::new(Decimal::from(val))
    }
}

// --- TryFrom Trait 实现（可能失败的转换）---

impl TryFrom<&str> for DecimalNum {
    type Error = NumError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        DecimalNum::from_str(s)
    }
}

impl TryFrom<String> for DecimalNum {
    type Error = NumError;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        DecimalNum::from_str(&s)
    }
}

// 针对浮点数，带默认上下文构造，NaN检测

impl TryFrom<f32> for DecimalNum {
    type Error = NumError;

    fn try_from(val: f32) -> Result<Self, Self::Error> {
        if val.is_nan() {
            return Err(NumError::InvalidOperation);
        }
        Ok(DecimalNum::new(
            Decimal::from_f32(val).ok_or(NumError::InvalidOperation)?,
        ))
    }
}

impl TryFrom<f64> for DecimalNum {
    type Error = NumError;

    fn try_from(val: f64) -> Result<Self, Self::Error> {
        if val.is_nan() {
            return Err(NumError::InvalidOperation);
        }
        Ok(DecimalNum::new(
            Decimal::from_f64(val).ok_or(NumError::InvalidOperation)?,
        ))
    }
}

impl fmt::Display for DecimalNum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.delegate)
    }
}

impl fmt::Debug for DecimalNum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "DecimalNum({})", self.delegate)
    }
}

impl Add for DecimalNum {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let math_context = self.choose_math_context_with_greater_precision(&rhs);
        Self::with_context(self.delegate + rhs.delegate, math_context)
    }
}

impl<'a> Add for &'a DecimalNum {
    type Output = DecimalNum;

    fn add(self, rhs: Self) -> Self::Output {
        let math_context = self.choose_math_context_with_greater_precision(rhs);
        DecimalNum::with_context(&self.delegate + &rhs.delegate, math_context)
    }
}

impl Sub for DecimalNum {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let math_context = self.choose_math_context_with_greater_precision(&rhs);
        Self::with_context(self.delegate - rhs.delegate, math_context)
    }
}

impl<'a> Sub for &'a DecimalNum {
    type Output = DecimalNum;

    fn sub(self, rhs: Self) -> Self::Output {
        let math_context = self.choose_math_context_with_greater_precision(rhs);
        DecimalNum::with_context(&self.delegate - &rhs.delegate, math_context)
    }
}

impl Mul for DecimalNum {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let math_context = self.choose_math_context_with_greater_precision(&rhs);
        let raw = self.delegate * rhs.delegate;
        Self::with_context(raw, math_context)
    }
}

impl<'a> Mul for &'a DecimalNum {
    type Output = DecimalNum;

    fn mul(self, rhs: Self) -> Self::Output {
        let math_context = self.choose_math_context_with_greater_precision(rhs);
        DecimalNum::with_context(&self.delegate * &rhs.delegate, math_context)
    }
}

impl Div for DecimalNum {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        let ctx = self.choose_math_context_with_greater_precision(&rhs);
        let raw = self.delegate / rhs.delegate;
        Self::with_context(raw, ctx)
    }
}

impl<'a> Div for &'a DecimalNum {
    type Output = DecimalNum;

    fn div(self, rhs: Self) -> Self::Output {
        let math_context = self.choose_math_context_with_greater_precision(rhs);
        DecimalNum::with_context(&self.delegate / &rhs.delegate, math_context)
    }
}

impl Neg for DecimalNum {
    type Output = Self;

    fn neg(self) -> Self::Output {
        let raw = -self.delegate;
        Self::with_context(raw, self.math_context.clone())
    }
}

impl Rem for DecimalNum {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        // 你需要实现自己的逻辑，比如用底层 Decimal 的取余实现
        // 假设你的 Decimal 类型有 rem 方法或者类似
        let result = self.delegate.rem(rhs.delegate);
        Self::with_context(result, self.math_context.clone())
    }
}

// Num trait implementation
impl Num for DecimalNum {
    type FromStrRadixErr = NumError;

    fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        if radix != 10 {
            return Err(NumError::InvalidOperation);
        }

        let decimal = str
            .parse::<Decimal>()
            .map_err(|_| NumError::InvalidOperation)?;

        // 默认 math_context 可统一配置
        let default_ctx = MathContext::default();
        Ok(Self::with_context(decimal, default_ctx))
    }
}

impl Zero for DecimalNum {
    fn zero() -> Self {
        let ctx = MathContext::default();
        Self::with_context(Decimal::ZERO, ctx)
    }

    fn is_zero(&self) -> bool {
        self.delegate.is_zero()
    }
}

impl One for DecimalNum {
    fn one() -> Self {
        let ctx = MathContext::default();
        Self::with_context(Decimal::ONE, ctx)
    }
}

impl ToPrimitive for DecimalNum {
    fn to_i64(&self) -> Option<i64> {
        self.delegate.to_i64()
    }

    fn to_u64(&self) -> Option<u64> {
        self.delegate.to_u64()
    }

    fn to_f64(&self) -> Option<f64> {
        self.delegate.to_f64()
    }
}

impl FromPrimitive for DecimalNum {
    fn from_i64(n: i64) -> Option<Self> {
        let ctx = MathContext::default();
        Decimal::from_i64(n).map(|d| Self::with_context(d, ctx))
    }

    fn from_u64(n: u64) -> Option<Self> {
        let ctx = MathContext::default();
        Decimal::from_u64(n).map(|d| Self::with_context(d, ctx))
    }

    fn from_f64(n: f64) -> Option<Self> {
        let ctx = MathContext::default();
        Decimal::from_f64(n).map(|d| Self::with_context(d, ctx))
    }
}

impl Signed for DecimalNum {
    fn abs(&self) -> Self {
        Self::with_context(self.delegate.abs(), self.math_context.clone())
    }

    fn abs_sub(&self, other: &Self) -> Self {
        if *self <= *other {
            Self::zero()
        } else {
            self.clone() - other.clone()
        }
    }

    fn signum(&self) -> Self {
        Self::with_context(self.delegate.signum(), self.math_context.clone())
    }

    fn is_positive(&self) -> bool {
        self.delegate.is_sign_positive()
    }

    fn is_negative(&self) -> bool {
        self.delegate.is_sign_negative()
    }
}

// TrNum trait implementation
impl TrNum for DecimalNum {
    type Factory = DecimalNumFactory;

    fn get_delegate(&self) -> NumberDelegate {
        NumberDelegate::Decimal(self.delegate)
    }

    fn get_factory(&self) -> Self::Factory {
        let precision = self.math_context.precision;
        DecimalNumFactory::new(precision)
    }

    fn get_name(&self) -> &'static str {
        "DecimalNum"
    }

    fn plus(&self, augend: &Self) -> Self {
        let ctx = self.choose_math_context_with_greater_precision(augend);
        let result = self.delegate - augend.delegate;
        Self::with_context(result, ctx)
    }

    fn minus(&self, subtrahend: &Self) -> Self {
        let ctx = self.choose_math_context_with_greater_precision(subtrahend);
        let result = self.delegate - subtrahend.delegate;
        Self::with_context(result, ctx)
    }

    fn multiplied_by(&self, multiplicand: &Self) -> Self {
        let ctx = self.choose_math_context_with_greater_precision(multiplicand);
        let result = self.delegate * multiplicand.delegate;
        Self::with_context(result, ctx)
    }

    fn divided_by(&self, divisor: &Self) -> Result<Self, NumError> {
        if divisor.is_zero() {
            return Err(NumError::DivisionByZero);
        }
        let ctx = self.choose_math_context_with_greater_precision(divisor);
        let result = self.delegate / divisor.delegate;
        Ok(Self::with_context(result, ctx))
    }

    fn remainder(&self, divisor: &Self) -> Result<Self, NumError> {
        if divisor.is_zero() {
            return Err(NumError::DivisionByZero);
        }
        let val = self.delegate % divisor.delegate;
        Ok(Self::with_context(val, self.math_context.clone()))
    }

    fn floor(&self) -> Self {
        Self::with_context(self.delegate.floor(), self.math_context.clone())
    }

    fn ceil(&self) -> Self {
        Self::with_context(self.delegate.ceil(), self.math_context.clone())
    }

    fn pow(&self, n: i32) -> Result<Self, NumError> {
        let result = self
            .delegate
            .checked_powi(n as i64)
            .ok_or(NumError::InvalidOperation)?;
        Ok(Self::with_context(result, self.math_context.clone()))
    }

    fn pow_num(&self, n: &Self) -> Result<Self, NumError> {
        use num_traits::ToPrimitive; // 显式引入 trait，避免 ambiguous 调用

        let base = self.to_f64().ok_or(NumError::InvalidOperation)?;
        let exp = n.to_f64().ok_or(NumError::InvalidOperation)?;
        if base.is_nan() || exp.is_nan() {
            return Err(NumError::InvalidOperation);
        }

        let res = base.powf(exp);
        let result_decimal = Decimal::from_f64(res).ok_or(NumError::InvalidOperation)?;
        let ctx = self.choose_math_context_with_greater_precision(n);
        Ok(Self::with_context(result_decimal, ctx))
    }

    fn log(&self) -> Result<Self, NumError> {
        if self.delegate <= Decimal::ZERO {
            return Err(NumError::NaN);
        }

        if self.delegate == Decimal::ONE {
            return Ok(Self::with_context(Decimal::ZERO, self.math_context.clone()));
        }

        let iter: u32 = 1000;
        let x = self.clone().sub(DecimalNum::with_context(
            Decimal::ONE,
            self.math_context.clone(),
        ));

        let mut ret = DecimalNum::with_context(Decimal::from(iter + 1), self.math_context.clone());

        for i in (0..=iter).rev() {
            let half_plus_one = DecimalNum::with_context(
                Decimal::from((i / 2 + 1) as u64),
                self.math_context.clone(),
            );

            let pow_result = half_plus_one.pow(2)?;
            let mut n = pow_result.mul_ref(&x);

            ret = n.div(ret);

            n = DecimalNum::with_context(Decimal::from(i + 1), self.math_context.clone());

            ret = ret.add(n);
        }

        ret = x.div(ret);

        Ok(ret)
    }

    fn sqrt(&self) -> Result<Self, NumError> {
        use num_traits::ToPrimitive;

        let val = self.to_f64().ok_or(NumError::InvalidOperation)?;
        if val < 0.0 {
            return Err(NumError::InvalidOperation);
        }
        let sqrt_val = val.sqrt();
        let result_decimal = Decimal::from_f64(sqrt_val).ok_or(NumError::InvalidOperation)?;
        Ok(Self::with_context(
            result_decimal,
            self.math_context.clone(),
        ))
    }

    fn min(&self, other: &Self) -> Self {
        if self <= other {
            self.clone()
        } else {
            other.clone()
        }
    }

    fn max(&self, other: &Self) -> Self {
        if self >= other {
            self.clone()
        } else {
            other.clone()
        }
    }

    fn to_decimal(&self) -> Option<Decimal> {
        Some(self.delegate)
    }
}

// Additional Send + Sync implementations
unsafe impl Send for DecimalNum {}
unsafe impl Sync for DecimalNum {}
