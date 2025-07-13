use rust_decimal::RoundingStrategy;
use rust_decimal::prelude::*;
use std::str::FromStr;

/// 数学上下文，对应Java的MathContext
#[derive(Debug, Clone, PartialEq)]
pub struct MathContext {
    pub precision: u32,
    pub rounding_mode: RoundingStrategy,
}

impl MathContext {
    pub fn new(precision: u32, rounding_mode: RoundingStrategy) -> Self {
        Self {
            precision,
            rounding_mode,
        }
    }

    pub fn default() -> Self {
        Self {
            precision: 32,
            rounding_mode: RoundingStrategy::MidpointNearestEven,
        }
    }
}

/// 高精度数值类型，对应ta4j的DecimalNum
#[derive(Debug, Clone, PartialEq)]
pub struct DecimalNum {
    delegate: Decimal,
    math_context: MathContext,
}

impl DecimalNum {
    pub const DEFAULT_PRECISION: u32 = 32;

    /// 对应ta4j的valueOf方法，默认精度和舍入模式
    pub fn value_of_string(val: &str) -> Result<Self, String> {
        Self::value_of_string_with_context(val, MathContext::default())
    }

    /// valueOf 带 MathContext
    pub fn value_of_string_with_context(
        val: &str,
        math_context: MathContext,
    ) -> Result<Self, String> {
        if val.eq_ignore_ascii_case("nan") {
            return Err("NumberFormatException: NaN is not a valid number".to_string());
        }
        let dec = rust_decimal::Decimal::from_str(val).map_err(|e| e.to_string())?;
        let dec = dec.round_dp_with_strategy(math_context.precision, math_context.rounding_mode);
        Ok(Self {
            delegate: dec,
            math_context,
        })
    }

    /// 返回内部 Decimal
    pub fn inner(&self) -> &rust_decimal::Decimal {
        &self.delegate
    }

    /// 获取 MathContext
    pub fn get_math_context(&self) -> &MathContext {
        &self.math_context
    }

    /// 精度匹配检查，对应 ta4j 的 matches 方法
    /// 判断两数是否在同样的精度下相等（四舍五入后相等）
    pub fn matches(&self, other: &Self, precision: u32) -> bool {
        let rmode = self.math_context.rounding_mode;
        let self_rounded = self.delegate.round_dp_with_strategy(precision, rmode);
        let other_rounded = other.delegate.round_dp_with_strategy(precision, rmode);
        self_rounded == other_rounded
    }

    /// 在偏差范围内匹配检查
    /// 判断 |self - other| <= delta
    pub fn matches_with_delta(&self, other: &Self, delta: &Self) -> bool {
        let diff = if self.delegate > other.delegate {
            self.delegate - other.delegate
        } else {
            other.delegate - self.delegate
        };
        diff <= delta.delegate
    }
}
