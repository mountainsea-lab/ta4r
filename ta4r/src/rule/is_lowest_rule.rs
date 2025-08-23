use crate::TradingRecord;
use crate::analysis::CostModel;
use crate::bar::types::BarSeries;
use crate::indicators::Indicator;
use crate::indicators::helpers::lowest_value_indicator::LowestValueIndicator;
use crate::num::TrNum;
use crate::rule::Rule;
use crate::rule::base_rule::BaseRule;
use std::marker::PhantomData;
use std::sync::Arc;

/// IsLowestRule: 当前指标值为过去 bar_count 内最低
pub struct IsLowestRule<T, CM, HM, S, IR, R>
where
    T: TrNum + Clone + 'static,
    CM: CostModel<T> + Clone,
    HM: CostModel<T> + Clone,
    S: BarSeries<T> + 'static,
    IR: Indicator<Num = T, Output = T, Series = S> + 'static,
    R: TradingRecord<T, CM, HM, S>,
{
    ref_ind: Arc<IR>,
    bar_count: usize,
    base_rule: BaseRule<Self>,
    _phantom: PhantomData<(CM, HM, R)>,
}

impl<T, CM, HM, S, IR, R> IsLowestRule<T, CM, HM, S, IR, R>
where
    T: TrNum + Clone + 'static,
    CM: CostModel<T> + Clone,
    HM: CostModel<T> + Clone,
    S: BarSeries<T> + 'static,
    IR: Indicator<Num = T, Output = T, Series = S>,
    R: TradingRecord<T, CM, HM, S>,
{
    /// 构造器
    pub fn new(ref_ind: Arc<IR>, bar_count: usize) -> Self {
        Self {
            ref_ind,
            bar_count,
            base_rule: BaseRule::new("IsLowestRule"),
            _phantom: PhantomData,
        }
    }
}

impl<T, CM, HM, S, IR, R> Rule for IsLowestRule<T, CM, HM, S, IR, R>
where
    T: TrNum + Clone + 'static,
    CM: CostModel<T> + Clone,
    HM: CostModel<T> + Clone,
    S: BarSeries<T> + 'static,
    IR: Indicator<Num = T, Output = T, Series = S> + 'static,
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
        // 构造过去 bar_count 内最低值指标
        let lowest_ind = LowestValueIndicator::new(self.ref_ind.clone(), self.bar_count);

        let lowest_val = match lowest_ind.get_value(index) {
            Ok(v) => v,
            Err(_) => return false,
        };
        let ref_val = match self.ref_ind.get_value(index) {
            Ok(v) => v,
            Err(_) => return false,
        };

        // 满足条件：当前值等于过去最低值，且非 NaN
        let satisfied = !ref_val.is_nan() && !lowest_val.is_nan() && ref_val == lowest_val;
        self.base_rule.trace_is_satisfied(index, satisfied);
        satisfied
    }
}
