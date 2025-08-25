use crate::TradingRecord;
use crate::analysis::CostModel;
use crate::bar::types::BarSeries;
use crate::indicators::Indicator;
use crate::num::TrNum;
use crate::rule::Rule;
use crate::rule::base_rule::BaseRule;
use std::marker::PhantomData;
use std::sync::Arc;

/// BooleanIndicatorRule：当 Boolean Indicator 返回 true 时满足
pub struct BooleanIndicatorRule<T, CM, HM, S, I, R>
where
    T: TrNum + Clone + 'static,
    CM: CostModel<T> + Clone,
    HM: CostModel<T> + Clone,
    S: BarSeries<T> + 'static,
    I: Indicator<Num = T, Series = S, Output = bool> + 'static,
    R: TradingRecord<T, CM, HM, S>,
{
    indicator: Arc<I>,
    base_rule: BaseRule,
    _phantom: PhantomData<(CM, HM, R)>,
}

impl<T, CM, HM, S, I, R> BooleanIndicatorRule<T, CM, HM, S, I, R>
where
    T: TrNum + Clone + 'static,
    CM: CostModel<T> + Clone,
    HM: CostModel<T> + Clone,
    S: BarSeries<T> + 'static,
    I: Indicator<Num = T, Series = S, Output = bool> + 'static,
    R: TradingRecord<T, CM, HM, S>,
{
    pub fn new(indicator: Arc<I>) -> Self {
        Self {
            indicator,
            base_rule: BaseRule::new("BooleanIndicatorRule"),
            _phantom: PhantomData,
        }
    }
}

impl<T, CM, HM, S, I, R> Clone for BooleanIndicatorRule<T, CM, HM, S, I, R>
where
    CM: Clone + CostModel<T>,
    HM: Clone + CostModel<T>,
    I: 'static + Indicator<Num = T, Series = S, Output = bool>,
    R: TradingRecord<T, CM, HM, S>,
    S: 'static + BarSeries<T>,
    T: 'static + Clone + TrNum,
{
    fn clone(&self) -> Self {
        Self {
            indicator: self.indicator.clone(),
            base_rule: self.base_rule.clone(),
            _phantom: PhantomData,
        }
    }
}

impl<T, CM, HM, S, I, R> Rule for BooleanIndicatorRule<T, CM, HM, S, I, R>
where
    T: TrNum + Clone + 'static,
    CM: CostModel<T> + Clone,
    HM: CostModel<T> + Clone,
    S: BarSeries<T> + 'static,
    I: Indicator<Num = T, Series = S, Output = bool> + 'static,
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
        let satisfied = self.indicator.get_value(index).unwrap_or(false);
        self.base_rule.trace_is_satisfied(index, satisfied);
        satisfied
    }
}
