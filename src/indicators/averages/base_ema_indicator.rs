use crate::bar::types::BarSeries;
use crate::indicators::Indicator;
use crate::indicators::abstract_indicator::BaseIndicator;
use crate::indicators::recursive_cached_indicator::RecursiveCachedIndicator;
use crate::indicators::types::{IndicatorCalculator, IndicatorError};
use crate::num::{NumFactory, TrNum};
use std::marker::PhantomData;

/// BaseEmaCalculator 持有对 indicator 的引用
pub struct BaseEmaCalculator<'a, T, S, I>
where
    T: TrNum + Clone + 'static,
    S: for<'any> BarSeries<'any, T>,
    I: Indicator<Num = T, Series<'a> = S> + 'a,
{
    pub(crate) indicator: &'a I,
    pub(crate) multiplier: T,
    pub(crate) _phantom: PhantomData<&'a S>,
}

impl<'a, T, S, I> Clone for BaseEmaCalculator<'a, T, S, I>
where
    T: TrNum + Clone + 'static,
    S: for<'any> BarSeries<'any, T>,
    I: Indicator<Num = T, Series<'a> = S> + 'a,
{
    fn clone(&self) -> Self {
        BaseEmaCalculator {
            indicator: self.indicator, // 复制引用即可
            multiplier: self.multiplier.clone(),
            _phantom: PhantomData,
        }
    }
}

impl<'a, T, S, I> IndicatorCalculator<'a, T, S> for BaseEmaCalculator<'a, T, S, I>
where
    T: TrNum + Clone + 'static,
    S: for<'any> BarSeries<'any, T>,
    I: Indicator<Num = T, Series<'a> = S> + 'a,
{
    fn calculate(&self, base: &BaseIndicator<'a, T, S>, index: usize) -> Result<T, IndicatorError> {
        if index == 0 {
            return self.indicator.get_value(0);
        }

        let prev = base.get_value(index - 1)?;
        let current = self.indicator.get_value(index)?;
        let diff = current.clone() - prev.clone();
        Ok(diff * self.multiplier.clone() + prev)
    }
}

/// BaseEmaIndicator 也持有 indicator 的引用
pub struct BaseEmaIndicator<'a, T, S, I>
where
    T: TrNum + Clone + 'static,
    S: for<'any> BarSeries<'any, T>,
    I: Indicator<Num = T, Series<'a> = S> + 'a,
{
    pub(crate) indicator: &'a I,
    pub(crate) bar_count: usize,
    pub(crate) multiplier: T,
    pub(crate) inner: RecursiveCachedIndicator<'a, T, S, BaseEmaCalculator<'a, T, S, I>>,
}

impl<'a, T, S, I> Clone for BaseEmaIndicator<'a, T, S, I>
where
    T: TrNum + Clone + 'static,
    S: for<'any> BarSeries<'any, T>,
    I: Indicator<Num = T, Series<'a> = S> + 'a,
{
    fn clone(&self) -> Self {
        BaseEmaIndicator {
            indicator: self.indicator, // 复制引用
            bar_count: self.bar_count,
            multiplier: self.multiplier.clone(),
            inner: self.inner.clone(),
        }
    }
}

impl<'a, T, S, I> BaseEmaIndicator<'a, T, S, I>
where
    T: TrNum + Clone + 'static,
    S: for<'any> BarSeries<'any, T>,
    I: Indicator<Num = T, Series<'a> = S> + 'a,
{
    /// 构造函数，传入一个 indicator 的引用和 bar_count
    pub fn new(indicator: &'a I, bar_count: usize) -> Self {
        let multiplier = indicator
            .get_bar_series()
            .num_factory()
            .num_of_i64((2.0 / (bar_count as f64 + 1.0)) as i64);

        let calculator = BaseEmaCalculator {
            indicator, // 传引用
            multiplier: multiplier.clone(),
            _phantom: PhantomData,
        };

        let inner = RecursiveCachedIndicator::from_indicator(indicator, calculator);

        Self {
            indicator,
            bar_count,
            multiplier,
            inner,
        }
    }

    pub fn bar_count(&self) -> usize {
        self.bar_count
    }
}

impl<'a, T, S, I> Indicator for BaseEmaIndicator<'a, T, S, I>
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
        self.indicator.get_bar_series()
    }

    fn get_count_of_unstable_bars(&self) -> usize {
        self.bar_count()
    }
}
