// use std::marker::PhantomData;
// use std::time::Instant;
// use crate::bar::types::BarSeries;
// use crate::indicators::abstract_indicator::BaseIndicator;
// use crate::indicators::cached_indicator::CachedIndicator;
// use crate::indicators::Indicator;
// use crate::indicators::types::{IndicatorCalculator, IndicatorError};
// use crate::num::TrNum;
//
// pub struct DateTimeCalculator<S> {
//     _phantom: PhantomData<S>,
// }
//
// impl<S> DateTimeCalculator<S> {
//     pub fn new() -> Self {
//         Self {
//             _phantom: PhantomData,
//         }
//     }
// }
//
// impl<'a, T, S> IndicatorCalculator<'a, T, S> for DateTimeCalculator<S>
// where
//     T: TrNum + Clone + 'static,
//     S: for<'any> BarSeries<'any, T>,
// {
//     type Output = T;
//
//     fn calculate(&self, base: &BaseIndicator<'a, T, S>, index: usize) -> Result<Self::Output, IndicatorError> {
//         let series = base.get_bar_series();
//         let datetime = series
//             .get_bar(index)
//             .ok_or(IndicatorError::IndexOutOfBounds{index})?
//             .get_datetime();
//         Ok(T::from_datetime(datetime)) // 你自己实现 TrNum::from_datetime
//     }
// }
//
// pub struct DateTimeIndicator<'a, T, S>
// where
//     T: TrNum + Clone + 'static,
//     S: for<'any> BarSeries<'any, T>,
// {
//     cached: CachedIndicator<'a, T, S, DateTimeCalculator<S>>,
// }
//
// impl<'a, T, S> DateTimeIndicator<'a, T, S>
// where
//     T: TrNum + Clone + 'static,
//     S: for<'any> BarSeries<'any, T>,
// {
//     pub fn new(series: &'a S) -> Self {
//         let calculator = DateTimeCalculator::new();
//         let cached = CachedIndicator::new_from_series(series, calculator);
//         Self { cached }
//     }
// }
//
// impl<'a, T, S> Indicator for DateTimeIndicator<'a, T, S>
// where
//     T: TrNum + Clone + 'static,
//     S: for<'any> BarSeries<'any, T>,
// {
//     type Num = T;
//     type Output = Instant;
//     type Series<'b> = S
//     where
//         Self: 'b;
//
//     fn get_value(&self, index: usize) -> Result<T, IndicatorError> {
//         self.cached.get_cached_value(index)
//     }
//
//     fn get_bar_series(&self) -> &Self::Series<'_> {
//         self.cached.get_bar_series()
//     }
//
//     fn get_count_of_unstable_bars(&self) -> usize {
//         0
//     }
// }
