use crate::TradingRecord;
use crate::analysis::CostModel;
use crate::bar::types::BarSeries;
use crate::num::TrNum;
use crate::rule::Rule;
use crate::strategy::Strategy;
use crate::strategy::and_strategy::AndStrategy;
use crate::strategy::opposite_strategy::OppositeStrategy;
use crate::strategy::or_strategy::OrStrategy;
use crate::strategy::types::{DynStrategies, DynStrategyAdapter, Strategies};

/// Rust 版 BaseStrategy
pub struct BaseStrategy<N, Cb, Cs, S, R, E, X>
where
    N: TrNum + 'static,
    Cb: CostModel<N> + Clone,
    Cs: CostModel<N> + Clone,
    S: BarSeries<N> + 'static,
    R: TradingRecord<N, Cb, Cs, S>,
    E: Rule<Num = N, CostBuy = Cb, CostSell = Cs, Series = S, TradingRec = R> + Clone,
    X: Rule<Num = N, CostBuy = Cb, CostSell = Cs, Series = S, TradingRec = R> + Clone,
{
    pub name: String,
    pub entry_rule: E,
    pub exit_rule: X,
    pub unstable_bars: usize,
    _phantom: std::marker::PhantomData<(N, Cb, Cs, S, R)>,
}

impl<N, Cb, Cs, S, R, E, X> Clone for BaseStrategy<N, Cb, Cs, S, R, E, X>
where
    N: TrNum + 'static,
    Cb: CostModel<N> + Clone,
    Cs: CostModel<N> + Clone,
    S: BarSeries<N> + 'static,
    R: TradingRecord<N, Cb, Cs, S>,
    E: Rule<Num = N, CostBuy = Cb, CostSell = Cs, Series = S, TradingRec = R> + Clone,
    X: Rule<Num = N, CostBuy = Cb, CostSell = Cs, Series = S, TradingRec = R> + Clone,
{
    fn clone(&self) -> Self {
        BaseStrategy {
            name: self.name.clone(),
            entry_rule: self.entry_rule.clone(),
            exit_rule: self.exit_rule.clone(),
            unstable_bars: self.unstable_bars,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<N, Cb, Cs, S, R, E, X> BaseStrategy<N, Cb, Cs, S, R, E, X>
where
    N: TrNum + 'static,
    Cb: CostModel<N> + Clone,
    Cs: CostModel<N> + Clone,
    S: BarSeries<N> + 'static,
    R: TradingRecord<N, Cb, Cs, S>,
    E: Rule<Num = N, CostBuy = Cb, CostSell = Cs, Series = S, TradingRec = R> + Clone,
    X: Rule<Num = N, CostBuy = Cb, CostSell = Cs, Series = S, TradingRec = R> + Clone,
{
    pub fn new(name: impl Into<String>, entry_rule: E, exit_rule: X, unstable_bars: usize) -> Self {
        Self {
            name: name.into(),
            entry_rule,
            exit_rule,
            unstable_bars,
            _phantom: Default::default(),
        }
    }

    /// 静态组合枚举
    pub fn boxed(self) -> Strategies<Self>
    where
        Self: Clone,
    {
        Strategies::Base(self)
    }

    /// 动态组合枚举
    pub fn boxed_dyn(self) -> DynStrategies<R>
    where
        Self: Clone + 'static,
    {
        DynStrategies::Base(Box::new(DynStrategyAdapter::new(self)))
    }

    // =========================
    // 静态分发链式组合
    // =========================
    pub fn and_strategy<S2>(self, other: S2) -> AndStrategy<Self, S2>
    where
        S2: Strategy<Num = N, CostBuy = Cb, CostSell = Cs, Series = S, TradingRec = R>,
    {
        AndStrategy::new(self, other)
    }

    pub fn or_strategy<S2>(self, other: S2) -> OrStrategy<Self, S2>
    where
        S2: Strategy<Num = N, CostBuy = Cb, CostSell = Cs, Series = S, TradingRec = R>,
    {
        OrStrategy::new(self, other)
    }

    pub fn opposite_strategy(self) -> OppositeStrategy<Self> {
        OppositeStrategy::new(self)
    }

    // =========================
    // 静态分发枚举组合
    // =========================
    pub fn and_boxed<S2>(self, other: S2) -> Strategies<Self>
    where
        Self: Clone,
        S2: Into<Self> + Clone,
    {
        Strategies::And(
            Box::new(Strategies::Base(self)),
            Box::new(Strategies::Base(other.into())),
        )
    }

    pub fn or_boxed<S2>(self, other: S2) -> Strategies<Self>
    where
        Self: Clone,
        S2: Into<Self> + Clone,
    {
        Strategies::Or(
            Box::new(Strategies::Base(self)),
            Box::new(Strategies::Base(other.into())),
        )
    }

    pub fn opposite_boxed(self) -> Strategies<Self>
    where
        Self: Clone,
    {
        Strategies::Opposite(Box::new(Strategies::Base(self)))
    }

    // =========================
    // 动态分发组合
    // =========================
    pub fn and_dyn<S2>(self, other: S2) -> DynStrategies<R>
    where
        Self: Clone + 'static,
        S2: Strategy<Num = N, CostBuy = Cb, CostSell = Cs, Series = S, TradingRec = R> + 'static,
    {
        DynStrategies::And(
            Box::new(DynStrategies::from_strategy(self)),
            Box::new(DynStrategies::from_strategy(other)),
        )
    }

    pub fn or_dyn<S2>(self, other: S2) -> DynStrategies<R>
    where
        Self: Clone + 'static,
        S2: Strategy<Num = N, CostBuy = Cb, CostSell = Cs, Series = S, TradingRec = R> + 'static,
    {
        DynStrategies::Or(
            Box::new(DynStrategies::from_strategy(self)),
            Box::new(DynStrategies::from_strategy(other)),
        )
    }

    pub fn opposite_dyn(self) -> DynStrategies<R>
    where
        Self: Clone + 'static,
    {
        DynStrategies::Opposite(Box::new(DynStrategies::from_strategy(self)))
    }
}

impl<N, Cb, Cs, S, R, E, X> Strategy for BaseStrategy<N, Cb, Cs, S, R, E, X>
where
    N: TrNum + 'static,
    Cb: CostModel<N> + Clone,
    Cs: CostModel<N> + Clone,
    S: BarSeries<N> + 'static,
    R: TradingRecord<N, Cb, Cs, S>,
    E: Rule<Num = N, CostBuy = Cb, CostSell = Cs, Series = S, TradingRec = R> + Clone,
    X: Rule<Num = N, CostBuy = Cb, CostSell = Cs, Series = S, TradingRec = R> + Clone,
{
    type Num = N;
    type CostBuy = Cb;
    type CostSell = Cs;
    type Series = S;
    type TradingRec = R;
    type EntryRule = E;
    type ExitRule = X;

    fn name(&self) -> &str {
        &self.name
    }

    fn entry_rule(&self) -> Self::EntryRule {
        self.entry_rule.clone()
    }

    fn exit_rule(&self) -> Self::ExitRule {
        self.exit_rule.clone()
    }

    fn unstable_bars(&self) -> usize {
        self.unstable_bars
    }

    fn should_enter(&self, index: usize, trading_record: Option<&Self::TradingRec>) -> bool {
        !self.is_unstable_at(index)
            && self
                .entry_rule
                .is_satisfied_with_record(index, trading_record)
    }

    fn should_exit(&self, index: usize, trading_record: Option<&Self::TradingRec>) -> bool {
        !self.is_unstable_at(index)
            && self
                .exit_rule
                .is_satisfied_with_record(index, trading_record)
    }
}
