use crate::bar::types::BarSeries;
use crate::indicators::Indicator;
use crate::indicators::abstract_indicator::BaseIndicator;
use crate::indicators::types::IndicatorError;
use crate::num::TrNum;

pub struct FixedIndicator<'a, T, S>
where
    T: TrNum + Clone + 'static,
    S: BarSeries<'a, T>,
{
    base: BaseIndicator<'a, T, S>,
    values: Vec<T>,
}

impl<'a, T, S> Clone for FixedIndicator<'a, T, S>
where
    T: TrNum + Clone + 'static,
    S: BarSeries<'a, T>,
{
    fn clone(&self) -> Self {
        Self {
            base: self.base.clone(),
            values: self.values.clone(), // Vec<T> 需要 T: Clone
        }
    }
}

impl<'a, T, S> FixedIndicator<'a, T, S>
where
    T: TrNum + Clone + 'static,
    S: BarSeries<'a, T>,
{
    pub fn new(series: &'a S, values: Vec<T>) -> Self {
        Self {
            base: BaseIndicator::new(series),
            values,
        }
    }

    pub fn add_value(&mut self, value: T) {
        self.values.push(value);
    }

    pub fn get_base(&self) -> &BaseIndicator<'a, T, S> {
        &self.base
    }
}

impl<'a, T, S> Indicator for FixedIndicator<'a, T, S>
where
    T: TrNum + Clone + 'static,
    S: for<'any> BarSeries<'any, T>,
{
    type Num = T;

    type Series<'b>
        = S
    where
        Self: 'b;

    fn get_value(&self, index: usize) -> Result<T, IndicatorError> {
        self.values
            .get(index)
            .cloned()
            .ok_or_else(|| IndicatorError::OutOfBounds { index })
    }

    fn get_bar_series(&self) -> &Self::Series<'_> {
        self.base.get_bar_series()
    }

    fn get_count_of_unstable_bars(&self) -> usize {
        0
    }
}
