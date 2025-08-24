// use std::sync::Arc;
// use std::time::Duration;
// use ta4r::bar::base_bar::BaseBar;
// use ta4r::bar::base_bar_series::BaseBarSeries;
// use ta4r::bar::builder::time_bar_builder::TimeBarBuilder;
// use ta4r::bar::types::{BarBuilder, BarSeries};
// use ta4r::base_trading_record::BaseTradingRecord;
// use ta4r::indicators::averages::sma_indicator::SmaIndicator;
// use ta4r::indicators::helpers::close_price_indicator::ClosePriceIndicator;
// use ta4r::num::decimal_num::DecimalNum;
// use ta4r::rule::over_indicator_rule::OverIndicatorRule;
// use ta4r::rule::under_indicator_rule::UnderIndicatorRule;
// use ta4r::strategy::base_strategy::BaseStrategy;
//
// static mut LAST_BAR_CLOSE_PRICE: Option<DecimalNum> = None;
//
// /// 初始化 Moving BarSeries
// fn init_moving_bar_series(max_bar_count: usize) -> BaseBarSeries<DecimalNum, TimeBarBuilder<'static, DecimalNum>> {
//     // 模拟 CSV 加载，直接生成初始序列
//     let mut series = BaseBarSeries::();
//     for _ in 0..max_bar_count {
//         let bar = generate_random_bar();
//         series.add_bar(bar);
//     }
//
//     let last_close = series.last_bar().close_price().clone();
//     unsafe {
//         LAST_BAR_CLOSE_PRICE = Some(last_close);
//     }
//
//     println!("Initialized series with max {} bars, last close = {:?}", max_bar_count, unsafe { LAST_BAR_CLOSE_PRICE.as_ref().unwrap() });
//     series
// }
//
// /// 构建简单策略：SMA 12 vs ClosePrice
// fn build_strategy(series: &BaseBarSeries<DecimalNum, TimeBarBuilder<'static, DecimalNum>>) -> SimpleStrategy<DecimalNum> {
//     let close_price = ClosePriceIndicator::new(series);
//     let sma = SmaIndicator::new(&close_price, 12);
//
//     BaseStrategy::new(
//         OverIndicatorRule::new(sma.clone(), close_price.clone()), // enter
//         UnderIndicatorRule::new(sma, close_price), // exit
//     )
// }
//
// /// 生成随机 DecimalNum
// fn rand_decimal(min: DecimalNum, max: DecimalNum) -> DecimalNum {
//     if min < max {
//         let mut rng = rand::thread_rng();
//         let r: f64 = rng.gen(); // 0..1
//         min + (max - min) * DecimalNum::from_f64(r).unwrap()
//     } else {
//         min
//     }
// }
//
// /// 生成随机 Bar
// fn generate_random_bar() -> BaseBar<DecimalNum> {
//     let max_range = DecimalNum::from_f64(0.03).unwrap();
//     let open_price = unsafe { LAST_BAR_CLOSE_PRICE.clone().unwrap_or_else(|| DecimalNum::one()) };
//     let mut rng = rand::thread_rng();
//     let low_price = open_price - max_range * DecimalNum::from_f64(rng.gen()).unwrap();
//     let high_price = open_price + max_range * DecimalNum::from_f64(rng.gen()).unwrap();
//     let close_price = rand_decimal(low_price, high_price);
//
//     unsafe { LAST_BAR_CLOSE_PRICE = Some(close_price.clone()) }
//
//     TimeBarBuilder::new()
//         .open_price(open_price)
//         .high_price(high_price)
//         .low_price(low_price)
//         .close_price(close_price)
//         .amount(DecimalNum::one())
//         .volume(DecimalNum::one())
//         .time_period(Duration::from_secs(86400))
//         .build()
// }
//
// fn main() {
//     println!("********************** Initialization **********************");
//
//     let mut series = init_moving_bar_series(20);
//     let strategy = build_strategy(&series);
//     let mut trading_record = BaseTradingRecord::default();
//
//     println!("************************************************************");
//
//     for i in 0..50 {
//         let new_bar = generate_random_bar();
//         println!("------------------------------------------------------\nBar {} added, close price = {:?}", i, new_bar.close_price());
//         series.add_bar(new_bar.clone());
//
//         let end_index = series.end_index().unwrap();
//
//         if strategy.should_enter(end_index) {
//             println!("Strategy should ENTER on {}", end_index);
//             let entered = trading_record.enter_with_price_amount(end_index, new_bar.close_price().clone(), DecimalNum::from_f64(10.0).unwrap());
//             if entered {
//                 let entry = trading_record.last_entry().unwrap();
//                 println!("Entered on {} (price={:?}, amount={:?})", entry.index(), entry.net_price(), entry.amount());
//             }
//         } else if strategy.should_exit(end_index) {
//             println!("Strategy should EXIT on {}", end_index);
//             let exited = trading_record.exit_with_price_amount(end_index, new_bar.close_price().clone(), DecimalNum::from_f64(10.0).unwrap());
//             if exited {
//                 let exit = trading_record.last_exit().unwrap();
//                 println!("Exited on {} (price={:?}, amount={:?})", exit.index(), exit.net_price(), exit.amount());
//             }
//         }
//     }
//
//     println!("Final trading record:");
//     trading_record.log_trading_record();
// }
