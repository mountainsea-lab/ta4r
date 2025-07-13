// /*!
//  * MIT License
//  *
//  * Copyright (c) 2025 Mountainsea
//  * Based on ta4j (c) 2017–2025 Ta4j Organization & respective authors (see AUTHORS)
//  *
//  * Permission is hereby granted, free of charge, to any person obtaining a copy
//  * of this software and associated documentation files (the "Software"), to deal
//  * in the Software without restriction, including without limitation the rights
//  * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
//  * copies of the Software, and to permit persons to whom the Software is
//  * furnished to do so, subject to the following conditions:
//  *
//  * The above copyright notice and this permission notice shall be included in all
//  * copies or substantial portions of the Software.
//  *
//  * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
//  * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
//  * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
//  * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
//  * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
//  * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
//  * SOFTWARE.
//  */
//
// use crate::num::nan_factory::NaNFactory;
// use crate::num::{NumError, TrNum};
// use rust_decimal::Decimal;
// use std::fmt;
// use std::hash::{Hash, Hasher};
// use crate::num::types::NumberDelegate;
//
// /// Not-a-Number 类型，对应 ta4j 的 NaN
// #[derive(Clone, Copy, PartialEq, PartialOrd, Eq)]
// pub struct NaN;
//
// impl NaN {
//     pub fn new() -> Self {
//         NaN
//     }
//
//     /// 永远返回 NaN 实例
//     pub fn value_of<T: Into<f64>>(_val: T) -> Self {
//         NaN
//     }
// }
//
// impl fmt::Debug for NaN {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "NaN")
//     }
// }
//
// impl fmt::Display for NaN {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "NaN")
//     }
// }
//
// impl Default for NaN {
//     fn default() -> Self {
//         NaN
//     }
// }
//
// impl Hash for NaN {
//     fn hash<H: Hasher>(&self, state: &mut H) {
//         "NaN".hash(state);
//     }
// }
//
// // NaN 实现 TrNum
// impl TrNum for NaN {
//     type Factory = NaNFactory;
//
//     fn get_delegate(&self) -> NumberDelegate {
//         NumberDelegate::NaN
//     }
//
//     fn get_factory(&self) -> Self::Factory {
//         NaNFactory
//     }
//
//     fn get_name(&self) -> &'static str {
//         "NaN"
//     }
//
//     fn plus(&self, _augend: &Self) -> Self {
//         NaN
//     }
//
//     fn minus(&self, _subtrahend: &Self) -> Self {
//         NaN
//     }
//
//     fn multiplied_by(&self, _multiplicand: &Self) -> Self {
//         NaN
//     }
//
//     fn divided_by(&self, _divisor: &Self) -> Result<Self, NumError> {
//         Ok(NaN)
//     }
//
//     fn remainder(&self, _divisor: &Self) -> Result<Self, NumError> {
//         Ok(NaN)
//     }
//
//     fn floor(&self) -> Self {
//         NaN
//     }
//
//     fn ceil(&self) -> Self {
//         NaN
//     }
//
//     fn pow(&self, _n: i32) -> Result<Self, NumError> {
//         Ok(NaN)
//     }
//
//     fn pow_num(&self, _n: &Self) -> Result<Self, NumError> {
//         Ok(NaN)
//     }
//
//     fn log(&self) -> Result<Self, NumError> {
//         Ok(NaN)
//     }
//
//     fn sqrt(&self) -> Result<Self, NumError> {
//         Ok(NaN)
//     }
//
//     fn abs(&self) -> Self {
//         NaN
//     }
//
//     fn negate(&self) -> Self {
//         NaN
//     }
//
//     fn is_zero(&self) -> bool {
//         false
//     }
//
//     fn is_positive(&self) -> bool {
//         false
//     }
//
//     fn is_positive_or_zero(&self) -> bool {
//         false
//     }
//
//     fn is_negative(&self) -> bool {
//         false
//     }
//
//     fn is_negative_or_zero(&self) -> bool {
//         false
//     }
//
//     fn is_nan(&self) -> bool {
//         true
//     }
//
//     fn is_equal(&self, other: &Self) -> bool {
//         // 只有两个 NaN 视为相等
//         matches!(other, NaN)
//     }
//
//     fn is_greater_than(&self, _other: &Self) -> bool {
//         false
//     }
//
//     fn is_greater_than_or_equal(&self, _other: &Self) -> bool {
//         false
//     }
//
//     fn is_less_than(&self, _other: &Self) -> bool {
//         false
//     }
//
//     fn is_less_than_or_equal(&self, _other: &Self) -> bool {
//         false
//     }
//
//     fn min(&self, _other: &Self) -> Self {
//         NaN
//     }
//
//     fn max(&self, _other: &Self) -> Self {
//         NaN
//     }
//
//     fn to_i32(&self) -> Option<i32> {
//         None
//     }
//
//     fn to_i64(&self) -> Option<i64> {
//         None
//     }
//
//     fn to_f32(&self) -> Option<f32> {
//         Some(f32::NAN)
//     }
//
//     fn to_f64(&self) -> Option<f64> {
//         Some(f64::NAN)
//     }
//
//     fn to_decimal(&self) -> Option<Decimal> {
//         None
//     }
// }