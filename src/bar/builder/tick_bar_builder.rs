// use std::time::Duration;
// use crate::bar::base_bar::BaseBar;
// use crate::bar::types::{BarBuilder, BarSeries};
// use crate::num::TrNum;
//
// /// TickBarBuilder 结构体 - 使用泛型参数避免动态分发
// #[derive(Debug, Clone)]
// pub struct TickBarBuilder<T: TrNum, S: BarSeries<T>> {
//     /// 数值工厂
//     num_factory: T::Factory,
//     /// 触发新 Bar 的交易次数阈值
//     tick_count: u64,
//     /// 当前已处理的交易次数
//     passed_ticks_count: u64,
//     /// 绑定的 BarSeries（可选，使用泛型参数）
//     bar_series: Option<S>,
//
//     // Bar 构建字段
//     time_period: Option<Duration>,
//     end_time: Option<SystemTime>,
//     open_price: Option<T>,
//     high_price: Option<T>,
//     low_price: Option<T>,
//     close_price: Option<T>,
//     volume: T,
//     amount: T,
//     trades: u64,
// }
//
// impl<T: TrNum, S: BarSeries<T>> TickBarBuilder<T, S> {
//     /// 创建新的 TickBarBuilder，使用默认数值工厂
//     pub fn new(tick_count: u64) -> TickBarBuilder<T, ()>
//     where
//         T::Factory: Default,
//     {
//         Self::new_with_factory(T::Factory::default(), tick_count)
//     }
//
//     /// 创建新的 TickBarBuilder，指定数值工厂
//     pub fn new_with_factory(num_factory: T::Factory, tick_count: u64) -> TickBarBuilder<T, ()> {
//         let mut builder = TickBarBuilder {
//             num_factory: num_factory.clone(),
//             tick_count,
//             passed_ticks_count: 0,
//             bar_series: None,
//             time_period: None,
//             end_time: None,
//             open_price: None,
//             high_price: None,
//             low_price: None,
//             close_price: None,
//             volume: T::zero(),
//             amount: T::zero(),
//             trades: 0,
//         };
//         builder.reset();
//         builder
//     }
//
//     /// 绑定到 BarSeries，返回新的类型化构建器
//     pub fn bind_to(self, bar_series: S) -> TickBarBuilder<T, S> {
//         TickBarBuilder {
//             num_factory: self.num_factory,
//             tick_count: self.tick_count,
//             passed_ticks_count: self.passed_ticks_count,
//             bar_series: Some(bar_series),
//             time_period: self.time_period,
//             end_time: self.end_time,
//             open_price: self.open_price,
//             high_price: self.high_price,
//             low_price: self.low_price,
//             close_price: self.close_price,
//             volume: self.volume,
//             amount: self.amount,
//             trades: self.trades,
//         }
//     }
//
//     /// 重置构建器状态
//     fn reset(&mut self) {
//         self.time_period = None;
//         self.open_price = None;
//         self.high_price = Some(T::zero());
//         self.low_price = Some(T::from_i64(i64::MAX));
//         self.close_price = None;
//         self.volume = T::zero();
//         self.amount = T::zero();
//         self.trades = 0;
//     }
// }
//
// impl<T: TrNum, S: BarSeries<T>> BarBuilder<T> for TickBarBuilder<T, S> {
//     type Bar = BaseBar<T>;
//
//     fn time_period(mut self, time_period: Duration) -> Self {
//         self.time_period = match self.time_period {
//             Some(existing) => Some(existing + time_period),
//             None => Some(time_period),
//         };
//         self
//     }
//
//     fn end_time(mut self, end_time: SystemTime) -> Self {
//         self.end_time = Some(end_time);
//         self
//     }
//
//     fn open_price(self, _open_price: T) -> Self {
//         panic!("TickBar can only be built from closePrice");
//     }
//
//     fn high_price(self, _high_price: T) -> Self {
//         panic!("TickBar can only be built from closePrice");
//     }
//
//     fn low_price(self, _low_price: T) -> Self {
//         panic!("TickBar can only be built from closePrice");
//     }
//
//     fn close_price(mut self, tick_price: T) -> Self {
//         self.close_price = Some(tick_price);
//
//         if self.open_price.is_none() {
//             self.open_price = Some(tick_price);
//         }
//
//         if let Some(high) = self.high_price {
//             self.high_price = Some(high.max(tick_price));
//         }
//         if let Some(low) = self.low_price {
//             self.low_price = Some(low.min(tick_price));
//         }
//
//         self
//     }
//
//     fn volume(mut self, volume: T) -> Self {
//         self.volume = self.volume.plus(&volume);
//         self
//     }
//
//     fn amount(mut self, amount: T) -> Self {
//         self.amount = amount;
//         self
//     }
//
//     fn trades(mut self, trades: u64) -> Self {
//         self.trades = trades;
//         self
//     }
//
//     fn build(self) -> Result<Self::Bar, String> {
//         BaseBar::new(
//             self.time_period.unwrap_or(Duration::ZERO),
//             self.end_time.unwrap_or(SystemTime::now()),
//             self.open_price,
//             self.high_price,
//             self.low_price,
//             self.close_price,
//             self.volume,
//             self.amount,
//             self.trades,
//         )
//     }
//
//     fn add(mut self) -> Result<(), String> {
//         self.passed_ticks_count += 1;
//
//         if self.passed_ticks_count % self.tick_count == 0 {
//             let bar = self.build()?;
//             if let Some(ref mut series) = self.bar_series {
//                 series.add_bar(bar);
//             }
//             self.reset();
//         }
//
//         Ok(())
//     }
// }
