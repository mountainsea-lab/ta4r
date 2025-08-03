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
use crate::num::bool_num_factory::BoolNumFactory;
use crate::num::types::NumberDelegate;
use crate::num::{NumError, TrNum};
use num_traits::{FromPrimitive, Num, One, Signed, ToPrimitive, Zero};
use rust_decimal::Decimal;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::ops::{Add, Div, Mul, Neg, Rem, Sub};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct BoolNum(pub bool);

impl AsRef<BoolNum> for BoolNum {
    fn as_ref(&self) -> &BoolNum {
        self
    }
}

impl From<bool> for BoolNum {
    fn from(b: bool) -> Self {
        BoolNum(b)
    }
}

impl fmt::Debug for BoolNum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "BoolNum({})", self.0)
    }
}

impl fmt::Display for BoolNum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Default for BoolNum {
    fn default() -> Self {
        BoolNum(false)
    }
}

impl Hash for BoolNum {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state)
    }
}

// 可选：实现 Num 相关 trait（用于 TrNum 内部要求）
impl Num for BoolNum {
    type FromStrRadixErr = ();

    fn from_str_radix(s: &str, _radix: u32) -> Result<Self, ()> {
        match s {
            "true" => Ok(BoolNum(true)),
            "false" => Ok(BoolNum(false)),
            _ => Err(()),
        }
    }
}

impl Zero for BoolNum {
    fn zero() -> Self {
        BoolNum(false)
    }
    fn is_zero(&self) -> bool {
        !self.0
    }
}

impl One for BoolNum {
    fn one() -> Self {
        BoolNum(true)
    }
    fn is_one(&self) -> bool {
        self.0
    }
}

impl Signed for BoolNum {
    fn abs(&self) -> Self {
        *self
    }
    fn abs_sub(&self, _other: &Self) -> Self {
        BoolNum(false)
    }
    fn signum(&self) -> Self {
        *self
    }
    fn is_positive(&self) -> bool {
        self.0
    }
    fn is_negative(&self) -> bool {
        false
    }
}

impl FromPrimitive for BoolNum {
    fn from_i64(n: i64) -> Option<Self> {
        Some(BoolNum(n != 0))
    }
    fn from_u64(n: u64) -> Option<Self> {
        Some(BoolNum(n != 0))
    }
    fn from_f64(n: f64) -> Option<Self> {
        Some(BoolNum(n != 0.0))
    }
    fn from_i32(n: i32) -> Option<Self> {
        Some(BoolNum(n != 0))
    }
    fn from_u32(n: u32) -> Option<Self> {
        Some(BoolNum(n != 0))
    }
    fn from_f32(n: f32) -> Option<Self> {
        Some(BoolNum(n != 0.0))
    }
}

impl ToPrimitive for BoolNum {
    fn to_i64(&self) -> Option<i64> {
        Some(self.0 as i64)
    }
    fn to_u64(&self) -> Option<u64> {
        Some(self.0 as u64)
    }
    fn to_f64(&self) -> Option<f64> {
        Some(if self.0 { 1.0 } else { 0.0 })
    }
    fn to_i32(&self) -> Option<i32> {
        Some(self.0 as i32)
    }
    fn to_u32(&self) -> Option<u32> {
        Some(self.0 as u32)
    }
    fn to_f32(&self) -> Option<f32> {
        Some(if self.0 { 1.0 } else { 0.0 })
    }
}

// 基本四则运算可选实现为按位或逻辑处理，或全部返回 false
impl Add for BoolNum {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        BoolNum(self.0 || rhs.0)
    }
}
impl Sub for BoolNum {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        BoolNum(self.0 && !rhs.0)
    }
}
impl Mul for BoolNum {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        BoolNum(self.0 && rhs.0)
    }
}
impl Div for BoolNum {
    type Output = Self;
    fn div(self, _rhs: Self) -> Self::Output {
        self // 或 panic!("not supported")
    }
}
impl Rem for BoolNum {
    type Output = Self;
    fn rem(self, _rhs: Self) -> Self::Output {
        self
    }
}
impl Neg for BoolNum {
    type Output = Self;
    fn neg(self) -> Self::Output {
        BoolNum(!self.0)
    }
}

// ✅ 实现 TrNum
impl TrNum for BoolNum {
    type Factory = BoolNumFactory;

    fn get_delegate(&self) -> NumberDelegate {
        NumberDelegate::Bool(self.0)
    }

    fn get_factory(&self) -> Self::Factory {
        BoolNumFactory
    }

    fn get_name(&self) -> &'static str {
        "BoolNum"
    }

    fn plus(&self, augend: &Self) -> Self {
        *self + *augend
    }

    fn minus(&self, subtrahend: &Self) -> Self {
        *self - *subtrahend
    }

    fn multiplied_by(&self, multiplicand: &Self) -> Self {
        *self * *multiplicand
    }

    fn divided_by(&self, _divisor: &Self) -> Result<Self, NumError> {
        Ok(*self)
    }

    fn remainder(&self, _divisor: &Self) -> Result<Self, NumError> {
        Ok(*self)
    }

    fn floor(&self) -> Self {
        *self
    }

    fn ceil(&self) -> Self {
        *self
    }

    fn pow(&self, _n: i32) -> Result<Self, NumError> {
        Ok(*self)
    }

    fn pow_num(&self, _n: &Self) -> Result<Self, NumError> {
        Ok(*self)
    }

    fn log(&self) -> Result<Self, NumError> {
        Ok(*self)
    }

    fn sqrt(&self) -> Result<Self, NumError> {
        Ok(*self)
    }

    fn is_nan(&self) -> bool {
        false
    }

    fn min(&self, other: &Self) -> Self {
        BoolNum(self.0 & other.0)
    }

    fn max(&self, other: &Self) -> Self {
        BoolNum(self.0 | other.0)
    }

    fn to_decimal(&self) -> Option<Decimal> {
        Some(if self.0 { Decimal::ONE } else { Decimal::ZERO })
    }
}
