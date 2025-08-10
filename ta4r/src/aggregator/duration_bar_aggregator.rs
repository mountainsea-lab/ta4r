use crate::aggregator::BarAggregator;
use crate::aggregator::types::unwrap_or_err;
use crate::bar::base_bar::BaseBar;
use crate::bar::base_bar_series::BaseBarSeries;
use crate::bar::builder::time_bar_builder::TimeBarBuilder;
use crate::bar::types::BarBuilder;
use crate::num::TrNum;
use std::marker::PhantomData;
use std::sync::Arc;
use time::{Duration, OffsetDateTime};

// pub struct DurationBarAggregator<T, BA>
// where
//     T: TrNum + 'static,
//     BA: BarAggregator<T, Bar = BaseBar<T>>,
// {
//     time_period: Duration,
//     only_final_bars: bool,
//     _marker: PhantomData<(T, BA)>,
// }
//
// impl<T, BA> DurationBarAggregator<T, BA>
// where
//     T: TrNum + 'static,
//     BA: BarAggregator<T, Bar = BaseBar<T>>,
// {
//     pub fn new(time_period: Duration, only_final_bars: bool) -> Self {
//         Self {
//             time_period,
//             only_final_bars,
//             _marker: PhantomData,
//         }
//     }
//
//     pub fn begin_times_in_duration(&self, start: OffsetDateTime, current: OffsetDateTime) -> bool {
//         (current - start) < self.time_period
//     }
//
//     pub fn is_in_duration(&self, duration: Duration) -> bool {
//         duration < self.time_period
//     }
// }
//
// impl<T, BA> BarAggregator<T> for DurationBarAggregator<T, BA>
// where
//     T: TrNum + 'static,
//     BA: BarAggregator<T, Bar = BaseBar<T>>,
// {
//     type Bar = BaseBar<T>;
//     fn aggregate(&self, bars: &[BaseBar<T>]) -> Result<Vec<BaseBar<T>>, String> {
//         let mut aggregated = Vec::new();
//         if bars.is_empty() {
//             return Ok(aggregated);
//         }
//
//         // 取第一个 bar 的周期，做倍数检查
//         let actual_dur = bars[0].time_period;
//         if self.time_period.whole_seconds() % actual_dur.whole_seconds() != 0 {
//             return Err("New time_period must be multiple of actual bar time_period".to_string());
//         }
//
//         let zero = T::zero();
//
//         let mut i = 0;
//         while i < bars.len() {
//             let first_bar = &bars[i];
//             let begin_time = first_bar.begin_time;
//             let open = first_bar.open_price.clone();
//             let mut high = first_bar.high_price.clone();
//             let mut low = first_bar.low_price.clone();
//
//             let mut close = None;
//             let mut volume = zero.clone();
//             let mut amount = zero.clone();
//             let mut trades: u64 = 0;
//
//             let mut sum_dur = Duration::ZERO;
//             while self.is_in_duration(sum_dur) && i < bars.len() {
//                 let bar = &bars[i];
//                 if !self.begin_times_in_duration(begin_time, bar.begin_time) {
//                     break;
//                 }
//
//                 // 更新 high / low
//                 if high.is_none() || bar.high_price > high.clone() {
//                     high = bar.high_price.clone();
//                 }
//                 if low.is_none() || bar.low_price < low.clone() {
//                     low = bar.low_price.clone();
//                 }
//                 close = bar.close_price.clone();
//
//                 volume = volume.plus(&bar.volume);
//
//                 if let Some(a) = &bar.amount {
//                     amount = amount.plus(&a.clone());
//                 }
//                 trades += bar.trades;
//
//                 sum_dur = sum_dur + actual_dur;
//                 i += 1;
//             }
//
//             // 只有最终条或者不限制时，才加入结果
//             if !self.only_final_bars || i <= bars.len() {
//                 let open = unwrap_or_err(open, "open price")?;
//                 let high = unwrap_or_err(high, "high price")?;
//                 let low = unwrap_or_err(low, "low price")?;
//                 let close = unwrap_or_err(close, "close price")?;
//
//                 let mut builder = TimeBarBuilder::<T, BaseBarSeries<T>>::new_with_factory(
//                     Arc::new(T::Factory::default()),
//                 );
//                 let builder = builder
//                     .time_period(self.time_period)
//                     .begin_time(begin_time)
//                     .end_time(begin_time + self.time_period)
//                     .open_price(open)
//                     .high_price(high)
//                     .low_price(low)
//                     .close_price(close)
//                     .volume(volume)
//                     .amount(amount)
//                     .trades(trades);
//
//                 let bar = builder.build().map_err(|e| e.to_string())?;
//
//                 aggregated.push(bar);
//             }
//         }
//
//         Ok(aggregated)
//     }
// }

