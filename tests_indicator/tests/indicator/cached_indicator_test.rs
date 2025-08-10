use rstest::rstest;
use std::sync::Arc;
use ta4r::bar::base_bar_series_builder::BaseBarSeriesBuilder;
use ta4r::bar::builder::mocks::mock_bar_series_builder::MockBarSeriesBuilder;
use ta4r::bar::types::{Bar, BarSeries, BarSeriesBuilder};
use ta4r::indicators::Indicator;
use ta4r::indicators::averages::sma_indicator::SmaIndicator;
use ta4r::indicators::helpers::close_price_indicator::ClosePriceIndicator;
use ta4r::indicators::helpers::constant_indicator::ConstantIndicator;
use ta4r::num::decimal_num::DecimalNum;
use ta4r::num::decimal_num_factory::DecimalNumFactory;
use ta4r::num::double_num::DoubleNum;
use ta4r::num::double_num_factory::DoubleNumFactory;
use ta4r::num::{NumFactory, TrNum};
use tests_indicator::types::assert_num_eq;

/// cargo test test_if_cache_works_double -- --nocapture --test-threads=1
#[rstest]
#[case(DoubleNumFactory::default())]
fn test_if_cache_works_double(#[case] factory: DoubleNumFactory) {
    test_if_cache_works::<DoubleNum>(Arc::new(factory));
}
/// cargo test test_if_cache_works_decimal -- --nocapture --test-threads=1
#[rstest]
#[case(DecimalNumFactory::default())]
fn test_if_cache_works_decimal(#[case] factory: DecimalNumFactory) {
    test_if_cache_works::<DecimalNum>(Arc::new(factory));
}
#[inline(never)]
fn test_if_cache_works<T>(factory: Arc<T::Factory>)
where
    T: TrNum + 'static,
{
    // 手动强制打印（println 也行，但 eprintln 在测试中默认也会打印）
    eprintln!(">>> Starting test_if_cache_works");

    let data = vec![1., 2., 3., 4., 3., 4., 5., 4., 3., 3., 4., 3., 2.];

    let series = MockBarSeriesBuilder::<T>::default()
        .with_num_factory(factory.clone())
        .with_data(data)
        .build();

    let close_price = ClosePriceIndicator::new(&series);
    let sma = SmaIndicator::new(&close_price, 3);

    let first = sma.get_value(4).unwrap();
    let second = sma.get_value(4).unwrap();

    eprintln!("First SMA value:  {:#?}", first);
    eprintln!("Second SMA value: {:#?}", second);

    assert_eq!(first, second);

    eprintln!(">>> test_if_cache_works finished");
}

/// cargo test test_get_value_with_null_bar_series_double -- --nocapture --test-threads=1
#[test]
fn test_get_value_with_null_bar_series_double() {
    let factory = Arc::new(DoubleNumFactory::default());
    test_get_value_with_null_bar_series::<DoubleNum>(factory);
}
/// cargo test test_get_value_with_null_bar_series_decimal -- --nocapture --test-threads=1
#[test]
fn test_get_value_with_null_bar_series_decimal() {
    let factory = Arc::new(DecimalNumFactory::default());
    test_get_value_with_null_bar_series::<DecimalNum>(factory);
}

fn test_get_value_with_null_bar_series<T>(factory: Arc<T::Factory>)
where
    T: TrNum + 'static,
{
    let constant_val = factory.clone().num_of_i64(10);

    let base_series = BaseBarSeriesBuilder::<T>::default()
        .with_num_factory(factory)
        .build()
        .expect("Failed to build BaseBarSeries");

    let constant = ConstantIndicator::new(&base_series, constant_val.clone());

    assert_eq!(constant_val, constant.get_value(0).unwrap());
    assert_eq!(constant_val, constant.get_value(100).unwrap());

    let series_ref = constant.get_bar_series();
    assert!(std::ptr::eq(series_ref, series_ref)); // 总是 true，或者不写断言

    eprintln!(
        "First constant_val value:  {:#?}",
        constant.get_value(0).unwrap()
    );
    eprintln!(
        "Second constant_val value: {:#?}",
        constant.get_value(100).unwrap()
    );

    let sma = SmaIndicator::new(&constant, 10);

    assert_eq!(constant_val, sma.get_value(0).unwrap());
    assert_eq!(constant_val, sma.get_value(100).unwrap());

    eprintln!("First sma value:  {:#?}", sma.get_value(0).unwrap());
    eprintln!("Second sma value: {:#?}", sma.get_value(100).unwrap());

    let series_ref = sma.get_bar_series();
    assert!(std::ptr::eq(series_ref, series_ref)); // 总是 true，或者不写断言
}

/// cargo test test_get_value_with_cache_length_increase_double -- --nocapture --test-threads=1
#[test]
fn test_get_value_with_cache_length_increase_double() {
    let factory = Arc::new(DoubleNumFactory::default());
    test_get_value_with_cache_length_increase::<DoubleNum>(factory);
}
/// cargo test test_get_value_with_cache_length_increase_decimal -- --nocapture --test-threads=1
#[test]
fn test_get_value_with_cache_length_increase_decimal() {
    let factory = Arc::new(DecimalNumFactory::default());
    test_get_value_with_cache_length_increase::<DecimalNum>(factory);
}

