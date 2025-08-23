use crate::TradingRecord;
use crate::analysis::CostModel;
use crate::bar::types::BarSeries;
use crate::indicators::Indicator;
use crate::indicators::helpers::constant_indicator::ConstantIndicator;
use crate::num::{NumFactory, TrNum};
use crate::rule::{Rule, base_rule::BaseRule};
use std::marker::PhantomData;
use std::sync::Arc;

/// InPipeRule
/// 满足条件：ref 指标的值在 lower 和 upper 指标（或常量）之间
pub struct InPipeRule<T, CM, HM, S, IR, IU, IL, R>
where
    T: TrNum + Clone + 'static,
    CM: CostModel<T> + Clone,
    HM: CostModel<T> + Clone,
    S: BarSeries<T> + 'static,
    IR: Indicator<Num = T, Output = T, Series = S>,
    IU: Indicator<Num = T, Output = T, Series = S>,
    IL: Indicator<Num = T, Output = T, Series = S>,
    R: TradingRecord<T, CM, HM, S>,
{
    ref_ind: Arc<IR>,
    upper: Arc<IU>,
    lower: Arc<IL>,
    base_rule: BaseRule<Self>,
    _phantom: PhantomData<(CM, HM, R)>,
}

impl<T, CM, HM, S, IR, IU, IL, R> InPipeRule<T, CM, HM, S, IR, IU, IL, R>
where
    T: TrNum + Clone + 'static,
    CM: CostModel<T> + Clone,
    HM: CostModel<T> + Clone,
    S: BarSeries<T> + 'static,
    IR: Indicator<Num = T, Output = T, Series = S>,
    IU: Indicator<Num = T, Output = T, Series = S>,
    IL: Indicator<Num = T, Output = T, Series = S>,
    R: TradingRecord<T, CM, HM, S>,
{
    /// 使用三个指标构造
    pub fn new(ref_ind: Arc<IR>, upper: Arc<IU>, lower: Arc<IL>) -> Self {
        Self {
            ref_ind,
            upper,
            lower,
            base_rule: BaseRule::new("InPipeRule"),
            _phantom: PhantomData,
        }
    }

    /// 使用 ref 指标 + 上下常量 Num 构造
    pub fn from_constants(
        ref_ind: Arc<IR>,
        upper: T,
        lower: T,
    ) -> InPipeRule<T, CM, HM, S, IR, ConstantIndicator<T, S>, ConstantIndicator<T, S>, R> {
        let upper = Arc::new(ConstantIndicator::new(ref_ind.bar_series(), upper));
        let lower = Arc::new(ConstantIndicator::new(ref_ind.bar_series(), lower));

        InPipeRule {
            ref_ind,
            upper,
            lower,
            base_rule: BaseRule::new("InPipeRule"),
            _phantom: PhantomData,
        }
    }

    /// 使用 ref 指标 + 上下常量 f64 构造
    pub fn from_f64(
        ref_ind: Arc<IR>,
        upper: f64,
        lower: f64,
    ) -> InPipeRule<T, CM, HM, S, IR, ConstantIndicator<T, S>, ConstantIndicator<T, S>, R> {
        let num_factory = ref_ind
            .bar_series()
            .with_ref(|s| s.num_factory())
            .expect("num_factory get failed");
        let upper_num = num_factory.num_of_f64(upper);
        let lower_num = num_factory.num_of_f64(lower);
        Self::from_constants(ref_ind, upper_num, lower_num)
    }

    pub fn get_ref(&self) -> Arc<IR> {
        Arc::clone(&self.ref_ind)
    }

    pub fn get_upper(&self) -> Arc<IU> {
        Arc::clone(&self.upper)
    }

    pub fn get_lower(&self) -> Arc<IL> {
        Arc::clone(&self.lower)
    }
}

impl<T, CM, HM, S, IR, IU, IL, R> Rule for InPipeRule<T, CM, HM, S, IR, IU, IL, R>
where
    T: TrNum + Clone + 'static,
    CM: CostModel<T> + Clone,
    HM: CostModel<T> + Clone,
    S: BarSeries<T> + 'static,
    IR: Indicator<Num = T, Output = T, Series = S>,
    IU: Indicator<Num = T, Output = T, Series = S>,
    IL: Indicator<Num = T, Output = T, Series = S>,
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
        let ref_val = match self.ref_ind.get_value(index) {
            Ok(v) => v,
            Err(_) => return false,
        };
        let upper_val = match self.upper.get_value(index) {
            Ok(v) => v,
            Err(_) => return false,
        };
        let lower_val = match self.lower.get_value(index) {
            Ok(v) => v,
            Err(_) => return false,
        };
        let satisfied = ref_val <= upper_val && ref_val >= lower_val;
        self.base_rule.trace_is_satisfied(index, satisfied);
        satisfied
    }
}
