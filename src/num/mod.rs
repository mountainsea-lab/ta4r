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

mod decimal_um;
mod double_num;
mod nan;
mod nan_factory;
mod double_num_factory;
mod types;

use num_traits::{FromPrimitive, Num, One, Signed, ToPrimitive, Zero};
use std::fmt::{Debug, Display};
use rust_decimal::Decimal;
use crate::num::types::{NumError, NumberDelegate};

/// 数值工厂trait，用于创建常用的数值实例
pub trait NumFactory<T: TrNum> {
    fn minus_one(&self) -> T;
    fn zero(&self) -> T;
    fn one(&self) -> T;
    fn two(&self) -> T;
    fn three(&self) -> T;
    fn hundred(&self) -> T;
    fn thousand(&self) -> T;

    fn num_of_str(&self, s: &str) -> Result<T, NumError>;
    fn num_of_i64(&self, val: i64) -> T;
    fn num_of_f64(&self, number: impl Into<f64>) -> Result<T, NumError>;

    /// 检查数值是否由此工厂创建
    fn produces(&self, num: &T) -> bool;
}

/// 数值计算的核心trait 依赖 num_traits，来复用数字基础功能, 提供统一的数值操作接口
// pub trait TrNum:
// Num + Clone + PartialEq + PartialOrd + Debug + Display + Send + Sync +
// ToPrimitive + FromPrimitive + Signed
// {
//     /// 关联工厂类型
//     type Factory: NumFactory<Self>;
//
//     /// 返回底层数字值，常用 NumberDelegate 表示
//     fn get_delegate(&self) -> NumberDelegate;
//
//     /// 返回创建该数字的工厂
//     fn get_factory(&self) -> Self::Factory;
//
//     /// 返回数字实现的名称描述
//     fn get_name(&self) -> &'static str;
//
//     /// 返回 this + augend
//     fn plus(&self, augend: &Self) -> Self;
//
//     /// 返回 this - subtrahend
//     fn minus(&self, subtrahend: &Self) -> Self;
//
//     /// 返回 this * multiplicand
//     fn multiplied_by(&self, multiplicand: &Self) -> Self;
//
//     /// 返回 this / divisor，可能除零返回错误
//     fn divided_by(&self, divisor: &Self) -> Result<Self, NumError>;
//
//     /// 返回 this % divisor，可能除零返回错误
//     fn remainder(&self, divisor: &Self) -> Result<Self, NumError>;
//
//     /// 向下取整
//     fn floor(&self) -> Self;
//
//     /// 向上取整
//     fn ceil(&self) -> Self;
//
//     /// 返回 this 的 n 次方
//     fn pow(&self, n: i32) -> Result<Self, NumError>;
//
//     /// 返回 this 的 n 次方，n 也是 Num 类型
//     fn pow_num(&self, n: &Self) -> Result<Self, NumError>;
//
//     /// 返回 this 的自然对数
//     fn log(&self) -> Result<Self, NumError>;
//
//     /// 返回 this 的平方根
//     fn sqrt(&self) -> Result<Self, NumError>;
//
//     /// 返回绝对值
//     fn abs(&self) -> Self;
//
//     /// 返回取负数
//     fn negate(&self) -> Self;
//
//     /// 是否是零
//     fn is_zero(&self) -> bool;
//
//     /// 是否是正数
//     fn is_positive(&self) -> bool;
//
//     /// 是否是正数或零
//     fn is_positive_or_zero(&self) -> bool;
//
//     /// 是否是负数
//     fn is_negative(&self) -> bool;
//
//     /// 是否是负数或零
//     fn is_negative_or_zero(&self) -> bool;
//
//     /// 是否是 NaN，默认 false，可特殊实现覆盖
//     fn is_nan(&self) -> bool {
//         false
//     }
//
//     /// 是否等于另一个数字
//     fn is_equal(&self, other: &Self) -> bool;
//
//     /// 是否大于另一个数字
//     fn is_greater_than(&self, other: &Self) -> bool;
//
//     /// 是否大于等于另一个数字
//     fn is_greater_than_or_equal(&self, other: &Self) -> bool;
//
//     /// 是否小于另一个数字
//     fn is_less_than(&self, other: &Self) -> bool;
//
//     /// 是否小于等于另一个数字
//     fn is_less_than_or_equal(&self, other: &Self) -> bool;
//
//     /// 返回较小值
//     fn min(&self, other: &Self) -> Self;
//
//     /// 返回较大值
//     fn max(&self, other: &Self) -> Self;
//
//     /// 转换成 i32，可能失败返回 None
//     fn to_i32(&self) -> Option<i32>;
//
//     /// 转换成 i64，可能失败返回 None
//     fn to_i64(&self) -> Option<i64>;
//
//     /// 转换成 f32，可能失败返回 None
//     fn to_f32(&self) -> Option<f32>;
//
//     /// 转换成 f64，可能失败返回 None
//     fn to_f64(&self) -> Option<f64>;
//
//     /// 转换成 decimal，可能失败返回 None（需要你自己实现 rust_decimal 转换）
//     fn to_decimal(&self) -> Option<rust_decimal::Decimal>;
// }

