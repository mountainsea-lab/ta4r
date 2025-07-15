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

use crate::num::decimal_num_factory::DecimalNumFactory;
use crate::num::double_num::DoubleNum;
use crate::num::{DecimalFactory, DoubleFactory, NumError, NumFactory};

/// This struct implements the `NumFactory` trait and provides convenient
/// access to commonly used constant values.
/// Factory implementation for creating `DoubleNum` instances.
#[derive(Clone, Copy, Debug)]
pub struct DoubleNumFactory;

impl DoubleNumFactory {
    /// Returns the singleton instance of `DoubleNumFactory`.
    pub fn instance() -> Self {
        DoubleNumFactory
    }
}

impl DoubleFactory for DecimalNumFactory {
    type Num = DoubleNum;
    fn num_of_f64(&self, number: impl Into<f64>) -> Result<Self::Num, NumError> {
        Ok(DoubleNum::new(number.into()))
    }
}

impl NumFactory for DoubleNumFactory {
    type Num = DoubleNum;
    type Output = DoubleNum; // 注意：值类型，避免 Arc 开销

    fn minus_one(&self) -> Self::Output {
        DoubleNum::MINUS_ONE
    }

    fn zero(&self) -> Self::Output {
        DoubleNum::ZERO
    }

    fn one(&self) -> Self::Output {
        DoubleNum::ONE
    }

    fn two(&self) -> Self::Output {
        DoubleNum::TWO
    }

    fn three(&self) -> Self::Output {
        DoubleNum::THREE
    }

    fn hundred(&self) -> Self::Output {
        DoubleNum::HUNDRED
    }

    fn thousand(&self) -> Self::Output {
        DoubleNum::THOUSAND
    }

    fn num_of_str(&self, s: &str) -> Result<Self::Num, NumError> {
        DoubleNum::from_str(s)
    }

    fn num_of_i64(&self, val: i64) -> Self::Num {
        DoubleNum::new(val as f64)
    }

    fn produces(&self, _num: &Self::Num) -> bool {
        true
    }
}
