use crate::bar::types::BarSeries;
use crate::indicators::Indicator;
use crate::indicators::averages::base_ema_indicator::BaseEmaIndicator;
use crate::indicators::types::IndicatorError;
use crate::num::TrNum;

/// 等价于 Java 的 EMAIndicator，封装标准 multiplier 的构造
pub struct EmaIndicator<'a, T, S, I>
where
    T: TrNum + Clone + 'static,
    S: for<'b> BarSeries<'b, T>,
    I: Indicator<Num = T, Series<'a> = S> + 'a,
{
    inner: BaseEmaIndicator<'a, T, S, I>,
}

impl<'a, T, S, I> Clone for EmaIndicator<'a, T, S, I>
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

impl<'a, T, S, I> EmaIndicator<'a, T, S, I>
where
    T: TrNum + Clone + 'static,
    S: for<'b> BarSeries<'b, T>,
    I: Indicator<Num = T, Series<'a> = S> + 'a,
{
    pub fn new(indicator: &'a I, bar_count: usize) -> Self {
        let multiplier = 2.0 / (bar_count as f64 + 1.0);
        let inner = BaseEmaIndicator::new(indicator, bar_count, multiplier as i64);
        Self { inner }
    }

    pub fn bar_count(&self) -> usize {
        self.inner.bar_count()
    }
}

impl<'a, T, S, I> Indicator for EmaIndicator<'a, T, S, I>
where
    T: TrNum + Clone + 'static,
    S: for<'b> BarSeries<'b, T>,
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
        self.bar_count()
    }
}
