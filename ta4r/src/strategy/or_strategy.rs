use crate::rule::or_rule::OrRule;
use crate::strategy::Strategy;
use crate::strategy::types::{DynStrategies, Strategies};

/// OrStrategy 组合两个策略
#[derive(Clone)]
pub struct OrStrategy<L, R> {
    pub left: L,
    pub right: R,
}

impl<L, R> OrStrategy<L, R>
where
    L: Strategy + Clone + 'static,
    R: Strategy<
            Num = L::Num,
            CostBuy = L::CostBuy,
            CostSell = L::CostSell,
            Series = L::Series,
            TradingRec = L::TradingRec,
        > + Clone
        + 'static,
{
    pub fn new(left: L, right: R) -> Self {
        Self { left, right }
    }

    /// 转换成枚举封装（静态类型相同场景,大多数情况）
    pub fn boxed(self) -> Strategies<L>
    where
        R: Into<L>,
    {
        Strategies::Or(
            Box::new(Strategies::Base(self.left)),
            Box::new(Strategies::Base(self.right.into())),
        )
    }

    /// 动态组合（支持不同类型，包装成 DynStrategies）
    pub fn boxed_dyn(self) -> DynStrategies<L::TradingRec> {
        DynStrategies::Or(
            Box::new(DynStrategies::from_strategy(self.left)),
            Box::new(DynStrategies::from_strategy(self.right)),
        )
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
    fn name(&self) -> &str {
        "OrStrategy"
    }

    /// 返回组合后的 EntryRule （每次调用生成新的对象）
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

    fn should_enter(&self, index: usize, trading_record: Option<&Self::TradingRec>) -> bool {
        self.left.should_enter(index, trading_record)
            || self.right.should_enter(index, trading_record)
    }

    fn should_exit(&self, index: usize, trading_record: Option<&Self::TradingRec>) -> bool {
        self.left.should_exit(index, trading_record)
            || self.right.should_exit(index, trading_record)
    }
}
