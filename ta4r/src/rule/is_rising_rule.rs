use crate::TradingRecord;
use crate::analysis::CostModel;
use crate::bar::types::BarSeries;
use crate::indicators::Indicator;
use crate::num::TrNum;
use crate::rule::Rule;
use crate::rule::base_rule::BaseRule;
use std::marker::PhantomData;
use std::sync::Arc;

/// IsRisingRule: 指标在过去 N 根 Bar 内上涨比例达到 min_strength
pub struct IsRisingRule<T, CM, HM, S, IR, R>
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
    min_strength: f64,
    base_rule: BaseRule,
    _phantom: PhantomData<(CM, HM, R)>,
}

impl<T, CM, HM, S, IR, R> IsRisingRule<T, CM, HM, S, IR, R>
where
    T: TrNum + Clone + 'static,
    CM: CostModel<T> + Clone,
    HM: CostModel<T> + Clone,
    S: BarSeries<T> + 'static,
    IR: Indicator<Num = T, Output = T, Series = S>,
    R: TradingRecord<T, CM, HM, S>,
{
    /// 构造器：bar_count 默认 min_strength = 1.0
    pub fn with_bar_count(ref_ind: Arc<IR>, bar_count: usize) -> Self {
        Self::new(ref_ind, bar_count, 1.0)
    }

    /// 构造器：指定 bar_count 和 min_strength
    pub fn new(ref_ind: Arc<IR>, bar_count: usize, min_strength: f64) -> Self {
        let strength = if min_strength >= 1.0 {
            0.99
        } else {
            min_strength
        };
        Self {
            ref_ind,
            bar_count,
            min_strength: strength,
            base_rule: BaseRule::new("IsRisingRule"),
            _phantom: PhantomData,
        }
    }
}

impl<T, CM, HM, S, IR, R> Clone for IsRisingRule<T, CM, HM, S, IR, R>
where
    CM: Clone + CostModel<T>,
    HM: Clone + CostModel<T>,
    IR: 'static + Indicator<Num = T, Output = T, Series = S>,
    R: TradingRecord<T, CM, HM, S>,
    S: 'static + BarSeries<T>,
    T: 'static + Clone + TrNum,
{
    fn clone(&self) -> Self {
        Self {
            ref_ind: self.ref_ind.clone(),
            bar_count: self.bar_count,
            min_strength: self.min_strength,
            base_rule: self.base_rule.clone(),
            _phantom: PhantomData,
        }
    }
}

impl<T, CM, HM, S, IR, R> Rule for IsRisingRule<T, CM, HM, S, IR, R>
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
        let start = if index + 1 >= self.bar_count {
            index + 1 - self.bar_count
        } else {
            0
        };
        let mut count = 0;

        for i in start..=index {
            let current = match self.ref_ind.get_value(i) {
                Ok(v) => v,
                Err(_) => continue,
            };
            let prev_index = if i > 0 { i - 1 } else { 0 };
            let prev = match self.ref_ind.get_value(prev_index) {
                Ok(v) => v,
                Err(_) => continue,
            };
            if current > prev {
                count += 1;
            }
        }

        let ratio = count as f64 / self.bar_count as f64;
        let satisfied = ratio >= self.min_strength;
        self.base_rule.trace_is_satisfied(index, satisfied);
        satisfied
    }
}
