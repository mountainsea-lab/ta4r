use crate::num::bool_num::BoolNum;
use crate::num::{DecimalFactory, DoubleFactory, NumError, NumFactory};
use rust_decimal::Decimal;

#[derive(Debug, Clone, Copy)]
pub struct BoolNumFactory;

impl DecimalFactory for BoolNumFactory {
    type Num = BoolNum;

    fn num_of_decimal(&self, number: impl Into<Decimal>) -> Result<Self::Num, NumError> {
        Ok(BoolNum(number.into() != Decimal::ZERO))
    }
}

impl DoubleFactory for BoolNumFactory {
    type Num = BoolNum;

    fn num_of_f64(&self, number: impl Into<f64>) -> Result<Self::Num, NumError> {
        Ok(BoolNum(number.into() != 0.0))
    }
}

impl Default for BoolNumFactory {
    fn default() -> Self {
        BoolNumFactory
    }
}

impl NumFactory<BoolNum> for BoolNumFactory {
    type Output = BoolNum;

    fn minus_one(&self) -> Self::Output {
        BoolNum(true)
    }

    fn zero(&self) -> Self::Output {
        BoolNum(false)
    }

    fn one(&self) -> Self::Output {
        BoolNum(true)
    }

    fn two(&self) -> Self::Output {
        BoolNum(true)
    }

    fn three(&self) -> Self::Output {
        BoolNum(true)
    }

    fn hundred(&self) -> Self::Output {
        BoolNum(true)
    }

    fn thousand(&self) -> Self::Output {
        BoolNum(true)
    }

    fn num_of_str(&self, s: &str) -> Result<BoolNum, NumError> {
        match s {
            "true" | "1" => Ok(BoolNum(true)),
            "false" | "0" => Ok(BoolNum(false)),
            _ => Err(NumError::ParseError(s.to_string())),
        }
    }

    fn num_of_i64(&self, val: i64) -> BoolNum {
        BoolNum(val != 0)
    }

    fn produces(&self, _num: &BoolNum) -> bool {
        true // 所有 BoolNum 都由此 factory 生成
    }
}
