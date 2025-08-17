// use std::rc::Rc;
// use std::cell::RefCell;
// use std::time::SystemTime;
// use crate::bar::types::BarSeries;
// use crate::core::{BarSeries, Indicator, CachedIndicator, Bar};
// use crate::indicators::cached_indicator::CachedIndicator;
//
// /// DateTimeIndicator
// ///
// /// 返回某个 bar 的时间戳（默认是 begin_time）。
// pub struct DateTimeIndicator<'a, S>
// where
//     S: BarSeries<'a>,
// {
//     inner: CachedIndicator<'a, SystemTime, S, C>,
// }
//
// impl<'a, S> DateTimeIndicator<'a, S>
// where
//     S: BarSeries<'a>,
// {
//     /// 默认构造函数，返回 bar 的 begin_time
//     pub fn new(series: &'a S) -> Self {
//         let calc = Rc::new(move |index: usize, s: &S| {
//             s.get_bar(index).begin_time()
//         });
//
//         Self {
//             inner: CachedIndicator::new(series, calc),
//         }
//     }
//
//     /// 自定义构造函数，允许指定从 `Bar` 提取时间戳的逻辑
//     pub fn with_action<F>(series: &'a S, action: F) -> Self
//     where
//         F: Fn(&Bar) -> SystemTime + 'static,
//     {
//         let calc = Rc::new(move |index: usize, s: &S| {
//             let bar = s.get_bar(index);
//             action(bar)
//         });
//
//         Self {
//             inner: CachedIndicator::new(series, calc),
//         }
//     }
// }
//
// impl<'a, S> Indicator<'a, SystemTime> for DateTimeIndicator<'a, S>
// where
//     S: BarSeries<'a>,
// {
//     fn get_value(&self, index: usize) -> SystemTime {
//         self.inner.get_value(index)
//     }
//
//     fn get_bar_series(&self) -> &S {
//         self.inner.get_bar_series()
//     }
//
//     fn get_count_of_unstable_bars(&self) -> usize {
//         0
//     }
// }