// pub trait TrNum:
// Num + Clone + PartialEq + PartialOrd + Debug + Display + Send + Sync +
// ToPrimitive + FromPrimitive + Signed
// {
//     type Factory: NumFactory<Self>;
//
//     fn get_delegate(&self) -> NumberDelegate;
//
//     fn get_factory(&self) -> Self::Factory;
//
//     fn get_name(&self) -> &'static str;
//
//     // 自定义的扩展方法，Num/Signed等trait一般没定义这些，所以保留
//     fn divided_by(&self, divisor: &Self) -> Result<Self, NumError>;
//
//     fn remainder(&self, divisor: &Self) -> Result<Self, NumError>;
//
//     fn floor(&self) -> Self;
//
//     fn ceil(&self) -> Self;
//
//     fn pow(&self, n: i32) -> Result<Self, NumError>;
//
//     fn pow_num(&self, n: &Self) -> Result<Self, NumError>;
//
//     fn log(&self) -> Result<Self, NumError>;
//
//     fn sqrt(&self) -> Result<Self, NumError>;
//
//     fn abs(&self) -> Self;
//
//     fn negate(&self) -> Self;
//
//     /// is_nan 不是所有基础 trait 都有，保留，默认 false
//     fn is_nan(&self) -> bool {
//         false
//     }
//
//     // 下面这些判断大小、相等的函数，基础 trait 已有（PartialEq、PartialOrd、Signed）方法，
//     // 可以直接用，故可选实现默认行为，也可以自定义扩展
//     fn is_equal(&self, other: &Self) -> bool {
//         self == other
//     }
//
//     fn is_greater_than(&self, other: &Self) -> bool {
//         self > other
//     }
//
//     fn is_greater_than_or_equal(&self, other: &Self) -> bool {
//         self >= other
//     }
//
//     fn is_less_than(&self, other: &Self) -> bool {
//         self < other
//     }
//
//     fn is_less_than_or_equal(&self, other: &Self) -> bool {
//         self <= other
//     }
//
//     // 返回较小值，基础 trait 没提供，需要保留
//     fn min(&self, other: &Self) -> Self;
//
//     // 返回较大值，基础 trait 没提供，需要保留
//     fn max(&self, other: &Self) -> Self;
//
//     // 下面这些基础 trait 已经提供（Signed 中有 is_zero、is_positive、is_negative）
//     // 可以用默认实现调用 trait 方法，因此不必重复声明
//     fn is_zero(&self) -> bool {
//         Signed::is_zero(self)
//     }
//
//     fn is_positive(&self) -> bool {
//         Signed::is_positive(self)
//     }
//
//     fn is_negative(&self) -> bool {
//         Signed::is_negative(self)
//     }
//
//     // 自定义组合方法，可以保留默认实现
//     fn is_positive_or_zero(&self) -> bool {
//         self.is_zero() || self.is_positive()
//     }
//
//     fn is_negative_or_zero(&self) -> bool {
//         self.is_zero() || self.is_negative()
//         num_traits::Zero::is_zero(&self) || self.is_negative()
//
//     }
//
//     // 转换方法，ToPrimitive trait 已经提供转换，但这里为了统一接口可保留声明
//     fn to_i32(&self) -> Option<i32> {
//         ToPrimitive::to_i32(self)
//     }
//
//     fn to_i64(&self) -> Option<i64> {
//         ToPrimitive::to_i64(self)
//     }
//
//     fn to_f32(&self) -> Option<f32> {
//         ToPrimitive::to_f32(self)
//     }
//
//     fn to_f64(&self) -> Option<f64> {
//         ToPrimitive::to_f64(self)
//     }
//
//     // 这个需要自己实现 rust_decimal 转换，保留
//     fn to_decimal(&self) -> Option<rust_decimal::Decimal>;
//
//     // 以下基本的加减乘法，Num trait 已经有，加一个默认实现即可（不声明为必须实现）
//     fn plus(&self, augend: &Self) -> Self {
//         self.clone() + augend.clone()
//     }
//
//     fn minus(&self, subtrahend: &Self) -> Self {
//         self.clone() - subtrahend.clone()
//     }
//
//     fn multiplied_by(&self, multiplicand: &Self) -> Self {
//         self.clone() * multiplicand.clone()
//     }
// }

