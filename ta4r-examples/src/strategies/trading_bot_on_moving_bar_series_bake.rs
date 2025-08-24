// use crate::loaders::csv_bars_loader::CsvBarsLoader;
// use rand::Rng;
// use std::sync::Arc;
// use ta4r::TradingRecord;
// use ta4r::bar::base_bar::BaseBar;
// use ta4r::bar::base_bar_series_builder::BaseBarSeriesBuilder;
// use ta4r::bar::types::{BarSeries, BarSeriesBuilder};
// use ta4r::base_trading_record::BaseTradingRecord;
// use ta4r::num::decimal_num::DecimalNum;
// use ta4r::num::decimal_num_factory::DecimalNumFactory;
// use ta4r::num::{NumFactory, TrNum};
// use time::{Duration, OffsetDateTime};
//
// /// 全局保存最后一根 Bar 的 close price
// static mut LAST_BAR_CLOSE_PRICE: Option<DecimalNum> = None;
//
// /// 初始化 Moving BarSeries，限制最大 bar 数
// pub fn init_moving_bar_series(max_bar_count: usize) -> BaseBarSeriesBuilder<DecimalNum> {
//     // 加载 CSV 数据，周期为 1 天
//     let mut builder =
//         CsvBarsLoader::load_csv_series("appleinc_bars_from_20130101_usd.csv", Duration::days(1))
//             .expect("Failed to load CSV");
//
//     // 设置最大 bar 数量
//     builder.set_max_bar_count(max_bar_count);
//
//     // 初始化 LAST_BAR_CLOSE_PRICE
//     if let Some(bar) = builder.bars.last() {
//         unsafe {
//             LAST_BAR_CLOSE_PRICE = bar.close_price.as_ref().map(|v| v.clone());
//         }
//     }
//
//     builder
// }
//
// fn generate_random_bar() -> BaseBar<DecimalNum> {
//     use rand::Rng;
//     let factory = DecimalNumFactory::instance();
//     let mut rng = rand::thread_rng();
//
//     // 获取开盘价，使用 LAST_BAR_CLOSE_PRICE 或 factory.one()
//     let open: Arc<DecimalNum> = unsafe {
//         Arc::from(
//             LAST_BAR_CLOSE_PRICE
//                 .clone()
//                 .unwrap_or_else(|| *factory.one()),
//         )
//     };
//
//     // 随机因子 0..1
//     let factor = factory.num_of_f64(rng.gen_range(0.0..1.0));
//
//     // 最大波动范围 3% = 0.03
//     let max_range = factory.num_of_f64(0.03);
//
//     // 计算低价 = open - max_range * factor
//     let low: Arc<DecimalNum> = Arc::new((**open - (*max_range * *factor)).clone());
//
//     // 计算高价 = open + max_range * factor
//     let high: Arc<DecimalNum> = Arc::new((**open + (*max_range * *factor)).clone());
//
//     // 随机收盘价在 low 和 high 之间
//     let low_f64 = (**low).to_f64().unwrap_or(0.0);
//     let high_f64 = (**high).to_f64().unwrap_or(low_f64);
//     let close_value = rng.gen_range(low_f64..=high_f64);
//     let close: Arc<DecimalNum> = Arc::new(factory.num_of_f64(close_value));
//
//     // 更新 LAST_BAR_CLOSE_PRICE
//     unsafe {
//         LAST_BAR_CLOSE_PRICE = Some(close.clone());
//     }
//
//     // 构建 Bar
//     BaseBar::<DecimalNum> {
//         time_period: Duration::days(1),
//         begin_time: OffsetDateTime::now_utc() - Duration::days(1),
//         end_time: OffsetDateTime::now_utc(),
//         open_price: Some(open),
//         high_price: Some(high),
//         low_price: Some(low),
//         close_price: Some(close),
//         volume: factory.num_of_i64(1),
//         amount: None,
//         trades: 0,
//     }
// }
//
// /// Rust 版 Trading Bot
// pub fn trading_bot_on_moving_series() {
//     println!("********************** Initialization **********************");
//
//     let mut builder = init_moving_bar_series(20);
//     let mut series = builder.build().expect("Build series failed");
//
//     // 构建策略，这里假设已有 ClosePriceIndicator + SMAIndicator + BaseStrategy
//     let strategy = buildStrategy(&series);
//
//     // 初始化交易记录
//     let mut trading_record = BaseTradingRecord::default();
//
//     println!("************************************************************");
//
//     // 模拟运行 50 根新 Bar
//     for i in 0..50 {
//         // 随机生成新 Bar
//         let new_bar = generate_random_bar();
//         println!(
//             "------------------------------------------------------\nBar {} added, close price = {}",
//             i,
//             new_bar.close_price.as_ref().unwrap()
//         );
//
//         series.add_bar(new_bar.clone());
//
//         let end_index = series.get_end_index().unwrap();
//
//         if strategy.should_enter(end_index, Some(&trading_record)) {
//             println!("Strategy should ENTER on {}", end_index);
//             let entered = trading_record.enter_with_price_amount(
//                 end_index,
//                 new_bar.close_price.clone().unwrap(),
//                 DecimalNumFactory::instance().num_of_usize(10),
//             );
//             if entered {
//                 let entry = trading_record.last_entry().unwrap();
//                 println!(
//                     "Entered on {} (price={}, amount={})",
//                     entry.index, entry.net_price, entry.amount
//                 );
//             }
//         } else if strategy.should_exit(end_index, Some(&trading_record)) {
//             println!("Strategy should EXIT on {}", end_index);
//             let exited = trading_record.exit_with_price_amount(
//                 end_index,
//                 new_bar.close_price.clone().unwrap(),
//                 DecimalNumFactory::instance().num_of_usize(10),
//             );
//             if exited {
//                 let exit = trading_record.last_exit().unwrap();
//                 println!(
//                     "Exited on {} (price={}, amount={})",
//                     exit.index, exit.net_price, exit.amount
//                 );
//             }
//         }
//     }
// }
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn test_trading_bot() {
//         trading_bot_on_moving_series();
//     }
// }
