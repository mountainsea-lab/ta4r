#[cfg(test)]
mod tests {
    use super::*;
    use std::marker::PhantomData;
    use ta4r::bar::base_bar_series::BaseBarSeries;
    use ta4r::num::double_num_factory::DoubleNumFactory;
    use tests_indicator::types::{NumKind, TestContext};

    #[test]
    fn test_cache_works() {
        let factory = SMAIndicatorFactory; // 你自己实现的 IndicatorFactory
        let context = TestContext {
            kind: NumKind::Double,
            factory,
            phantom: PhantomData,
        };

        let mut series =
            BaseBarSeries::from_close_prices(DoubleNumFactory(), &[1.0, 2.0, 3.0, 4.0, 3.0]);
        let sma = context.build_indicator(&series, &[3]).unwrap();
        let first = sma.get_value(4);
        let second = sma.get_value(4);
        assert_eq!(first, second);
    }
}
