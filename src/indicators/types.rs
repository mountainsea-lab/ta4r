use crate::indicators::Indicator;
use thiserror::Error;

///===========================base sturct types======================
#[derive(Debug, Clone, Error)]
pub enum IndicatorError {
    #[error("Invalid index: {index} (max allowed: {max})")]
    InvalidIndex { index: usize, max: usize },

    #[error("Calculation error: {message}")]
    CalculationError { message: String },

    #[error("Other error: {message}")]
    Other { message: String },
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
