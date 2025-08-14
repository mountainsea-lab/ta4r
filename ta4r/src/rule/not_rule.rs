use crate::rule::Rule;
use crate::rule::base_rule::BaseRule;

/// 一个取反（NOT）规则
///
/// 当目标规则不满足时返回 `true`；
/// 当目标规则满足时返回 `false`。
pub struct NotRule<'a, L>
where
    L: Rule<'a>,
{
    base: BaseRule<'a, L>,
    rule_to_negate: L,
}

impl<'a, L> NotRule<'a, L>
where
    L: Rule<'a>,
{
    /// 创建一个 NOT 规则
    pub fn new(rule_to_negate: L) -> Self {
        Self {
            base: BaseRule::new("NotRule"),
            rule_to_negate,
        }
    }

    /// 获取被取反的规则
    pub fn rule_to_negate(&self) -> &L {
        &self.rule_to_negate
    }
}

impl<'a, L> Rule<'a> for NotRule<'a, L>
where
    L: Rule<'a>,
{
    type Num = L::Num;
    type CostBuy = L::CostBuy;
    type CostSell = L::CostSell;
    type Series = L::Series;
    type TradingRec = L::TradingRec;

    fn is_satisfied_with_record(
        &self,
        index: usize,
        trading_record: Option<&Self::TradingRec>,
    ) -> bool {
        let satisfied = !self
            .rule_to_negate
            .is_satisfied_with_record(index, trading_record);
        self.base.trace_is_satisfied(index, satisfied);
        satisfied
    }
}
