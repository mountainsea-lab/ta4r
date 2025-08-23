use crate::TradingRecord;
use crate::analysis::CostModel;
use crate::bar::types::BarSeries;
use crate::indicators::Indicator;
use crate::indicators::helpers::previous_value_indicator::PreviousValueIndicator;
use crate::indicators::numeric::binary_operation::BinaryOperation;
use crate::num::TrNum;
use crate::rule::Rule;
use crate::rule::base_rule::BaseRule;
use std::marker::PhantomData;
use std::sync::Arc;

/// InSlopeRule: 判断指标在指定的斜率区间内
pub struct InSlopeRule<T, CM, HM, S, IR, R>
where
    T: TrNum + Clone + 'static,
    CM: CostModel<T> + Clone,
    HM: CostModel<T> + Clone,
    S: BarSeries<T> + 'static,
    IR: Indicator<Num = T, Output = T, Series = S> + 'static,
    R: TradingRecord<T, CM, HM, S>,
{
    ref_ind: Arc<IR>,
    prev: PreviousValueIndicator<T, S, IR>,
    min_slope: T,
    max_slope: T,
    base_rule: BaseRule<Self>,
    _phantom: PhantomData<(CM, HM, R)>,
}

impl<T, CM, HM, S, IR, R> InSlopeRule<T, CM, HM, S, IR, R>
where
    T: TrNum + Clone + 'static,
    CM: CostModel<T> + Clone,
    HM: CostModel<T> + Clone,
    S: BarSeries<T> + 'static,
    IR: Indicator<Num = T, Output = T, Series = S>,
    R: TradingRecord<T, CM, HM, S>,
{
    /// Java: InSlopeRule(ref, minSlope)
    pub fn with_min(ref_ind: Arc<IR>, min_slope: T) -> Self {
        Self::new(ref_ind, 1, min_slope, T::nan())
    }

    /// Java: InSlopeRule(ref, minSlope, maxSlope)
    pub fn with_min_max(ref_ind: Arc<IR>, min_slope: T, max_slope: T) -> Self {
        Self::new(ref_ind, 1, min_slope, max_slope)
    }

    /// Java: InSlopeRule(ref, nthPrevious, maxSlope)
    pub fn with_max_n(ref_ind: Arc<IR>, nth_previous: usize, max_slope: T) -> Self {
        Self::new(ref_ind, nth_previous, T::nan(), max_slope)
    }

    /// Java: InSlopeRule(ref, nthPrevious, minSlope, maxSlope)
    pub fn new(ref_ind: Arc<IR>, nth_previous: usize, min_slope: T, max_slope: T) -> Self {
        let prev = PreviousValueIndicator::with_n(ref_ind.clone(), nth_previous);
        Self {
            ref_ind,
            prev,
            min_slope,
            max_slope,
            base_rule: BaseRule::new("InSlopeRule"),
            _phantom: PhantomData,
        }
    }
}

impl<T, CM, HM, S, IR, R> Rule for InSlopeRule<T, CM, HM, S, IR, R>
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
        // 如果 min 和 max 都没定义，规则无意义
        if self.min_slope.is_nan() && self.max_slope.is_nan() {
            self.base_rule.trace_is_satisfied(index, false);
            return false;
        }

        // 计算 ref - prev
        let diff = BinaryOperation::difference(self.ref_ind.clone(), Arc::new(self.prev.clone()));
        let val = match diff.get_value(index) {
            Ok(v) => v,
            Err(_) => return false,
        };

        let min_ok = self.min_slope.is_nan() || val >= self.min_slope;
        let max_ok = self.max_slope.is_nan() || val <= self.max_slope;
        let satisfied = min_ok && max_ok;

        self.base_rule.trace_is_satisfied(index, satisfied);
        satisfied
    }
}
