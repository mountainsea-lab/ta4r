use crate::rule::and_rule::AndRule;
use crate::strategy::Strategy;

/// AndStrategy 组合两个策略
pub struct AndStrategy<L, R> {
    pub(crate) left: L,
    pub(crate) right: R,
}

impl<L, R> AndStrategy<L, R>
where
    L: Strategy,
    R: Strategy<
            Num = L::Num,
            CostBuy = L::CostBuy,
            CostSell = L::CostSell,
            Series = L::Series,
            TradingRec = L::TradingRec,
        >,
{
    pub fn new(left: L, right: R) -> Self {
        Self { left, right }
    }
}

impl<L, R> Strategy for AndStrategy<L, R>
where
    L: Strategy,
    R: Strategy<
            Num = L::Num,
            CostBuy = L::CostBuy,
            CostSell = L::CostSell,
            Series = L::Series,
            TradingRec = L::TradingRec,
        >,
{
    type Num = L::Num;
    type CostBuy = L::CostBuy;
    type CostSell = L::CostSell;
    type Series = L::Series;
    type TradingRec = L::TradingRec;

    type EntryRule = AndRule<L::EntryRule, R::EntryRule>;
    type ExitRule = AndRule<L::ExitRule, R::ExitRule>;

    /// 返回组合后的 EntryRule（每次调用生成新的对象）
    fn entry_rule(&self) -> Self::EntryRule {
        AndRule::new(self.left.entry_rule(), self.right.entry_rule())
    }

    /// 返回组合后的 ExitRule（每次调用生成新的对象）
    fn exit_rule(&self) -> Self::ExitRule {
        AndRule::new(self.left.exit_rule(), self.right.exit_rule())
    }

    fn unstable_bars(&self) -> usize {
        self.left.unstable_bars().max(self.right.unstable_bars())
    }
}
