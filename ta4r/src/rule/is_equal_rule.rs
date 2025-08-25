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

/// IsEqualRule: 满足当第一个指标等于第二个指标时
pub struct IsEqualRule<T, CM, HM, S, I1, I2, R>
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
    base_rule: BaseRule,
    _phantom: PhantomData<(CM, HM, R)>,
}

impl<T, CM, HM, S, I1, I2, R> IsEqualRule<T, CM, HM, S, I1, I2, R>
where
    T: TrNum + Clone + 'static,
    CM: CostModel<T> + Clone,
    HM: CostModel<T> + Clone,
    S: BarSeries<T> + 'static,
    I1: Indicator<Num = T, Output = T, Series = S> + 'static,
    I2: Indicator<Num = T, Output = T, Series = S> + 'static,
    R: TradingRecord<T, CM, HM, S>,
{
    /// Rust 版: IsEqualRule(indicator, value: TrNum)
    pub fn with_value_num(
        indicator: Arc<I1>,
        value: T,
    ) -> IsEqualRule<T, CM, HM, S, I1, ConstantIndicator<T, S>, R> {
        let constant = Arc::new(ConstantIndicator::new(indicator.bar_series(), value));
        IsEqualRule {
            first: indicator,
            second: constant,
            base_rule: BaseRule::new("IsEqualRule"),
            _phantom: PhantomData,
        }
    }

    /// Rust 版: IsEqualRule(indicator, value: TrNum)
    pub fn with_value_f64(
        indicator: Arc<I1>,
        value: f64,
    ) -> IsEqualRule<T, CM, HM, S, I1, ConstantIndicator<T, S>, R> {
        let num_factory = indicator
            .bar_series()
            .with_ref(|s| s.num_factory())
            .expect("num_factory get failed");
        let num = num_factory.num_of_f64(value);

        Self::with_value_num(indicator, num)
    }

    /// Rust 版: IsEqualRule(first, second)
    pub fn new(first: Arc<I1>, second: Arc<I2>) -> Self {
        Self {
            first,
            second,
            base_rule: BaseRule::new("IsEqualRule"),
            _phantom: PhantomData,
        }
    }
}

impl<T, CM, HM, S, I1, I2, R> Clone for IsEqualRule<T, CM, HM, S, I1, I2, R>
where
    CM: Clone + CostModel<T>,
    HM: Clone + CostModel<T>,
    I1: 'static + Indicator<Num = T, Output = T, Series = S>,
    I2: 'static + Indicator<Num = T, Output = T, Series = S>,
    R: TradingRecord<T, CM, HM, S>,
    S: 'static + BarSeries<T>,
    T: 'static + Clone + TrNum,
{
    fn clone(&self) -> Self {
        Self {
            first: self.first.clone(),
            second: self.second.clone(),
            base_rule: self.base_rule.clone(),
            _phantom: PhantomData,
        }
    }
}

impl<T, CM, HM, S, I1, I2, R> Rule for IsEqualRule<T, CM, HM, S, I1, I2, R>
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
        let satisfied = v1 == v2;
        self.base_rule.trace_is_satisfied(index, satisfied);
        satisfied
    }
}
