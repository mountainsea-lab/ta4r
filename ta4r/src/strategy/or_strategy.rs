use crate::rule::or_rule::OrRule;
use crate::strategy::Strategy;

/// OrStrategy 组合两个策略
pub struct OrStrategy<L, R> {
    pub(crate) left: L,
    pub(crate) right: R,
}

impl<L, R> OrStrategy<L, R>
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

impl<L, R> Strategy for OrStrategy<L, R>
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

    type EntryRule = OrRule<L::EntryRule, R::EntryRule>;
    type ExitRule = OrRule<L::ExitRule, R::ExitRule>;

    /// 返回组合后的 EntryRule（每次调用生成新的对象）
    fn entry_rule(&self) -> Self::EntryRule {
        OrRule::new(self.left.entry_rule(), self.right.entry_rule())
    }

    /// 返回组合后的 ExitRule（每次调用生成新的对象）
    fn exit_rule(&self) -> Self::ExitRule {
        OrRule::new(self.left.exit_rule(), self.right.exit_rule())
    }

    fn unstable_bars(&self) -> usize {
        self.left.unstable_bars().max(self.right.unstable_bars())
    }
}