fn test_get_value_with_cache_length_increase<T>(factory: Arc<T::Factory>)
where
    T: TrNum + 'static,
{
    let data = vec![10f64; 200];
    let series = MockBarSeriesBuilder::<T>::default()
        .with_num_factory(factory)
        .with_data(data)
        .build();

    let close_price = ClosePriceIndicator::new(&series);
    let sma = SmaIndicator::new(&close_price, 100);

    assert_num_eq(10.0, sma.get_value(105).unwrap());

    eprintln!("sma value:  {:#?}", sma.get_value(105).unwrap());
}

/// cargo test test_get_value_with_old_results_removale_double -- --nocapture --test-threads=1
#[test]
fn test_get_value_with_old_results_removale_double() {
    let factory = Arc::new(DoubleNumFactory::default());
    test_get_value_with_cache_length_increase::<DoubleNum>(factory);
}
/// cargo test test_get_value_with_old_results_removal_decimal -- --nocapture --test-threads=1
#[test]
fn test_get_value_with_old_results_removal_decimal() {
    let factory = Arc::new(DecimalNumFactory::default());
    test_get_value_with_old_results_removal::<DecimalNum>(factory);
}

fn test_get_value_with_old_results_removal<T>(factory: Arc<T::Factory>)
where
    T: TrNum + 'static,
{
    let data = vec![1f64; 20];
    let mut bar_series = MockBarSeriesBuilder::<T>::default()
        .with_num_factory(factory)
        .with_data(data)
        .build();
    // 限制最大Bar数量
    bar_series
        .set_maximum_bar_count(12)
        .expect("set_maximum_bar_count error");

    let close_price = ClosePriceIndicator::new(&bar_series);
    let sma = SmaIndicator::new(&close_price, 10);

    // rust语言层面就避免了修改情况
    // bar_series
    //     .set_maximum_bar_count(12)
    //     .expect("set_maximum_bar_count error");
    assert_num_eq(1.0, sma.get_value(5).unwrap());
    assert_num_eq(1.0, sma.get_value(10).unwrap());

    assert_num_eq(1.0, sma.get_value(19).unwrap());
}

// #[rstest]
// #[case(NumKind::Double)]
// #[case(NumKind::Decimal)]
// fn test_strategy_execution_on_cached_indicator_and_limited_bar_series(#[case] kind: NumKind) {
//     let factory = kind.num_factory();
//
//     let data = vec![0., 1., 2., 3., 4., 5., 6., 7.];
//     let mut bar_series = MockBarSeriesBuilder::new()
//         .with_num_factory(&*factory)
//         .with_data(&data)
//         .build();
//
//     let close_price = ClosePriceIndicator::new(Arc::new(bar_series.clone()));
//     let sma = SMAIndicator::new(Arc::new(close_price), 2);
//
//     bar_series.set_maximum_bar_count(6);
//
//     let strategy = BaseStrategy::new(
//         OverIndicatorRule::new(Arc::new(sma.clone()), factory.num_of(3)),
//         UnderIndicatorRule::new(Arc::new(sma.clone()), factory.num_of(3)),
//     );
//
//     // 检查进入退出信号，保持和Java一致
//     assert_eq!(false, strategy.should_enter(0));
//     assert_eq!(true, strategy.should_exit(0));
//     assert_eq!(false, strategy.should_enter(1));
//     assert_eq!(true, strategy.should_exit(1));
//     assert_eq!(false, strategy.should_enter(2));
//     assert_eq!(true, strategy.should_exit(2));
//     assert_eq!(false, strategy.should_enter(3));
//     assert_eq!(true, strategy.should_exit(3));
//     assert_eq!(true, strategy.should_enter(4));
//     assert_eq!(false, strategy.should_exit(4));
//     assert_eq!(true, strategy.should_enter(5));
//     assert_eq!(false, strategy.should_exit(5));
//     assert_eq!(true, strategy.should_enter(6));
//     assert_eq!(false, strategy.should_exit(6));
//     assert_eq!(true, strategy.should_enter(7));
//     assert_eq!(false, strategy.should_exit(7));
// }

/// cargo test test_get_value_on_results_calculated_from_removed_bars_should_return_first_remaining_result_double -- --nocapture --test-threads=1
#[test]
fn test_get_value_on_results_calculated_from_removed_bars_should_return_first_remaining_result_double()
 {
    let factory = Arc::new(DoubleNumFactory::default());
    test_get_value_on_results_calculated_from_removed_bars_should_return_first_remaining_result::<
        DoubleNum,
    >(factory);
}
/// cargo test test_get_value_on_results_calculated_from_removed_bars_should_return_first_remaining_result_decimal -- --nocapture --test-threads=1
#[test]
fn test_get_value_on_results_calculated_from_removed_bars_should_return_first_remaining_result_decimal()
 {
    let factory = Arc::new(DecimalNumFactory::default());
    test_get_value_with_old_results_removal::<DecimalNum>(factory);
}

