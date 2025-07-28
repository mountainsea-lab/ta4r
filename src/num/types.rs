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

use rust_decimal::RoundingStrategy;
use thiserror::Error;

/// 数学上下文，对应Java的MathContext
#[derive(Debug, Clone, PartialEq)]
pub struct MathContext {
    pub precision: u32,
    pub rounding_mode: RoundingStrategy,
}

impl MathContext {
    pub const DEFAULT_PRECISION: u32 = 32;
    pub fn new(precision: u32, rounding_mode: RoundingStrategy) -> Self {
        Self {
            precision,
            rounding_mode,
        }
    }

    pub fn default() -> Self {
        Self {
            precision: Self::DEFAULT_PRECISION,
            rounding_mode: RoundingStrategy::MidpointAwayFromZero,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum NumberDelegate {
    Int(i64),
    Float(f64),
    Decimal(rust_decimal::Decimal),
    Bool(bool),
    NaN,
    // 可以扩展更多
}

/// 数值计算错误类型
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum NumError {
    #[error("Division by zero")]
    DivisionByZero,

    #[error("Invalid mathematical operation")]
    InvalidOperation,

    #[error("Numeric overflow")]
    Overflow,

    #[error("Parse error: {0}")]
    ParseError(String),

    #[error("Not a Number (NaN)")]
    NaN,

    #[error("Negative number for square root")]
    NegativeSqrt,

    #[error("Logarithm of non-positive number")]
    InvalidLog,
}
