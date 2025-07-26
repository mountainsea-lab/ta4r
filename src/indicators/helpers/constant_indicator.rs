use crate::bar::types::BarSeries;
use crate::indicators::Indicator;
use crate::indicators::abstract_indicator::BaseIndicator;
use crate::indicators::types::IndicatorError;
use crate::num::TrNum;

/// 常数指标：所有索引返回同一个值
pub struct ConstantIndicator<'a, T, S>
where
    T: TrNum + 'static,
    S: BarSeries<'a, T>,
{
    base: BaseIndicator<'a, T, S>,
    value: T,
}

impl<'a, T, S> Clone for ConstantIndicator<'a, T, S>
where
    T: TrNum + Clone + 'static,
    S: BarSeries<'a, T>,
{
    fn clone(&self) -> Self {
        Self {
            base: self.base.clone(),
            value: self.value.clone(),
        }
    }
}

impl<'a, T, S> ConstantIndicator<'a, T, S>
where
    T: TrNum + 'static,
    S: BarSeries<'a, T>,
{
    pub fn new(series: &'a S, value: T) -> Self {
        Self {
            base: BaseIndicator::new(series),
            value,
        }
    }
}

impl<'a, T, S> Indicator for ConstantIndicator<'a, T, S>
where
    T: TrNum + 'static,
    S: for<'any> BarSeries<'any, T>,
{
    type Num = T;
    type Series<'b>
        = S
    where
        Self: 'b;
    fn get_value(&self, _index: usize) -> Result<T, IndicatorError> {
        Ok(self.value.clone())
    }

    fn get_bar_series(&self) -> &Self::Series<'_> {
        self.base.get_bar_series()
    }

    fn get_count_of_unstable_bars(&self) -> usize {
        0
    }
}
