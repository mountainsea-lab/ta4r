use thiserror::Error;
use crate::bar::types::BarSeries;
use crate::indicators::Indicator;
use crate::num::TrNum;

///===========================base sturct types======================
#[derive(Debug, Clone, Error)]
pub enum IndicatorError {
    InvalidIndex { index: usize, max: usize },
    CalculationError { message: String },
    // ...
}

pub struct AbstractIndicator<'a, N: TrNum> {
    pub bar_series: &'a BarSeries<'a, N>,
}

/// ========================tools=============================
pub type IndicatorResult<T> = Result<T, IndicatorError>;

pub struct IndicatorIterator<'a, I: Indicator + ?Sized> {
    pub(crate) indicator: &'a I,
    pub(crate) index: i32,
    pub(crate) end: i32,
}

impl<'a, I: Indicator + ?Sized> Iterator for IndicatorIterator<'a, I> {
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