pub struct DurationBarAggregator<T>
where
    T: TrNum + 'static,
{
    time_period: Duration,
    only_final_bars: bool,
    _marker: PhantomData<T>,
}

impl<T> DurationBarAggregator<T>
where
    T: TrNum + 'static,
{
    pub fn new(time_period: Duration, only_final_bars: bool) -> Self {
        Self {
            time_period,
            only_final_bars,
            _marker: PhantomData,
        }
    }

    pub fn begin_times_in_duration(&self, start: OffsetDateTime, current: OffsetDateTime) -> bool {
        (current - start) < self.time_period
    }

    pub fn is_in_duration(&self, duration: Duration) -> bool {
        duration < self.time_period
    }
}

impl<T> BarAggregator<T> for DurationBarAggregator<T>
where
    T: TrNum + 'static,
{
    type Bar = BaseBar<T>;
    fn aggregate(&self, bars: &[BaseBar<T>]) -> Result<Vec<BaseBar<T>>, String> {
        let mut aggregated = Vec::new();
        if bars.is_empty() {
            return Ok(aggregated);
        }

        // 取第一个 bar 的周期，做倍数检查
        let actual_dur = bars[0].time_period;
        if self.time_period.whole_seconds() % actual_dur.whole_seconds() != 0 {
            return Err("New time_period must be multiple of actual bar time_period".to_string());
        }

        let zero = T::zero();

        let mut i = 0;
        while i < bars.len() {
            let first_bar = &bars[i];
            let begin_time = first_bar.begin_time;
            let open = first_bar.open_price.clone();
            let mut high = first_bar.high_price.clone();
            let mut low = first_bar.low_price.clone();

            let mut close = None;
            let mut volume = zero.clone();
            let mut amount = zero.clone();
            let mut trades: u64 = 0;

            let mut sum_dur = Duration::ZERO;
            while self.is_in_duration(sum_dur) && i < bars.len() {
                let bar = &bars[i];
                if !self.begin_times_in_duration(begin_time, bar.begin_time) {
                    break;
                }

                // 更新 high / low
                if high.is_none() || bar.high_price > high.clone() {
                    high = bar.high_price.clone();
                }
                if low.is_none() || bar.low_price < low.clone() {
                    low = bar.low_price.clone();
                }
                close = bar.close_price.clone();

                volume = volume.plus(&bar.volume);

                if let Some(a) = &bar.amount {
                    amount = amount.plus(&a.clone());
                }
                trades += bar.trades;

                sum_dur = sum_dur + actual_dur;
                i += 1;
            }

            // 只有最终条或者不限制时，才加入结果
            if !self.only_final_bars || i <= bars.len() {
                let open = unwrap_or_err(open, "open price")?;
                let high = unwrap_or_err(high, "high price")?;
                let low = unwrap_or_err(low, "low price")?;
                let close = unwrap_or_err(close, "close price")?;

                let mut builder = TimeBarBuilder::<T, BaseBarSeries<T>>::new_with_factory(
                    Arc::new(T::Factory::default()),
                );
                let builder = builder
                    .time_period(self.time_period)
                    .begin_time(begin_time)
                    .end_time(begin_time + self.time_period)
                    .open_price(open)
                    .high_price(high)
                    .low_price(low)
                    .close_price(close)
                    .volume(volume)
                    .amount(amount)
                    .trades(trades);

                let bar = builder.build().map_err(|e| e.to_string())?;

                aggregated.push(bar);
            }
        }

        Ok(aggregated)
    }
}
