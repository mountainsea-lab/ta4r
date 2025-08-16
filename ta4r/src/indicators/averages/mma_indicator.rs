use crate::bar::types::BarSeries;
use crate::indicators::Indicator;
use crate::indicators::averages::base_ema_indicator::BaseEmaIndicator;
use crate::indicators::types::IndicatorError;
use crate::num::{NumFactory, TrNum};

/// Modified moving average indicator (MMA).
///
/// Similar to exponential moving average but smooths more slowly.
/// Used in Welles Wilder's indicators like ADX, RSI.
///
/// Formula: multiplier = 1 / bar_count
pub struct MMAIndicator<'a, T, S, I>
where
    T: TrNum + Clone + 'static,
    S: for<'any> BarSeries<'any, T>,
    I: Indicator<Num = T, Series<'a> = S> + 'a,
{
    inner: BaseEmaIndicator<'a, T, S, I>,
}

impl<'a, T, S, I> Clone for MMAIndicator<'a, T, S, I>
where
    T: TrNum + Clone + 'static,
    S: for<'b> BarSeries<'b, T>,
    I: Indicator<Num = T, Series<'a> = S> + 'a,
{
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<'a, T, S, I> MMAIndicator<'a, T, S, I>
where
    T: TrNum + Clone + 'static,
    S: for<'any> BarSeries<'any, T>,
    I: Indicator<Num = T, Series<'a> = S> + 'a,
{
    /// Constructs a new MMAIndicator
    ///
    /// # Arguments
    /// * `indicator` - the base indicator
    /// * `bar_count` - the MMA time frame
    pub fn new(indicator: &'a I, bar_count: usize) -> Result<Self, IndicatorError> {
        let num_factory = indicator.get_bar_series().num_factory();

        let one = num_factory.one().as_ref().clone();

        let bar_count_t = num_factory.num_of_usize(bar_count).clone();

        let multiplier = one.divided_by(&bar_count_t)?;
        let inner = BaseEmaIndicator::new(indicator, bar_count, multiplier);

        Ok(Self { inner })
    }

    /// Returns the number of unstable bars, equal to the bar count
    pub fn get_count_of_unstable_bars(&self) -> usize {
        self.inner.bar_count()
    }
}

impl<'a, T, S, I> Indicator for MMAIndicator<'a, T, S, I>
where
    T: TrNum + Clone + 'static,
    S: for<'any> BarSeries<'any, T>,
    I: Indicator<Num = T, Series<'a> = S> + 'a,
{
    type Num = T;
    type Series<'b>
        = S
    where
        Self: 'b;

    fn get_value(&self, index: usize) -> Result<T, IndicatorError> {
        self.inner.get_value(index)
    }

    fn get_bar_series(&self) -> &Self::Series<'_> {
        self.inner.get_bar_series()
    }

    fn get_count_of_unstable_bars(&self) -> usize {
        self.inner.get_count_of_unstable_bars()
    }
}
