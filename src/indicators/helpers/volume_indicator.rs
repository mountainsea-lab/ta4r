use crate::bar::types::{Bar, BarSeries};
use crate::indicators::Indicator;
use crate::indicators::abstract_indicator::BaseIndicator;
use crate::indicators::cached_indicator::CachedIndicator;
use crate::indicators::types::{IndicatorCalculator, IndicatorError};
use crate::num::{NumFactory, TrNum};
use std::marker::PhantomData;

/// Volume计算器，实现 IndicatorCalculator trait
pub struct VolumeCalculator<T, S> {
    bar_count: usize,
    _phantom: PhantomData<(T, S)>,
}

impl<T, S> Clone for VolumeCalculator<T, S> {
    fn clone(&self) -> Self {
        Self {
            bar_count: self.bar_count,
            _phantom: PhantomData,
        }
    }
}

impl<T, S> VolumeCalculator<T, S> {
    pub fn new(bar_count: usize) -> Self {
        Self {
            bar_count: bar_count.max(1),
            _phantom: PhantomData,
        }
    }
}

impl<'a, T, S> IndicatorCalculator<'a, T, S> for VolumeCalculator<T, S>
where
    T: TrNum + Clone + 'static,
    S: for<'any> BarSeries<'any, T>,
{
    fn calculate(&self, base: &BaseIndicator<'a, T, S>, index: usize) -> Result<T, IndicatorError> {
        let series = base.get_bar_series();
        let start_index = index.saturating_sub(self.bar_count - 1);

        let mut sum = series.num_factory().zero().as_ref().clone();

        for i in start_index..=index {
            let volume = series.get_bar(i).map_or_else(
                || series.num_factory().zero().as_ref().clone(),
                |bar| bar.get_volume(),
            );
            sum = sum.plus(&volume);
        }

        Ok(sum)
    }
}

/// 基于 VolumeCalculator 的 VolumeIndicator
pub struct VolumeIndicator<'a, T, S>
where
    T: TrNum + Clone + 'static,
    S: for<'any> BarSeries<'any, T>,
{
    cached: CachedIndicator<'a, T, S, VolumeCalculator<T, S>>,
}

impl<'a, T, S> Clone for VolumeIndicator<'a, T, S>
where
    T: TrNum + Clone + 'static,
    S: for<'any> BarSeries<'any, T>,
{
    fn clone(&self) -> Self {
        Self {
            cached: self.cached.clone(),
        }
    }
}

impl<'a, T, S> VolumeIndicator<'a, T, S>
where
    T: TrNum + Clone + 'static,
    S: for<'any> BarSeries<'any, T>,
{
    /// 默认构造器，bar_count = 1
    pub fn new(series: &'a S) -> Self {
        Self::with_bar_count(series, 1)
    }

    /// 自定义 bar_count 构造器
    pub fn with_bar_count(series: &'a S, bar_count: usize) -> Self {
        let calculator = VolumeCalculator::new(bar_count);
        let cached = CachedIndicator::new_from_series(series, calculator);
        Self { cached }
    }
}

impl<'a, T, S> Indicator for VolumeIndicator<'a, T, S>
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
        self.cached.get_cached_value(index)
    }

    fn get_bar_series(&self) -> &Self::Series<'_> {
        self.cached.get_bar_series()
    }

    fn get_count_of_unstable_bars(&self) -> usize {
        self.cached.calculator.bar_count
    }
}
