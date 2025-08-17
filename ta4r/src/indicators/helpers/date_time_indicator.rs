use crate::bar::types::{Bar, BarSeries};
use crate::indicators::Indicator;
use crate::indicators::abstract_indicator::BaseIndicator;
use crate::indicators::cached_indicator::CachedIndicator;
use crate::indicators::types::{IndicatorCalculator, IndicatorError};
use crate::num::TrNum;
use std::marker::PhantomData;
use time::OffsetDateTime;

// ------------------- Calculator -------------------
pub struct DateTimeCalculator<S, F> {
    action: F,
    _phantom: PhantomData<S>,
}

impl<S, F: Copy> Clone for DateTimeCalculator<S, F> {
    fn clone(&self) -> Self {
        Self {
            action: self.action,
            _phantom: PhantomData,
        }
    }
}

impl<S, F> DateTimeCalculator<S, F> {
    pub fn new(action: F) -> Self {
        Self {
            action,
            _phantom: PhantomData,
        }
    }
}

// 默认函数：返回 Bar 的 begin_time
pub fn default_get_begin_time<T: TrNum + 'static, B: Bar<T>>(bar: &B) -> OffsetDateTime {
    bar.get_begin_time()
}

impl<'a, T, S, F> IndicatorCalculator<'a, T, S> for DateTimeCalculator<S, F>
where
    T: TrNum + Clone + 'static,
    S: for<'any> BarSeries<'any, T>,
    F: Fn(&<S as BarSeries<'a, T>>::Bar) -> OffsetDateTime + Copy,
{
    type Output = OffsetDateTime;

    fn calculate(
        &self,
        base: &BaseIndicator<'a, T, S>,
        index: usize,
    ) -> Result<Self::Output, IndicatorError> {
        let series = base.get_bar_series();
        let bar = series
            .get_bar(index)
            .ok_or(IndicatorError::OutOfBounds { index })?;
        Ok((self.action)(bar))
    }
}

// ------------------- Indicator -------------------
pub struct DateTimeIndicator<'a, T, S, F>
where
    T: TrNum + Clone + 'static,
    S: for<'any> BarSeries<'any, T>,
    F: Fn(&<S as BarSeries<'a, T>>::Bar) -> OffsetDateTime + Copy,
{
    cached: CachedIndicator<'a, T, S, DateTimeCalculator<S, F>>,
}

impl<'a, T, S, F> Clone for DateTimeIndicator<'a, T, S, F>
where
    T: TrNum + Clone + 'static,
    S: for<'b> BarSeries<'b, T>,
    F: Fn(&<S as BarSeries<'a, T>>::Bar) -> OffsetDateTime + Copy,
    CachedIndicator<'a, T, S, DateTimeCalculator<S, F>>: Clone,
{
    fn clone(&self) -> Self {
        Self {
            cached: self.cached.clone(),
        }
    }
}

impl<'a, T, S, F> DateTimeIndicator<'a, T, S, F>
where
    T: TrNum + Clone + 'static,
    S: for<'any> BarSeries<'any, T>,
    F: Fn(&<S as BarSeries<'a, T>>::Bar) -> OffsetDateTime + Copy,
{
    /// 自定义函数构造
    pub fn with_func(series: &'a S, f: F) -> Self {
        let calculator = DateTimeCalculator::new(f);
        let cached = CachedIndicator::new_from_series(series, calculator);
        Self { cached }
    }
}

impl<'a, T, S> DateTimeIndicator<'a, T, S, fn(&<S as BarSeries<'a, T>>::Bar) -> OffsetDateTime>
where
    T: TrNum + Clone + 'static,
    S: for<'any> BarSeries<'any, T>,
{
    /// 默认构造函数
    pub fn new(series: &'a S) -> Self {
        Self::with_func(
            series,
            default_get_begin_time::<T, <S as BarSeries<'a, T>>::Bar>,
        )
    }
}

impl<'a, T, S, F> Indicator for DateTimeIndicator<'a, T, S, F>
where
    T: TrNum + Clone + 'static,
    S: for<'any> BarSeries<'any, T>,
    F: Fn(&<S as BarSeries<'a, T>>::Bar) -> OffsetDateTime + Copy,
{
    type Num = T;
    type Output = OffsetDateTime;
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
        0
    }
}
