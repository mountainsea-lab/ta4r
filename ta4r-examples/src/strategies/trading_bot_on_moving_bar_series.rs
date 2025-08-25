use crate::loaders::csv_bars_loader::CsvBarsLoader;
use num_traits::cast::ToPrimitive;
use parking_lot::RwLock;
use rand::Rng;
use std::sync::Arc;
use ta4r::TradingRecord;
use ta4r::analysis::CostModel;
use ta4r::analysis::cost::fixed_transaction_cost_model::FixedTransactionCostModel;
use ta4r::analysis::cost::zero_cost_model::ZeroCostModel;
use ta4r::bar::base_bar::BaseBar;
use ta4r::bar::base_bar_series_builder::BaseBarSeriesBuilder;
use ta4r::bar::types::{Bar, BarSeries, BarSeriesBuilder};
use ta4r::base_trading_record::BaseTradingRecord;
use ta4r::indicators::averages::sma_indicator::SmaIndicator;
use ta4r::indicators::helpers::close_price_indicator::ClosePriceIndicator;
use ta4r::num::decimal_num::DecimalNum;
use ta4r::num::decimal_num_factory::DecimalNumFactory;
use ta4r::num::{NumFactory, TrNum};
use ta4r::rule::over_indicator_rule::OverIndicatorRule;
use ta4r::rule::under_indicator_rule::UnderIndicatorRule;
use ta4r::strategy::Strategy;
use ta4r::strategy::base_strategy::BaseStrategy;
use time::{Duration, OffsetDateTime};

/// 初始化 Moving BarSeries，限制最大 bar 数
pub fn init_moving_bar_series(max_bar_count: usize) -> BaseBarSeriesBuilder<DecimalNum> {
    let mut builder =
        CsvBarsLoader::load_csv_series("appleinc_bars_from_20130101_usd.csv", Duration::days(1))
            .expect("Failed to load CSV");

    builder.set_max_bar_count(max_bar_count);

    builder
}

/// 随机生成 Bar
// fn generate_random_bar(series: &impl BarSeries<DecimalNum>) -> BaseBar<DecimalNum> {
//     let factory = DecimalNumFactory::instance();
//     let mut rng = rand::thread_rng();
//
//     // 从 series 里获取最后一个 close price，而不是全局变量
//
//     let open: DecimalNum = series
//         .get_last_bar()
//         .and_then(|bar| bar.get_close_price())
//         .unwrap_or_else(|| factory.num_of_usize(1));
//
//     // 随机因子 0..1
//     let factor = factory.num_of_f64(rng.gen_range(0.0..1.0));
//
//     // 最大波动范围 3% = 0.03
//     let max_range = factory.num_of_f64(0.03);
//
//     let low = open.clone() - max_range.clone() * factor.clone();
//     let high = open.clone() + max_range.clone() * factor;
//
//     let close_value = rng.gen_range(low.to_f64().unwrap()..=high.to_f64().unwrap());
//     let close = factory.num_of_f64(close_value);
//
//     BaseBar {
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

/// 随机生成 Bar
fn generate_random_bar(last_close: Option<DecimalNum>) -> BaseBar<DecimalNum> {
    let factory = DecimalNumFactory::instance();
    let mut rng = rand::thread_rng();

    // 上一个 bar 的 close，若无则默认 1
    let open: DecimalNum = last_close.unwrap_or_else(|| factory.num_of_usize(1));

    // 随机因子 0..1
    let factor = factory.num_of_f64(rng.gen_range(0.0..1.0));

    // 最大波动范围 3% = 0.03
    let max_range = factory.num_of_f64(0.03);

    let low = open.clone() - max_range.clone() * factor.clone();
    let high = open.clone() + max_range.clone() * factor;

    let close_value = rng.gen_range(low.to_f64().unwrap()..=high.to_f64().unwrap());
    let close = factory.num_of_f64(close_value);

    BaseBar {
        time_period: Duration::days(1),
        begin_time: OffsetDateTime::now_utc() - Duration::days(1),
        end_time: OffsetDateTime::now_utc(),
        open_price: Some(open),
        high_price: Some(high),
        low_price: Some(low),
        close_price: Some(close),
        volume: factory.num_of_i64(1),
        amount: None,
        trades: 0,
    }
}

