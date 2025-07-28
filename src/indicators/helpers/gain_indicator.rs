use crate::bar::types::BarSeries;
use crate::indicators::Indicator;
use crate::indicators::abstract_indicator::BaseIndicator;
use crate::indicators::cached_indicator::CachedIndicator;
use crate::indicators::types::{IndicatorCalculator, IndicatorError};
use crate::num::{NumFactory, TrNum};
use std::marker::PhantomData;

/// GainCalculator 调用被包装的 indicator 计算 gain
pub struct GainCalculator<'a, T, S, I>
where
    T: TrNum + Clone + 'static,
    S: for<'any> BarSeries<'any, T>,
    I: Indicator<Num = T, Series<'a> = S>,
{
    indicator: &'a I,
    _phantom: PhantomData<(T, S)>,
}

impl<'a, T, S, I> Clone for GainCalculator<'a, T, S, I>
where
    T: TrNum + Clone + 'static,
    S: for<'any> BarSeries<'any, T>,
    I: Indicator<Num = T, Series<'a> = S>,
{
    fn clone(&self) -> Self {
        Self {
            indicator: self.indicator,
            _phantom: PhantomData,
        }
    }
}

impl<'a, T, S, I> GainCalculator<'a, T, S, I>
where
    T: TrNum + Clone + 'static,
    S: for<'any> BarSeries<'any, T>,
    I: Indicator<Num = T, Series<'a> = S>,
{
    pub fn new(indicator: &'a I) -> Self {
        Self {
            indicator,
            _phantom: PhantomData,
        }
    }
}

impl<'a, T, S, I> IndicatorCalculator<'a, T, S> for GainCalculator<'a, T, S, I>
where
    T: TrNum + Clone + 'static,
    S: for<'any> BarSeries<'any, T>,
    I: Indicator<Num = T, Series<'a> = S>,
{
    fn calculate(&self, base: &BaseIndicator<'a, T, S>, index: usize) -> Result<T, IndicatorError> {
        let zero = base.get_bar_series().num_factory().zero().as_ref().clone();

        if index == 0 {
            return Ok(zero);
        }

        let actual_value = self.indicator.get_value(index)?;
        let previous_value = self.indicator.get_value(index - 1)?;

        if actual_value.is_greater_than(&previous_value) {
            Ok(actual_value.minus(&previous_value))
        } else {
            Ok(zero)
        }
    }
}

/// GainIndicator 组合 CachedIndicator，持有泛型 Indicator
pub struct GainIndicator<'a, T, S, I>
where
    T: TrNum + Clone + 'static,
    S: for<'any> BarSeries<'any, T>,
    I: Indicator<Num = T, Series<'a> = S>,
{
    cached: CachedIndicator<'a, T, S, GainCalculator<'a, T, S, I>>,
}

impl<'a, T, S, I> Clone for GainIndicator<'a, T, S, I>
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

impl<'a, T, S, I> GainIndicator<'a, T, S, I>
where
    T: TrNum + Clone + 'static,
    S: for<'any> BarSeries<'any, T>,
    I: Indicator<Num = T, Series<'a> = S>,
{
    /// 构造函数，传入被包装的 indicator 引用
    pub fn new(indicator: &'a I) -> Self {
        let calculator = GainCalculator::new(indicator);
        let cached = CachedIndicator::new_from_indicator(indicator, calculator);
        Self { cached }
    }
}

impl<'a, T, S, I> Indicator for GainIndicator<'a, T, S, I>
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
        1
    }
}
