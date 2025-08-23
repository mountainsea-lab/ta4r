use crate::TradingRecord;
use crate::analysis::CostModel;
use crate::bar::types::BarSeries;
use crate::indicators::Indicator;
use crate::indicators::helpers::constant_indicator::ConstantIndicator;
use crate::num::{NumFactory, TrNum};
use crate::rule::Rule;
use crate::rule::base_rule::BaseRule;
use std::marker::PhantomData;
use std::sync::Arc;

/// OverIndicatorRule: 当第一个指标严格大于第二个指标时满足
pub struct OverIndicatorRule<T, CM, HM, S, I1, I2, R>
where
    T: TrNum + Clone + 'static,
    CM: CostModel<T> + Clone,
    HM: CostModel<T> + Clone,
    S: BarSeries<T> + 'static,
    I1: Indicator<Num = T, Output = T, Series = S> + 'static,
    I2: Indicator<Num = T, Output = T, Series = S> + 'static,
    R: TradingRecord<T, CM, HM, S>,
{
    first: Arc<I1>,
    second: Arc<I2>,
    base_rule: BaseRule<Self>,
    _phantom: PhantomData<(CM, HM, R)>,
}

impl<T, CM, HM, S, I1, I2, R> OverIndicatorRule<T, CM, HM, S, I1, I2, R>
where
    T: TrNum + Clone + 'static,
    CM: CostModel<T> + Clone,
    HM: CostModel<T> + Clone,
    S: BarSeries<T> + 'static,
    I1: Indicator<Num = T, Output = T, Series = S>,
    I2: Indicator<Num = T, Output = T, Series = S>,
    R: TradingRecord<T, CM, HM, S>,
{
    /// Rust 版: OverIndicatorRule(indicator, threshold: TrNum)
    pub fn with_threshold_num(
        indicator: Arc<I1>,
        threshold: T,
    ) -> OverIndicatorRule<T, CM, HM, S, I1, ConstantIndicator<T, S>, R> {
        let constant = Arc::new(ConstantIndicator::new(indicator.bar_series(), threshold));
        OverIndicatorRule {
            first: indicator,
            second: constant,
            base_rule: BaseRule::new("OverIndicatorRule"),
            _phantom: PhantomData,
        }
    }

    /// Rust 版: OverIndicatorRule(indicator, threshold: f64)
    pub fn with_threshold_f64(
        indicator: Arc<I1>,
        threshold: f64,
    ) -> OverIndicatorRule<T, CM, HM, S, I1, ConstantIndicator<T, S>, R> {
        let num_factory = indicator
            .bar_series()
            .with_ref(|s| s.num_factory())
            .expect("num_factory get failed");
        let num = num_factory.num_of_f64(threshold);

        Self::with_threshold_num(indicator, num)
    }

    /// Rust 版: OverIndicatorRule(first, second)
    pub fn new(first: Arc<I1>, second: Arc<I2>) -> Self {
        Self {
            first,
            second,
            base_rule: BaseRule::new("OverIndicatorRule"),
            _phantom: PhantomData,
        }
    }
}

impl<T, CM, HM, S, I1, I2, R> Rule for OverIndicatorRule<T, CM, HM, S, I1, I2, R>
where
    T: TrNum + Clone + 'static,
    CM: CostModel<T> + Clone,
    HM: CostModel<T> + Clone,
    S: BarSeries<T> + 'static,
    I1: Indicator<Num = T, Output = T, Series = S> + 'static,
    I2: Indicator<Num = T, Output = T, Series = S> + 'static,
    R: TradingRecord<T, CM, HM, S>,
{
    type Num = T;
    type CostBuy = CM;
    type CostSell = HM;
    type Series = S;
    type TradingRec = R;

    fn is_satisfied_with_record(
        &self,
        index: usize,
        _trading_record: Option<&Self::TradingRec>,
    ) -> bool {
        let v1 = match self.first.get_value(index) {
            Ok(v) => v,
            Err(_) => return false,
        };
        let v2 = match self.second.get_value(index) {
            Ok(v) => v,
            Err(_) => return false,
        };
        let satisfied = v1 > v2;
        self.base_rule.trace_is_satisfied(index, satisfied);
        satisfied
    }
}
