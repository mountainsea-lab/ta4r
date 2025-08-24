use crate::rule::Rule;
use crate::rule::or_rule::OrRule;
use crate::strategy::Strategy;
use crate::strategy::types::{DynStrategies, Strategies};
use std::sync::Arc;

/// OrStrategy 组合两个策略
// #[derive(Clone)]
pub struct OrStrategy<L, R> {
    pub left: L,
    pub right: R,
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

    /// 转换成枚举封装（静态类型相同场景,大多数情况）
    pub fn boxed(self) -> Strategies<L>
    where
        R: Into<L>,
    {
        Strategies::Or(
            Arc::new(Strategies::Base(Arc::new(self.left))),
            Arc::new(Strategies::Base(Arc::new(self.right.into()))),
        )
    }

    /// 动态组合（支持不同类型，包装成 DynStrategies）
    pub fn boxed_dyn(self) -> DynStrategies<L::TradingRec>
    where
        L: Strategy + Clone + 'static,
        R: Strategy<
                Num = L::Num,
                CostBuy = L::CostBuy,
                CostSell = L::CostSell,
                Series = L::Series,
                TradingRec = L::TradingRec,
            > + 'static,
    {
        DynStrategies::Or(
            Arc::new(DynStrategies::from_strategy(self.left)),
            Arc::new(DynStrategies::from_strategy(self.right)),
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
    fn entry_rule(&self) -> Arc<Self::EntryRule> {
        let left_entry_rule = (*self.left.entry_rule()).clone();
        let right_entry_rule = (*self.right.entry_rule()).clone();
        OrRule::new(left_entry_rule, right_entry_rule).clone_rule()
    }

    /// 返回组合后的 ExitRule（每次调用生成新的对象）
    fn exit_rule(&self) -> Arc<Self::ExitRule> {
        let left_exit_rule = (*self.left.exit_rule()).clone();
        let right_exit_rule = (*self.right.exit_rule()).clone();
        OrRule::new(left_exit_rule, right_exit_rule).clone_rule()
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
