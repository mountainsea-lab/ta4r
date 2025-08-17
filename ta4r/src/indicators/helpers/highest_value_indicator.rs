use crate::bar::types::BarSeries;
use crate::indicators::Indicator;
use crate::indicators::abstract_indicator::BaseIndicator;
use crate::indicators::cached_indicator::CachedIndicator;
use crate::indicators::types::{IndicatorCalculator, IndicatorError};
use crate::num::TrNum;
use std::marker::PhantomData;

/// 实际计算逻辑
pub struct HighestValueCalculator<'a, T, S, I>
where
    T: TrNum + Clone + 'static,
    S: for<'any> BarSeries<'any, T>,
    I: Indicator<Num = T, Output = T, Series<'a> = S>,
{
    indicator: &'a I,
    bar_count: usize,
    _phantom: PhantomData<&'a S>,
}

impl<'a, T, S, I> Clone for HighestValueCalculator<'a, T, S, I>
where
    T: TrNum + Clone + 'static,
    S: for<'b> BarSeries<'b, T>,
    I: Indicator<Num = T, Output = T, Series<'a> = S>,
{
    fn clone(&self) -> Self {
        Self {
            indicator: self.indicator,
            bar_count: self.bar_count,
            _phantom: PhantomData,
        }
    }
}

impl<'a, T, S, I> IndicatorCalculator<'a, T, S> for HighestValueCalculator<'a, T, S, I>
where
    T: TrNum + Clone + 'static,
    S: for<'any> BarSeries<'any, T>,
    I: Indicator<Num = T, Output = T, Series<'a> = S>,
{
    type Output = T;
    fn calculate(
        &self,
        base: &BaseIndicator<'a, T, S>,
        index: usize,
    ) -> Result<Self::Output, IndicatorError> {
        let value = self.indicator.get_value(index)?;

        if value.is_nan() && self.bar_count > 1 {
            let tmp = HighestValueCalculator {
                indicator: self.indicator,
                bar_count: self.bar_count - 1,
                _phantom: PhantomData,
            };
            // 与 Java 一致：递归调用 barCount-1，并且 index-1
            return tmp.calculate(base, index - 1);
        }

        let end = index.saturating_sub(self.bar_count - 1); // Math.max(0, index - barCount + 1)
        let mut highest = value;

        for i in (end..index).rev() {
            let v = self.indicator.get_value(i)?;
            if highest.is_less_than(&v) {
                highest = v;
            }
        }

        Ok(highest)
    }
}
pub struct HighestValueIndicator<'a, T, S, I>
where
    T: TrNum + Clone + 'static,
    S: for<'any> BarSeries<'any, T>,
    I: Indicator<Num = T, Output = T, Series<'a> = S>,
{
    cached: CachedIndicator<'a, T, S, HighestValueCalculator<'a, T, S, I>>,
    bar_count: usize,
}

impl<'a, T, S, I> HighestValueIndicator<'a, T, S, I>
where
    T: TrNum + Clone + 'static,
    S: for<'any> BarSeries<'any, T>,
    I: Indicator<Num = T, Output = T, Series<'a> = S>,
{
    pub fn new(indicator: &'a I, bar_count: usize) -> Self {
        let calculator = HighestValueCalculator {
            indicator,
            bar_count,
            _phantom: PhantomData,
        };
        Self {
            cached: CachedIndicator::new_from_indicator(indicator, calculator),
            bar_count,
        }
    }
}

impl<'a, T, S, I> Clone for HighestValueIndicator<'a, T, S, I>
where
    T: TrNum + Clone + 'static,
    S: for<'b> BarSeries<'b, T>,
    I: Indicator<Num = T, Output = T, Series<'a> = S>,
{
    fn clone(&self) -> Self {
        Self {
            cached: self.cached.clone(),
            bar_count: self.bar_count,
        }
    }
}

impl<'a, T, S, I> Indicator for HighestValueIndicator<'a, T, S, I>
where
    T: TrNum + Clone + 'static,
    S: for<'any> BarSeries<'any, T>,
    I: Indicator<Num = T, Output = T, Series<'a> = S>,
{
    type Num = T;
    type Output = T;
    type Series<'b>
        = S
    where
        Self: 'b;

    fn get_value(&self, index: usize) -> Result<Self::Output, IndicatorError> {
        self.cached.get_cached_value(index)
    }

    fn get_bar_series(&self) -> &Self::Series<'_> {
        self.cached.get_bar_series()
    }

    fn get_count_of_unstable_bars(&self) -> usize {
        self.bar_count
    }
}
