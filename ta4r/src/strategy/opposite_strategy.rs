use crate::rule::not_rule::NotRule;
use crate::strategy::Strategy;

/// OppositeStrategy 取反策略
pub struct OppositeStrategy<S> {
    pub(crate) strategy: S,
}

impl<S> OppositeStrategy<S>
where
    S: Strategy,
{
    pub fn new(strategy: S) -> Self {
        Self { strategy }
    }
}

impl<S> Strategy for OppositeStrategy<S>
where
    S: Strategy,
{
    type Num = S::Num;
    type CostBuy = S::CostBuy;
    type CostSell = S::CostSell;
    type Series = S::Series;
    type TradingRec = S::TradingRec;

    type EntryRule = NotRule<S::EntryRule>;
    type ExitRule = NotRule<S::ExitRule>;

    /// 返回取反的 EntryRule（每次调用生成新的对象）
    fn entry_rule(&self) -> Self::EntryRule {
        NotRule::new(self.strategy.entry_rule())
    }

    /// 返回取反的 ExitRule（每次调用生成新的对象）
    fn exit_rule(&self) -> Self::ExitRule {
        NotRule::new(self.strategy.exit_rule())
    }

    fn unstable_bars(&self) -> usize {
        self.strategy.unstable_bars()
    }
}
