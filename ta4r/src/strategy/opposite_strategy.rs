use crate::rule::not_rule::NotRule;
use crate::strategy::Strategy;
use crate::strategy::types::{DynStrategies, Strategies};

/// OppositeStrategy 取反策略
#[derive(Clone)]
pub struct OppositeStrategy<S> {
    pub strategy: S,
}

impl<S> OppositeStrategy<S>
where
    S: Strategy + Clone + 'static,
{
    pub fn new(strategy: S) -> Self {
        Self { strategy }
    }

    /// 转换成枚举封装，方便链式自由组合
    pub fn boxed(self) -> Strategies<S> {
        Strategies::Opposite(Box::new(Strategies::Base(self.strategy)))
    }

    /// 动态组合（支持不同类型策略）
    pub fn boxed_dyn(self) -> DynStrategies<S::TradingRec> {
        DynStrategies::Opposite(Box::new(DynStrategies::from_strategy(self.strategy)))
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

    fn name(&self) -> &str {
        "OppositeStrategy"
    }

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

    fn should_enter(&self, index: usize, trading_record: Option<&Self::TradingRec>) -> bool {
        !self.strategy.should_enter(index, trading_record)
    }

    fn should_exit(&self, index: usize, trading_record: Option<&Self::TradingRec>) -> bool {
        !self.strategy.should_exit(index, trading_record)
    }
}
