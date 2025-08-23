use crate::rule::and_rule::AndRule;
use crate::strategy::Strategy;
use crate::strategy::types::{DynStrategies, Strategies};

/// AndStrategy 组合两个策略
#[derive(Clone)]
pub struct AndStrategy<L, R> {
    pub(crate) left: L,
    pub(crate) right: R,
}

impl<L, R> AndStrategy<L, R>
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
    pub fn new(left: L, right: R) -> Self {
        Self { left, right }
    }
    /// 转换成枚举封装（静态类型相同场景,大多数情况）
    pub fn boxed(self) -> Strategies<L>
    where
        R: Into<L>, // 仅当 R 可以转换成 L 时
    {
        Strategies::And(
            Box::new(Strategies::Base(self.left)),
            Box::new(Strategies::Base(self.right.into())),
        )
    }

    /// 完全自由组合（跨不同类型的策略，使用 dyn StrategyDyn）
    pub fn boxed_dyn(self) -> DynStrategies<L::TradingRec> {
        DynStrategies::And(
            Box::new(DynStrategies::from_strategy(self.left)),
            Box::new(DynStrategies::from_strategy(self.right)),
        )
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

    fn name(&self) -> &str {
        "AndStrategy"
    }

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

    fn should_enter(&self, index: usize, trading_record: Option<&Self::TradingRec>) -> bool {
        self.left.should_enter(index, trading_record)
            && self.right.should_enter(index, trading_record)
    }

    fn should_exit(&self, index: usize, trading_record: Option<&Self::TradingRec>) -> bool {
        self.left.should_exit(index, trading_record)
            && self.right.should_exit(index, trading_record)
    }
}
