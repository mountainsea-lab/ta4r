use crate::indicators::{Indicator, ToNumber};
use crate::num::types::NumError;
use crate::num::{NumFactory, TrNum};
use thiserror::Error;

///===========================base sturct types======================
#[derive(Debug, Clone, Error)]
pub enum IndicatorError {
    #[error("Invalid index: {index} (max allowed: {max})")]
    InvalidIndex { index: usize, max: usize },

    #[error("Calculation error: {message}")]
    CalculationError { message: String },

    #[error("Number error: {0}")]
    NumError(#[from] NumError),

    #[error("Other error: {message}")]
    Other { message: String },
}

// 二元运算符定义
#[derive(Clone, Copy)]
pub enum BinaryOp<T: TrNum> {
    Simple(fn(&T, &T) -> T),
    Fallible(fn(&T, &T) -> Result<T, IndicatorError>),
}

/// 数字包装类型 NumConst<T>
#[derive(Clone, Debug)]
pub struct NumConst<T>(pub T);

/// 转换数字类型的实现，针对包装类型内部具体数字类型的转换
impl<T> ToNumber<T> for NumConst<i64>
where
    T: TrNum + Clone + 'static,
{
    fn to_number(&self, factory: &T::Factory) -> Result<T, NumError> {
        Ok(factory.num_of_i64(self.0))
    }
}

/// 针对 &str 的实现
impl<T> ToNumber<T> for NumConst<&str>
where
    T: TrNum + Clone + 'static,
{
    fn to_number(&self, factory: &T::Factory) -> Result<T, NumError> {
        factory.num_of_str(self.0)
    }
}

/// ========================tools=============================
pub type IndicatorResult<T> = Result<T, IndicatorError>;

pub struct IndicatorIterator<'a, I: Indicator> {
    pub(crate) indicator: &'a I,
    pub(crate) index: usize,
    pub(crate) end: usize,
}

impl<'a, I: Indicator> Iterator for IndicatorIterator<'a, I> {
    type Item = Result<I::Num, IndicatorError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index > self.end {
            None
        } else {
            let result = self.indicator.get_value(self.index);
            self.index += 1;
            Some(result)
        }
    }
}
