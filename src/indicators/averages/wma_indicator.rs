use crate::bar::types::BarSeries;
use crate::indicators::Indicator;
use crate::indicators::abstract_indicator::BaseIndicator;
use crate::indicators::cached_indicator::CachedIndicator;
use crate::indicators::types::{IndicatorCalculator, IndicatorError};
use crate::num::{NumFactory, TrNum};
use std::marker::PhantomData;

/// WMA (Weighted Moving Average) 的计算器
pub struct WmaCalculator<'a, T, S, I>
where
    T: TrNum + Clone + 'static,
    S: for<'any> BarSeries<'any, T>,
    I: Indicator<Num = T, Series<'a> = S>,
{
    indicator: &'a I,
    bar_count: usize,
    _phantom: PhantomData<(T, S)>,
}

impl<'a, T, S, I> Clone for WmaCalculator<'a, T, S, I>
where
    T: TrNum + Clone + 'static,
    S: for<'any> BarSeries<'any, T>,
    I: Indicator<Num = T, Series<'a> = S>,
{
    fn clone(&self) -> Self {
        Self {
            indicator: self.indicator,
            bar_count: self.bar_count,
            _phantom: PhantomData,
        }
    }
}

impl<'a, T, S, I> WmaCalculator<'a, T, S, I>
where
    T: TrNum + Clone + 'static,
    S: for<'any> BarSeries<'any, T>,
    I: Indicator<Num = T, Series<'a> = S>,
{
    pub fn new(indicator: &'a I, bar_count: usize) -> Self {
        Self {
            indicator,
            bar_count,
            _phantom: PhantomData,
        }
    }
}

impl<'a, T, S, I> IndicatorCalculator<'a, T, S> for WmaCalculator<'a, T, S, I>
where
    T: TrNum + Clone + 'static,
    S: for<'any> BarSeries<'any, T>,
    I: Indicator<Num = T, Series<'a> = S>,
{
    fn calculate(&self, base: &BaseIndicator<'a, T, S>, index: usize) -> Result<T, IndicatorError> {
        let num_factory = base.get_bar_series().num_factory();
        let zero = num_factory.zero().as_ref().clone();

        if index == 0 {
            return self.indicator.get_value(0);
        }

        let loop_len = if index < self.bar_count {
            index + 1
        } else {
            self.bar_count
        };

        let mut weighted_sum = zero.clone();
        let mut actual_index = index;

        for i in (1..=loop_len).rev() {
            let weight = num_factory.num_of_i64(i as i64);
            let value = self.indicator.get_value(actual_index)?;
            weighted_sum = weighted_sum.plus(&weight.multiplied_by(&value));
            actual_index -= 1;
        }

        let denominator = (loop_len * (loop_len + 1)) / 2;

        let denominator = num_factory.num_of_i64(denominator as i64);
        let result = weighted_sum.divided_by(&denominator)?; // 或使用 map_err

        Ok(result)
    }
}

pub struct WmaIndicator<'a, T, S, I>
where
    T: TrNum + Clone + 'static,
    S: for<'any> BarSeries<'any, T>,
    I: Indicator<Num = T, Series<'a> = S>,
{
    cached: CachedIndicator<'a, T, S, WmaCalculator<'a, T, S, I>>,
}

impl<'a, T, S, I> Clone for WmaIndicator<'a, T, S, I>
where
    T: TrNum + Clone + 'static,
    S: for<'any> BarSeries<'any, T>,
    I: Indicator<Num = T, Series<'a> = S>,
{
    fn clone(&self) -> Self {
        Self {
            cached: self.cached.clone(),
        }
    }
}

impl<'a, T, S, I> WmaIndicator<'a, T, S, I>
where
    T: TrNum + Clone + 'static,
    S: for<'any> BarSeries<'any, T>,
    I: Indicator<Num = T, Series<'a> = S>,
{
    pub fn new(indicator: &'a I, bar_count: usize) -> Self {
        let calculator = WmaCalculator::new(indicator, bar_count);
        let cached = CachedIndicator::new_from_indicator(indicator, calculator);
        Self { cached }
    }
}

impl<'a, T, S, I> Indicator for WmaIndicator<'a, T, S, I>
where
    T: TrNum + Clone + 'static,
    S: for<'any> BarSeries<'any, T>,
    I: Indicator<Num = T, Series<'a> = S>,
{
    type Num = T;
    type Series<'b>
        = S
    where
        Self: 'b;

    fn get_value(&self, index: usize) -> Result<T, IndicatorError> {
        self.cached.get_cached_value(index)
    }

    fn get_bar_series(&self) -> &Self::Series<'_> {
        self.cached.get_bar_series()
    }

    fn get_count_of_unstable_bars(&self) -> usize {
        self.cached.calculator().bar_count
    }
}

impl<'a, T, S, I> std::fmt::Display for WmaIndicator<'a, T, S, I>
where
    T: TrNum + Clone + 'static,
    S: for<'any> BarSeries<'any, T> + std::fmt::Debug,
    I: Indicator<Num = T, Series<'a> = S>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "WMA(bar_count={})", self.get_count_of_unstable_bars())
    }
}
