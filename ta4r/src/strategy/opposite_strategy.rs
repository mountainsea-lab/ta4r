use crate::rule::not_rule::NotRule;
use crate::strategy::Strategy;
use std::marker::PhantomData;

/// OppositeStrategy 取反策略
pub struct OppositeStrategy<'a, S> {
    pub(crate) strategy: S,
    pub(crate) _phantom: PhantomData<&'a ()>,
}

impl<'a, S> OppositeStrategy<'a, S>
where
    S: Strategy<'a>,
{
    pub fn new(strategy: S) -> Self {
        Self {
            strategy,
            _phantom: PhantomData,
        }
    }
}

impl<'a, S> Strategy<'a> for OppositeStrategy<'a, S>
where
    S: Strategy<'a>,
    S::EntryRule: 'a,
    S::ExitRule: 'a,
{
    type Num = S::Num;
    type CostBuy = S::CostBuy;
    type CostSell = S::CostSell;
    type Series = S::Series;
    type TradingRec = S::TradingRec;

    type EntryRule = NotRule<'a, S::EntryRule>;
    type ExitRule = NotRule<'a, S::ExitRule>;

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