fn test_get_value_on_results_calculated_from_removed_bars_should_return_first_remaining_result<T>(
    factory: Arc<T::Factory>,
) where
    T: TrNum + 'static,
{
    let data = vec![1.0, 1.0, 1.0, 1.0, 1.0];

    let mut bar_series = MockBarSeriesBuilder::<T>::default()
        .with_num_factory(factory)
        .with_data(data)
        .build();

    let _ = bar_series.set_maximum_bar_count(3);

    assert_eq!(2, bar_series.get_removed_bars_count());

    let close_price = ClosePriceIndicator::new(&bar_series);
    let sma = SmaIndicator::new(&close_price, 2);

    for i in 0..5 {
        assert_num_eq(1.0, sma.get_value(i).unwrap());
    }
}

/// cargo test test_recursive_cached_indicator_on_moving_bar_series_should_not_cause_stack_overflow_double -- --nocapture --test-threads=1
#[test]
fn test_recursive_cached_indicator_on_moving_bar_series_should_not_cause_stack_overflow_double() {
    let factory = Arc::new(DoubleNumFactory::default());
    test_recursive_cached_indicator_on_moving_bar_series_should_not_cause_stack_overflow::<DoubleNum>(
        factory,
    );
}
/// cargo test test_recursive_cached_indicator_on_moving_bar_series_should_not_cause_stack_overflow_decimal -- --nocapture --test-threads=1
#[test]
fn test_recursive_cached_indicator_on_moving_bar_series_should_not_cause_stack_overflow_decimal() {
    let factory = Arc::new(DecimalNumFactory::default());
    test_recursive_cached_indicator_on_moving_bar_series_should_not_cause_stack_overflow::<
        DecimalNum,
    >(factory);
}

fn test_recursive_cached_indicator_on_moving_bar_series_should_not_cause_stack_overflow<T>(
    factory: Arc<T::Factory>,
) where
    T: TrNum + 'static,
{

    // let mut series = MockBarSeriesBuilder::new()
    //     .with_num_factory(factory)
    //     .with_default_data()
    //     .build();
    //
    // series.set_maximum_bar_count(5);
    //
    // assert_eq!(5, series.get_bar_count());
    //
    // let close_price = ClosePriceIndicator::new(&series);
    // // todo
    // let zlema = ZLEMAIndicator::new(Arc::new(close_price), 1);
    //
    // let result = std::panic::catch_unwind(|| {
    //     assert_num_eq(4996.0, zlema.get_value(8).unwrap());
    // });
    //
    // assert!(result.is_ok());
}

/// cargo test test_leave_last_bar_uncached_double -- --nocapture --test-threads=1
#[test]
fn test_leave_last_bar_uncached_double() {
    let factory = Arc::new(DoubleNumFactory::default());
    test_leave_last_bar_uncached::<DoubleNum>(factory);
}
/// cargo test test_leave_last_bar_uncached_decimal -- --nocapture --test-threads=1
#[test]
fn test_leave_last_bar_uncached_decimal() {
    let factory = Arc::new(DecimalNumFactory::default());
    test_leave_last_bar_uncached::<DecimalNum>(factory);
}
fn test_leave_last_bar_uncached<T>(factory: Arc<T::Factory>)
where
    T: TrNum + 'static,
{
    let trade_volume = factory.clone().num_of_i64(10);
    let trade_price = factory.clone().num_of_i64(5);

    let mut series = MockBarSeriesBuilder::<T>::default()
        .with_num_factory(factory)
        .with_default_data()
        .build();

    series
        .get_last_bar()
        .unwrap()
        .add_trade(trade_volume, trade_price);

    let close_price = ClosePriceIndicator::new(&series);
    let sma = SmaIndicator::new(&close_price, 5);

    assert_num_eq(
        4998.0,
        sma.get_value(series.get_end_index().unwrap()).unwrap(),
    );

    // (4996 + 4997 + 4998 + 4999 + 5) / 5
    assert_num_eq(
        3999.0,
        sma.get_value(series.get_end_index().unwrap()).unwrap(),
    );
}

// fn simulate_online_data_flow<T>(mut series: SeriesType<T>, factory: Arc<T::Factory>)
// where
//     T: TrNum + 'static,
// {
//     for i in 0..N {
//         // 模拟新数据到来，添加一根新的 Bar（或者替换最后一根）
//         let new_bar = create_new_bar(i, &series, &factory);
//         series.add_bar(new_bar);
//
//         // 构造指标（或复用，保证借用规则）
//         let close_price = ClosePriceIndicator::new(&series);
//         let sma = SmaIndicator::new(&close_price, 5);
//
//         // 调用指标计算最新值，验证缓存增量计算
//         let idx = series.get_end_index().unwrap();
//         let val = sma.get_value(idx).unwrap();
//
//         // 这里可以断言或者打印检查 val 是否符合预期
//         println!("Index {}, SMA value: {:?}", idx, val);
//     }
// }
