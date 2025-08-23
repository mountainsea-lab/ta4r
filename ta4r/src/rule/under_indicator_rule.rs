use crate::TradingRecord;
use crate::analysis::CostModel;
use crate::bar::types::BarSeries;
use crate::indicators::Indicator;
use crate::num::TrNum;
use crate::rule::Rule;
use crate::rule::base_rule::BaseRule;
use std::marker::PhantomData;
use std::sync::Arc;

/// UnderIndicatorRule: 规则满足当 first < second
pub struct UnderIndicatorRule<T, CM, HM, S, I1, I2, R>
where
    T: TrNum + Clone + 'static,
    CM: CostModel<T> + Clone,
    HM: CostModel<T> + Clone,
    S: BarSeries<T> + 'static,
    I1: Indicator<Num = T, Series = S, Output = T> + 'static,
    I2: Indicator<Num = T, Series = S, Output = T> + 'static,
    R: TradingRecord<T, CM, HM, S>,
{
    first: Arc<I1>,
    second: Arc<I2>,
    base_rule: BaseRule<Self>,
    _phantom: PhantomData<(T, CM, HM, S, R)>,
}

impl<T, CM, HM, S, I1, I2, R> UnderIndicatorRule<T, CM, HM, S, I1, I2, R>
where
    T: TrNum + Clone + 'static,
    CM: CostModel<T> + Clone,
    HM: CostModel<T> + Clone,
    S: BarSeries<T> + 'static,
    I1: Indicator<Num = T, Series = S, Output = T> + 'static,
    I2: Indicator<Num = T, Series = S, Output = T> + 'static,
    R: TradingRecord<T, CM, HM, S>,
{
    pub fn new(first: Arc<I1>, second: Arc<I2>) -> Self {
        Self {
            first,
            second,
            base_rule: BaseRule::new("UnderIndicatorRule"),
            _phantom: PhantomData,
        }
    }
}

impl<T, CM, HM, S, I1, I2, R> Rule for UnderIndicatorRule<T, CM, HM, S, I1, I2, R>
where
    T: TrNum + Clone + 'static,
    CM: CostModel<T> + Clone,
    HM: CostModel<T> + Clone,
    S: BarSeries<T> + 'static,
    I1: Indicator<Num = T, Series = S, Output = T> + 'static,
    I2: Indicator<Num = T, Series = S, Output = T> + 'static,
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
        let satisfied = match (self.first.get_value(index), self.second.get_value(index)) {
            (Ok(val1), Ok(val2)) => val1 < val2,
            _ => false,
        };

        self.base_rule.trace_is_satisfied(index, satisfied);
        satisfied
    }
}
