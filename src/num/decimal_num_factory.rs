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

use crate::num::decimal_num::DecimalNum;
use crate::num::types::{MathContext, NumError};
use crate::num::{DecimalFactory, DoubleFactory, NumFactory};
use once_cell::sync::Lazy;
use rust_decimal::{Decimal, RoundingStrategy};
use std::sync::Arc;

// 先定义常量 MathContext
const MC_ONE_PRECISION: MathContext = MathContext {
    precision: 1,
    rounding_mode: RoundingStrategy::MidpointAwayFromZero,
};
const MC_THREE_PRECISION: MathContext = MathContext {
    precision: 3,
    rounding_mode: RoundingStrategy::MidpointAwayFromZero,
};
const MC_FOUR_PRECISION: MathContext = MathContext {
    precision: 4,
    rounding_mode: RoundingStrategy::MidpointAwayFromZero,
};

// 静态常量必须定义在模块层级，不能在 impl 里
pub static MINUS_ONE: Lazy<Arc<DecimalNum>> =
    Lazy::new(|| Arc::new(DecimalNum::with_context(-1, MC_ONE_PRECISION)));
pub static ZERO: Lazy<Arc<DecimalNum>> =
    Lazy::new(|| Arc::new(DecimalNum::with_context(0, MC_ONE_PRECISION)));
pub static ONE: Lazy<Arc<DecimalNum>> =
    Lazy::new(|| Arc::new(DecimalNum::with_context(1, MC_ONE_PRECISION)));
pub static TWO: Lazy<Arc<DecimalNum>> =
    Lazy::new(|| Arc::new(DecimalNum::with_context(2, MC_ONE_PRECISION)));
pub static THREE: Lazy<Arc<DecimalNum>> =
    Lazy::new(|| Arc::new(DecimalNum::with_context(3, MC_ONE_PRECISION)));
pub static HUNDRED: Lazy<Arc<DecimalNum>> =
    Lazy::new(|| Arc::new(DecimalNum::with_context(100, MC_THREE_PRECISION)));
pub static THOUSAND: Lazy<Arc<DecimalNum>> =
    Lazy::new(|| Arc::new(DecimalNum::with_context(1000, MC_FOUR_PRECISION)));

pub struct DecimalNumFactory {
    math_context: MathContext,
}

impl DecimalNumFactory {
    pub const DEFAULT_PRECISION: u32 = 32;

    pub fn new(precision: u32) -> Self {
        let math_context = MathContext {
            precision,
            rounding_mode: RoundingStrategy::MidpointAwayFromZero,
        };

        Self { math_context }
    }

    pub fn instance() -> Self {
        Self::new(Self::DEFAULT_PRECISION)
    }
}

impl DecimalFactory for DecimalNumFactory {
    type Num = DecimalNum;
    fn num_of_decimal(&self, number: impl Into<Decimal>) -> Result<Self::Num, NumError> {
        Ok(DecimalNum::with_context(number, self.math_context.clone()))
    }
}

impl NumFactory<DecimalNum> for DecimalNumFactory {
    type Output = Arc<DecimalNum>;

    fn minus_one(&self) -> Self::Output {
        MINUS_ONE.clone()
    }

    fn zero(&self) -> Self::Output {
        ZERO.clone()
    }

    fn one(&self) -> Self::Output {
        ONE.clone()
    }

    fn two(&self) -> Self::Output {
        TWO.clone()
    }

    fn three(&self) -> Self::Output {
        THREE.clone()
    }

    fn hundred(&self) -> Self::Output {
        HUNDRED.clone()
    }

    fn thousand(&self) -> Self::Output {
        THOUSAND.clone()
    }

    fn num_of_str(&self, s: &str) -> Result<DecimalNum, NumError> {
        DecimalNum::from_str_with_context(s, self.math_context.clone())
    }

    fn num_of_i64(&self, val: i64) -> DecimalNum {
        DecimalNum::with_context(val, self.math_context.clone())
    }

    fn produces(&self, _num: &DecimalNum) -> bool {
        // 这里示意总返回true，你可以自定义判断逻辑
        true
    }
}
