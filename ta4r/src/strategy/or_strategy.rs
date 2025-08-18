use crate::rule::or_rule::OrRule;
use crate::strategy::Strategy;
use std::marker::PhantomData;

/// OrStrategy 组合两个策略
pub struct OrStrategy<'a, L, R> {
    pub(crate) left: L,
    pub(crate) right: R,
    pub(crate) _phantom: PhantomData<&'a ()>,
}

impl<'a, L, R> OrStrategy<'a, L, R>
where
    L: Strategy<'a>,
    R: Strategy<
            'a,
            Num = L::Num,
            CostBuy = L::CostBuy,
            CostSell = L::CostSell,
            Series = L::Series,
            TradingRec = L::TradingRec,
        >,
{
    pub fn new(left: L, right: R) -> Self {
        Self {
            left,
            right,
            _phantom: PhantomData,
        }
    }
}

impl<'a, L, R> Strategy<'a> for OrStrategy<'a, L, R>
where
    L: Strategy<'a>,
    R: Strategy<
            'a,
            Num = L::Num,
            CostBuy = L::CostBuy,
            CostSell = L::CostSell,
            Series = L::Series,
            TradingRec = L::TradingRec,
        >,
    L::EntryRule: 'a,
    L::ExitRule: 'a,
    R::EntryRule: 'a,
    R::ExitRule: 'a,
{
    type Num = L::Num;
    type CostBuy = L::CostBuy;
    type CostSell = L::CostSell;
    type Series = L::Series;
    type TradingRec = L::TradingRec;

    type EntryRule = OrRule<'a, L::EntryRule, R::EntryRule>;
    type ExitRule = OrRule<'a, L::ExitRule, R::ExitRule>;

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