pub fn build_strategy<N, Cb, Cs, S, R>(
    series: Arc<RwLock<S>>,
    sma_period: usize,
) -> BaseStrategy<
    N,
    Cb,
    Cs,
    S,
    R,
    OverIndicatorRule<
        N,
        Cb,
        Cs,
        S,
        SmaIndicator<N, S, ClosePriceIndicator<N, S>>,
        ClosePriceIndicator<N, S>,
        R,
    >,
    UnderIndicatorRule<
        N,
        Cb,
        Cs,
        S,
        SmaIndicator<N, S, ClosePriceIndicator<N, S>>,
        ClosePriceIndicator<N, S>,
        R,
    >,
>
where
    N: TrNum + 'static,
    Cb: CostModel<N> + Clone + 'static,
    Cs: CostModel<N> + Clone + 'static,
    S: BarSeries<N> + 'static,
    R: TradingRecord<N, Cb, Cs, S>,
{
    // Create indicators
    let close_price_indicator = Arc::new(ClosePriceIndicator::from_shared(series.clone()));
    let sma_indicator = Arc::new(SmaIndicator::new(close_price_indicator.clone(), sma_period));

    // Create entry and exit rules
    let entry_rule = Arc::new(OverIndicatorRule::new(
        sma_indicator.clone(),
        close_price_indicator.clone(),
    ));
    let exit_rule = Arc::new(UnderIndicatorRule::new(
        sma_indicator,
        close_price_indicator,
    ));

    // Build and return the strategy
    BaseStrategy::default(entry_rule, exit_rule) // Here, we set 1 for unstable_bars just as an example
}

/// Rust 版 Trading Bot
pub fn trading_bot_on_moving_series() {
    println!("********************** Initialization **********************");

    let mut builder = init_moving_bar_series(20);
    let mut series = Arc::new(RwLock::new(builder.build().expect("Build series failed")));

    // 构建策略
    let strategy: BaseStrategy<
        DecimalNum,
        FixedTransactionCostModel<DecimalNum>,
        ZeroCostModel<DecimalNum>,
        _,
        _,
        _,
        _,
    > = build_strategy(series.clone(), 12);
    // 初始化交易记录
    let mut trading_record: BaseTradingRecord<_, _, _, _> = BaseTradingRecord::default();

    println!("************************************************************");

    // 模拟运行 50 根新 Bar
    for i in 0..50 {
        // 只取 last close，读锁立刻释放
        let last_close = {
            let series_locked = series.read();
            series_locked
                .get_last_bar()
                .and_then(|bar| bar.get_close_price())
        };

        // 生成新 bar（不持有锁）
        let new_bar = generate_random_bar(last_close);

        println!(
            "------------------------------------------------------\nBar {} added, close price = {}",
            i,
            new_bar.close_price.as_ref().unwrap()
        );

        // 写锁添加新 bar
        {
            let mut series_locked = series.write();
            series_locked.add_bar(new_bar.clone());
        }

        // 再开一个读锁获取 end_index
        let end_index = {
            let series_locked = series.read();
            series_locked.get_end_index().unwrap()
        };

        if strategy.should_enter(end_index, Some(&trading_record)) {
            println!("Strategy should ENTER on {}", end_index);
            let entered = trading_record.enter_with_price_amount(
                end_index,
                new_bar.close_price.clone().unwrap(),
                DecimalNumFactory::instance().num_of_usize(10),
            );
            if entered {
                let entry = trading_record.last_entry().unwrap();
                println!(
                    "Entered on {} (price={}, amount={})",
                    entry.index, entry.net_price, entry.amount
                );
            }
        } else if strategy.should_exit(end_index, Some(&trading_record)) {
            println!("Strategy should EXIT on {}", end_index);
            let exited = trading_record.exit_with_price_amount(
                end_index,
                new_bar.close_price.clone().unwrap(),
                DecimalNumFactory::instance().num_of_usize(10),
            );
            if exited {
                let exit = trading_record.last_exit().unwrap();
                println!(
                    "Exited on {} (price={}, amount={})",
                    exit.index, exit.net_price, exit.amount
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trading_bot() {
        trading_bot_on_moving_series();
    }
}
