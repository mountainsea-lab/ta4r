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

use std::fmt::{self, Display, Formatter};
use std::ops::{Add, Div, Mul, Neg, Rem, Sub};

use num_traits::{Num, One, Signed, ToPrimitive, Zero};
use rust_decimal::Decimal;
use rust_decimal::prelude::FromPrimitive;

use crate::num::double_num_factory::DoubleNumFactory;
use crate::num::{NumError, TrNum};
use crate::num::types::NumberDelegate;

#[derive(Clone, Copy, PartialEq, PartialOrd, Debug)]
pub struct DoubleNum {
    delegate: f64,
}

impl DoubleNum {
    /// 精度阈值，浮点数比较时使用
    const EPS: f64 = 1e-8;

    /// 常用静态实例
    pub const MINUS_ONE: Self = Self { delegate: -1.0 };
    pub const ZERO: Self = Self { delegate: 0.0 };
    pub const ONE: Self = Self { delegate: 1.0 };
    pub const TWO: Self = Self { delegate: 2.0 };
    pub const THREE: Self = Self { delegate: 3.0 };
    pub const HUNDRED: Self = Self { delegate: 100.0 };
    pub const THOUSAND: Self = Self { delegate: 1000.0 };

    /// 构造新的实例
    #[inline]
    pub fn new(val: f64) -> Self {
        Self { delegate: val }
    }

    /// 从字符串解析为 DoubleNum
    pub fn from_str(val: &str) -> Result<Self, NumError> {
        val.parse::<f64>()
            .map(|v| Self::new(v))
            .map_err(|_| NumError::ParseError(format!("Failed to parse '{}' to f64", val)))
    }

    /// 内部浮点值访问器
    #[inline]
    pub fn inner(&self) -> f64 {
        self.delegate
    }
}

// --- ToPrimitive 实现 ---

impl ToPrimitive for DoubleNum {
    fn to_i32(&self) -> Option<i32> {
        if self.delegate.is_finite() { Some(self.delegate as i32) } else { None }
    }

    fn to_i64(&self) -> Option<i64> {
        if self.delegate.is_finite() { Some(self.delegate as i64) } else { None }
    }

    fn to_u32(&self) -> Option<u32> {
        if self.delegate.is_finite() && self.delegate >= 0.0 {
            Some(self.delegate as u32)
        } else {
            None
        }
    }

    fn to_u64(&self) -> Option<u64> {
        if self.delegate.is_finite() && self.delegate >= 0.0 {
            Some(self.delegate as u64)
        } else {
            None
        }
    }

    fn to_f32(&self) -> Option<f32> {
        if self.delegate.is_finite() { Some(self.delegate as f32) } else { None }
    }

    fn to_f64(&self) -> Option<f64> {
        if self.delegate.is_finite() { Some(self.delegate) } else { None }
    }
}

// --- Zero 实现 ---

impl Zero for DoubleNum {
    fn zero() -> Self {
        Self::ZERO
    }

    fn is_zero(&self) -> bool {
        self.delegate.abs() < Self::EPS
    }
}

// --- One 实现 ---

impl One for DoubleNum {
    fn one() -> Self {
        Self::ONE
    }
}

// --- Num 实现 ---

impl Num for DoubleNum {
    type FromStrRadixErr = NumError;

    fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        if radix != 10 {
            return Err(NumError::ParseError(format!("Only radix 10 supported, got {}", radix)));
        }
        Self::from_str(str)
    }
}

// --- 运算符重载 ---

impl Add for DoubleNum {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        self.plus(&rhs)
    }
}

impl Sub for DoubleNum {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        self.minus(&rhs)
    }
}

impl Mul for DoubleNum {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        self.multiplied_by(&rhs)
    }
}

impl Div for DoubleNum {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        // 保持浮点除法默认行为，允许 Inf 或 NaN
        Self::new(self.delegate / rhs.delegate)
    }
}

// --- Signed 实现 ---

impl Signed for DoubleNum {
    fn abs(&self) -> Self {
        Self::new(self.delegate.abs())
    }

    fn abs_sub(&self, other: &Self) -> Self {
        let diff = self.delegate - other.delegate;
        if diff < 0.0 {
            Self::ZERO
        } else {
            Self::new(diff)
        }
    }

    fn signum(&self) -> Self {
        if self.delegate > 0.0 {
            Self::ONE
        } else if self.delegate < 0.0 {
            Self::MINUS_ONE
        } else {
            Self::ZERO
        }
    }

    fn is_positive(&self) -> bool {
        self.delegate > 0.0
    }

    fn is_negative(&self) -> bool {
        self.delegate < 0.0
    }
}

// --- FromPrimitive 实现 ---

impl FromPrimitive for DoubleNum {
    fn from_i64(n: i64) -> Option<Self> {
        Some(Self::new(n as f64))
    }

    fn from_u64(n: u64) -> Option<Self> {
        Some(Self::new(n as f64))
    }

    fn from_f64(n: f64) -> Option<Self> {
        Some(Self::new(n))
    }

    fn from_i32(n: i32) -> Option<Self> {
        Some(Self::new(n as f64))
    }

