use crate::bar::types::{Bar, BarSeries};
use crate::indicators::abstract_indicator::BaseIndicator;
use crate::indicators::types::IndicatorError;
use crate::indicators::{Indicator, OptionExt};
use crate::num::TrNum;

/// An indicator that returns the low price of each bar.
pub struct LowPriceIndicator<'a, T, S>
where
    T: TrNum + 'static,
    S: BarSeries<'a, T>,
{
    base: BaseIndicator<'a, T, S>,
}

impl<'a, T, S> Clone for LowPriceIndicator<'a, T, S>
where
    T: TrNum + 'static,
    S: BarSeries<'a, T>,
{
    fn clone(&self) -> Self {
        Self {
            base: self.base.clone(),
        }
    }
}

impl<'a, T, S> LowPriceIndicator<'a, T, S>
where
    T: TrNum + 'static,
    S: BarSeries<'a, T>,
{
    /// Creates a new low price indicator based on the given bar series.
    pub fn new(series: &'a S) -> Self {
        Self {
            base: BaseIndicator::new(series),
        }
    }
}

impl<'a, T, S> Indicator for LowPriceIndicator<'a, T, S>
where
    T: TrNum + 'static,
    S: for<'any> BarSeries<'any, T>,
{
    type Num = T;
    type Series<'b>
        = S
    where
        Self: 'b;

    #[inline]
    fn get_value(&self, index: usize) -> Result<Self::Num, IndicatorError> {
        let series = self.base.get_bar_series();
        let max = series.get_bar_count().saturating_sub(1);
        series
            .get_bar(index)
            .or_invalid_index(index, max)?
            .get_low_price()
            .ok_or_else(|| IndicatorError::CalculationError {
                message: "Missing low price".to_string(),
            })
    }

    #[inline]
    fn get_bar_series(&self) -> &Self::Series<'_> {
        self.base.get_bar_series()
    }

    #[inline]
    fn get_count_of_unstable_bars(&self) -> usize {
        0
    }
}
