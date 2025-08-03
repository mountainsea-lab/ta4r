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

pub mod bool_num;
pub mod bool_num_factory;
pub mod decimal_num;
pub mod decimal_num_factory;
pub mod double_num;
pub mod double_num_factory;
pub mod nan;
pub mod nan_factory;
pub mod types;
mod dashu_num;

use crate::num::types::{NumError, NumberDelegate};
use num_traits::{FromPrimitive, Num, One, Signed, ToPrimitive, Zero};
use rust_decimal::Decimal;
use std::fmt::{Debug, Display};

/// 数值工厂 trait，用于创建常用数值实例，兼容 Arc<T> 或 T
pub trait NumFactory<T: TrNum> {
    type Output: AsRef<T> + Clone;

    fn minus_one(&self) -> Self::Output;
    fn zero(&self) -> Self::Output;
    fn one(&self) -> Self::Output;
    fn two(&self) -> Self::Output;
    fn three(&self) -> Self::Output;
    fn hundred(&self) -> Self::Output;
    fn thousand(&self) -> Self::Output;

    fn num_of_str(&self, s: &str) -> Result<T, NumError>;
    fn num_of_i64(&self, val: i64) -> T;

    /// 检查数值是否由该工厂创建
    fn produces(&self, num: &T) -> bool;
}

pub trait DoubleFactory {
    type Num: TrNum;

    fn num_of_f64(&self, number: impl Into<f64>) -> Result<Self::Num, NumError>;

    // 其它通用方法...
}

pub trait DecimalFactory {
    type Num: TrNum;

    fn num_of_decimal(&self, number: impl Into<Decimal>) -> Result<Self::Num, NumError>;

    // 其它通用方法...
}

/// 数值计算的核心trait 依赖 num_traits，来复用数字基础功能, 提供统一的数值操作接口
pub trait TrNum:
    Num + Clone + PartialOrd + Debug + Display + Send + Sync + ToPrimitive + FromPrimitive + Signed
{
    type Factory: NumFactory<Self> + Default + Debug + Clone + Send + Sync;

    /// 获取底层委托数字
    fn get_delegate(&self) -> NumberDelegate;

    /// 获取对应的数值工厂
    fn get_factory(&self) -> Self::Factory;

    /// 类型名称
    fn get_name(&self) -> &'static str;

    // **必须实现，不提供默认实现**
    fn plus(&self, augend: &Self) -> Self;

    fn minus(&self, subtrahend: &Self) -> Self;

    fn multiplied_by(&self, multiplicand: &Self) -> Self;

    // --- Java版Num接口里没有，但Rust标准trait里也没有的自定义扩展 ---

    /// 除法，可能失败（对应 Java dividedBy）
    fn divided_by(&self, divisor: &Self) -> Result<Self, NumError>;

    // 引用链式扩展
    fn add_ref(&self, other: &Self) -> Self {
        self.plus(other)
    }

    fn sub_ref(&self, other: &Self) -> Self {
        self.minus(other)
    }

    fn multiplied_by_ref(&self, other: &Self) -> Self {
        self.multiplied_by(other)
    }

    fn divided_by_ref(&self, other: &Self) -> Result<Self, NumError> {
        self.divided_by(other)
    }

    /// 取余数，可能失败（对应 Java remainder）
    fn remainder(&self, divisor: &Self) -> Result<Self, NumError>;

    /// 向下取整
    fn floor(&self) -> Self;

    /// 向上取整
    fn ceil(&self) -> Self;

    /// 幂运算（整数次方）
    fn pow(&self, n: i32) -> Result<Self, NumError>;

    /// 幂运算（任意次方，参数为 Self）
    fn pow_num(&self, n: &Self) -> Result<Self, NumError>;

    /// 自然对数
    fn log(&self) -> Result<Self, NumError>;

    /// 平方根
    fn sqrt(&self) -> Result<Self, NumError>;

    // /// 是否为 NaN，默认 false，NaN 类型可覆盖
    fn is_nan(&self) -> bool {
        false
    }

    /// 返回较小值（标准 trait 没有）
    fn min(&self, other: &Self) -> Self;

    /// 返回较大值（标准 trait 没有）
    fn max(&self, other: &Self) -> Self;

    /// 自定义 Decimal 类型转换
    fn to_decimal(&self) -> Option<Decimal>;

    /// 判断是否相等（可自定义精度）
    fn is_equal(&self, other: &Self) -> bool {
        self == other
    }

    /// 是否大于另一个数值
    fn is_greater_than(&self, other: &Self) -> bool {
        self > other
    }
    /// 是否大于或者等于另一个数值
    fn is_greater_than_or_equal(&self, other: &Self) -> bool {
        self >= other
    }
}
