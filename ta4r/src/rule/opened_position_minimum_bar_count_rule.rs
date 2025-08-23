use crate::TradingRecord;
use crate::analysis::CostModel;
use crate::bar::types::BarSeries;
use crate::num::TrNum;
use crate::rule::Rule;
use crate::rule::base_rule::BaseRule;
use std::marker::PhantomData;

/// OpenedPositionMinimumBarCountRule: 最小持仓 Bar 数规则
pub struct OpenedPositionMinimumBarCountRule<T, CM, HM, S, R>
where
    T: TrNum + Clone + 'static,
    CM: CostModel<T> + Clone,
    HM: CostModel<T> + Clone,
    S: BarSeries<T> + 'static,
    R: TradingRecord<T, CM, HM, S>,
{
    bar_count: usize,
    base_rule: BaseRule<Self>,
    _phantom: PhantomData<(T, CM, HM, S, R)>,
}

impl<T, CM, HM, S, R> OpenedPositionMinimumBarCountRule<T, CM, HM, S, R>
where
    T: TrNum + Clone + 'static,
    CM: CostModel<T> + Clone,
    HM: CostModel<T> + Clone,
    S: BarSeries<T> + 'static,
    R: TradingRecord<T, CM, HM, S>,
{
    /// 构造器
    pub fn new(bar_count: usize) -> Self {
        assert!(bar_count > 0, "Bar count must be positive");
        Self {
            bar_count,
            base_rule: BaseRule::new("OpenedPositionMinimumBarCountRule"),
            _phantom: PhantomData,
        }
    }

    /// 获取 bar_count
    pub fn bar_count(&self) -> usize {
        self.bar_count
    }
}

impl<T, CM, HM, S, R> Rule for OpenedPositionMinimumBarCountRule<T, CM, HM, S, R>
where
    T: TrNum + Clone + 'static,
    CM: CostModel<T> + Clone,
    HM: CostModel<T> + Clone,
    S: BarSeries<T> + 'static,
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
        trading_record: Option<&Self::TradingRec>,
    ) -> bool {
        let satisfied = trading_record
            .and_then(|tr| {
                if tr.get_current_position().is_opened() {
                    tr.get_last_entry()
                        .map(|entry| index >= entry.index() + self.bar_count)
                } else {
                    None
                }
            })
            .unwrap_or(false);

        self.base_rule.trace_is_satisfied(index, satisfied);
        satisfied
    }
}