pub trait TrNum:
Num + Clone + PartialEq + PartialOrd + Debug + Display + Send + Sync +
ToPrimitive + FromPrimitive + Signed
{
    type Factory: NumFactory<Self>;

    /// 获取底层委托数字，业务定制
    fn get_delegate(&self) -> NumberDelegate;

    /// 获取对应的数值工厂，业务定制
    fn get_factory(&self) -> Self::Factory;

    /// 类型名称
    fn get_name(&self) -> &'static str;

    // --- 自定义扩展，标准 trait 一般不包含的操作 ---

    fn divided_by(&self, divisor: &Self) -> Result<Self, NumError>;

    fn remainder(&self, divisor: &Self) -> Result<Self, NumError>;

    fn floor(&self) -> Self;

    fn ceil(&self) -> Self;

    fn pow(&self, n: i32) -> Result<Self, NumError>;

    fn pow_num(&self, n: &Self) -> Result<Self, NumError>;

    fn log(&self) -> Result<Self, NumError>;

    fn sqrt(&self) -> Result<Self, NumError>;

    /// 负数的绝对值，标准 trait `Signed` 已有 abs()，这里可复用默认实现
    fn abs(&self) -> Self {
        Signed::abs(self)
    }

    /// 取反，Signed trait 已有 `negate`，可以复用
    fn negate(&self) -> Self {
        // -(*self) 必须copy 但不方便扩展
        -(self.clone())
    }

    /// 是否为 NaN，默认 false
    fn is_nan(&self) -> bool {
        false
    }

    // --- 以下是一些标准 trait 已经提供的判断操作，你无需重新声明 ---
    // 你可自定义复合判断，默认实现即可

    fn is_positive_or_zero(&self) -> bool {
        self.is_zero() || self.is_positive()
    }

    fn is_negative_or_zero(&self) -> bool {
        self.is_zero() || self.is_negative()
    }

    // --- 标准 trait 没有的比较组合方法，可保留默认实现 ---

    fn is_equal(&self, other: &Self) -> bool {
        self == other
    }

    fn is_greater_than(&self, other: &Self) -> bool {
        self > other
    }

    fn is_greater_than_or_equal(&self, other: &Self) -> bool {
        self >= other
    }

    fn is_less_than(&self, other: &Self) -> bool {
        self < other
    }

    fn is_less_than_or_equal(&self, other: &Self) -> bool {
        self <= other
    }

    // --- 返回较小/较大值，标准 trait 没有，必须实现 ---

    fn min(&self, other: &Self) -> Self;

    fn max(&self, other: &Self) -> Self;

    // --- 转换相关方法，标准 trait 提供接口，默认实现调用即可 ---

    fn to_i32(&self) -> Option<i32> {
        ToPrimitive::to_i32(self)
    }

    fn to_i64(&self) -> Option<i64> {
        ToPrimitive::to_i64(self)
    }

    fn to_f32(&self) -> Option<f32> {
        ToPrimitive::to_f32(self)
    }

    fn to_f64(&self) -> Option<f64> {
        ToPrimitive::to_f64(self)
    }

    /// 你自己的 Decimal 转换需要自行实现
    fn to_decimal(&self) -> Option<Decimal>;

    // --- 基本算术方法，标准 trait 已定义操作符，可用默认实现 ---

    fn plus(&self, augend: &Self) -> Self {
        self.clone() + augend.clone()
    }

    fn minus(&self, subtrahend: &Self) -> Self {
        self.clone() - subtrahend.clone()
    }

    fn multiplied_by(&self, multiplicand: &Self) -> Self {
        self.clone() * multiplicand.clone()
    }
}
