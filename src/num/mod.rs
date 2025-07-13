mod decimal_um;
mod double_num;
mod nan;
mod nan_factory;

use std::fmt::{Debug, Display};
use thiserror::Error;

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

/// 数值工厂trait，用于创建常用的数值实例
pub trait NumFactory<T: Num> {
    fn minus_one() -> T;
    fn zero() -> T;
    fn one() -> T;
    fn two() -> T;
    fn three() -> T;
    fn hundred() -> T;
    fn thousand() -> T;

    fn from_str(s: &str) -> Result<T, NumError>;
    fn from_f64(val: f64) -> Result<T, NumError>;
    fn from_i64(val: i64) -> T;
    fn from_number(number: impl Into<f64>) -> Result<T, NumError>;

    /// 检查数值是否由此工厂创建
    fn produces(&self, num: &T) -> bool;
}

/// 数值计算的核心trait，提供统一的数值操作接口
pub trait Num: Clone + PartialEq + PartialOrd + Debug + Display + Send + Sync {
    type Factory: NumFactory<Self>;

    /// @return factory that created this instance with defined precision
    fn factory(&self) -> Self::Factory;

    /// @return the delegate used from this {@code Num} implementation
    fn name(&self) -> &'static str;

    // 基本算术运算
    fn add(&self, other: &Self) -> Self;
    fn subtract(&self, other: &Self) -> Self;
    fn multiply(&self, other: &Self) -> Self;
    fn divide(&self, other: &Self) -> Result<Self, NumError>;
    fn remainder(&self, other: &Self) -> Result<Self, NumError>;

    // 数学函数
    fn abs(&self) -> Self;
    fn negate(&self) -> Self;
    fn sqrt(&self) -> Result<Self, NumError>;
    fn pow(&self, exp: i32) -> Result<Self, NumError>;
    fn pow_num(&self, exp: &Self) -> Result<Self, NumError>;
    fn log(&self) -> Result<Self, NumError>;
    fn floor(&self) -> Self;
    fn ceil(&self) -> Self;

    // 状态检查
    fn is_zero(&self) -> bool;
    fn is_positive(&self) -> bool;
    fn is_positive_or_zero(&self) -> bool;
    fn is_negative(&self) -> bool;
    fn is_negative_or_zero(&self) -> bool;
    fn is_nan(&self) -> bool;

    // 比较操作
    fn is_equal(&self, other: &Self) -> bool;
    fn is_greater_than(&self, other: &Self) -> bool;
    fn is_greater_than_or_equal(&self, other: &Self) -> bool;
    fn is_less_than(&self, other: &Self) -> bool;
    fn is_less_than_or_equal(&self, other: &Self) -> bool;
    fn min(&self, other: &Self) -> Self;
    fn max(&self, other: &Self) -> Self;

    // 类型转换
    fn to_i32(&self) -> Option<i32>;
    fn to_i64(&self) -> Option<i64>;
    fn to_f32(&self) -> Option<f32>;
    fn to_f64(&self) -> Option<f64>;
    fn to_decimal(&self) -> Option<rust_decimal::Decimal>;
}