    fn from_u32(n: u32) -> Option<Self> {
        Some(Self::new(n as f64))
    }
}

impl Neg for DoubleNum {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.delegate)
    }
}

impl Rem for DoubleNum {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        Self::new(self.delegate % rhs.delegate)
    }
}

// --- Display 实现 ---

impl Display for DoubleNum {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if self.delegate.is_nan() {
            write!(f, "NaN")
        } else {
            write!(f, "{}", self.delegate)
        }
    }
}
impl TrNum for DoubleNum {
    type Factory = DoubleNumFactory;

    #[inline]
    fn get_delegate(&self) -> NumberDelegate {
        NumberDelegate::Float(self.delegate)
    }

    #[inline]
    fn get_factory(&self) -> Self::Factory {
        DoubleNumFactory
    }

    #[inline]
    fn get_name(&self) -> &'static str {
        "DoubleNum"
    }

    #[inline]
    fn plus(&self, augend: &Self) -> Self {
        Self::new(self.delegate + augend.delegate)
    }

    #[inline]
    fn minus(&self, subtrahend: &Self) -> Self {
        Self::new(self.delegate - subtrahend.delegate)
    }

    #[inline]
    fn multiplied_by(&self, multiplicand: &Self) -> Self {
        Self::new(self.delegate * multiplicand.delegate)
    }

    fn divided_by(&self, divisor: &Self) -> Result<Self, NumError> {
        if divisor.delegate.abs() < Self::EPS {
            Err(NumError::DivisionByZero)
        } else {
            Ok(Self::new(self.delegate / divisor.delegate))
        }
    }

    fn remainder(&self, divisor: &Self) -> Result<Self, NumError> {
        if divisor.delegate.abs() < Self::EPS {
            Err(NumError::DivisionByZero)
        } else {
            Ok(Self::new(self.delegate % divisor.delegate))
        }
    }

    #[inline]
    fn floor(&self) -> Self {
        Self::new(self.delegate.floor())
    }

    #[inline]
    fn ceil(&self) -> Self {
        Self::new(self.delegate.ceil())
    }

    fn pow(&self, n: i32) -> Result<Self, NumError> {
        Ok(Self::new(self.delegate.powi(n)))
    }

    fn pow_num(&self, n: &Self) -> Result<Self, NumError> {
        Ok(Self::new(self.delegate.powf(n.delegate)))
    }

    fn log(&self) -> Result<Self, NumError> {
        if self.delegate <= 0.0 {
            Err(NumError::InvalidLog)
        } else {
            Ok(Self::new(self.delegate.ln()))
        }
    }

    fn sqrt(&self) -> Result<Self, NumError> {
        if self.delegate < 0.0 {
            Err(NumError::NegativeSqrt)
        } else {
            Ok(Self::new(self.delegate.sqrt()))
        }
    }

    #[inline]
    fn abs(&self) -> Self {
        Self::new(self.delegate.abs())
    }

    #[inline]
    fn negate(&self) -> Self {
        Self::new(-self.delegate)
    }

    #[inline]
    fn is_nan(&self) -> bool {
        self.delegate.is_nan()
    }

    // 统一调用 ToPrimitive trait
    #[inline]
    fn to_i32(&self) -> Option<i32> {
        ToPrimitive::to_i32(self)
    }

    #[inline]
    fn to_i64(&self) -> Option<i64> {
        ToPrimitive::to_i64(self)
    }

    #[inline]
    fn to_f32(&self) -> Option<f32> {
        ToPrimitive::to_f32(self)
    }

    #[inline]
    fn to_f64(&self) -> Option<f64> {
        ToPrimitive::to_f64(self)
    }

    #[inline]
    fn is_positive_or_zero(&self) -> bool {
        self.delegate >= 0.0
    }

    #[inline]
    fn is_negative_or_zero(&self) -> bool {
        self.delegate <= 0.0
    }

    /// 误差阈值用于浮点比较
    #[inline]
    fn is_equal(&self, other: &Self) -> bool {
        (self.delegate - other.delegate).abs() < Self::EPS
    }

    #[inline]
    fn is_greater_than(&self, other: &Self) -> bool {
        self.delegate > other.delegate + Self::EPS
    }

    #[inline]
    fn is_greater_than_or_equal(&self, other: &Self) -> bool {
        self.is_greater_than(other) || self.is_equal(other)
    }

    #[inline]
    fn is_less_than(&self, other: &Self) -> bool {
        self.delegate < other.delegate - Self::EPS
    }

    #[inline]
    fn is_less_than_or_equal(&self, other: &Self) -> bool {
        self.is_less_than(other) || self.is_equal(other)
    }

    #[inline]
    fn min(&self, other: &Self) -> Self {
        if self.is_less_than(other) { self.clone() } else { other.clone() }
    }

    #[inline]
    fn max(&self, other: &Self) -> Self {
        if self.is_greater_than(other) { self.clone() } else { other.clone() }
    }

    fn to_decimal(&self) -> Option<Decimal> {
        Decimal::from_f64(self.delegate)
    }
}