use crate::TradingRecord;
use crate::analysis::CostModel;
use crate::bar::types::BarSeries;
use crate::num::TrNum;
use crate::rule::Rule;
use crate::rule::and_rule::AndRule;
use crate::rule::or_rule::OrRule;
use crate::strategy::Strategy;
use std::marker::PhantomData;

/// BaseStrategy: 基础策略实现（对应 Java BaseStrategy）
pub struct BaseStrategy<'a, E, X, N, CB, CS, S, TR>
where
    E: Rule<'a, Num = N, Series = S, TradingRec = TR>,
    X: Rule<'a, Num = N, Series = S, TradingRec = TR>,
{
    name: Option<String>,
    entry_rule: E,
    exit_rule: X,
    unstable_bars: usize,
    _phantom: PhantomData<(&'a N, CB, CS, S, TR)>,
}

impl<'a, E, X, N, CB, CS, S, TR> BaseStrategy<'a, E, X, N, CB, CS, S, TR>
where
    E: Rule<'a, Num = N, Series = S, TradingRec = TR>,
    X: Rule<'a, Num = N, Series = S, TradingRec = TR>,
{
    pub fn new(entry_rule: E, exit_rule: X) -> Self {
        Self {
            name: None,
            entry_rule,
            exit_rule,
            unstable_bars: 0,
            _phantom: PhantomData,
        }
    }

    pub fn with_name(name: impl Into<String>, entry_rule: E, exit_rule: X) -> Self {
        Self {
            name: Some(name.into()),
            entry_rule,
            exit_rule,
            unstable_bars: 0,
            _phantom: PhantomData,
        }
    }

    pub fn with_unstable(
        name: impl Into<String>,
        entry_rule: E,
        exit_rule: X,
        unstable: usize,
    ) -> Self {
        Self {
            name: Some(name.into()),
            entry_rule,
            exit_rule,
            unstable_bars: unstable,
            _phantom: PhantomData,
        }
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn set_unstable_bars(&mut self, unstable: usize) {
        self.unstable_bars = unstable;
    }
}

impl<'a, E, X, N, CB, CS, S, TR> Strategy<'a> for BaseStrategy<'a, E, X, N, CB, CS, S, TR>
where
    N: TrNum + 'static,
    CB: CostModel<N> + Clone,
    CS: CostModel<N> + Clone,
    S: BarSeries<'a, N> + 'a,
    TR: TradingRecord<'a, N, CB, CS, S>,
    E: Rule<'a, Num = N, CostBuy = CB, CostSell = CS, Series = S, TradingRec = TR> + Clone,
    X: Rule<'a, Num = N, CostBuy = CB, CostSell = CS, Series = S, TradingRec = TR> + Clone,
{
    type Num = N;
    type CostBuy = CB;
    type CostSell = CS;
    type Series = S;
    type TradingRec = TR;

    type EntryRule = E;
    type ExitRule = X;

    fn entry_rule(&self) -> Self::EntryRule {
        // 返回克隆 / 构造新的 entry_rule
        // Rule 要求实现 Clone
        self.entry_rule.clone()
    }

    fn exit_rule(&self) -> Self::ExitRule {
        self.exit_rule.clone()
    }

    fn unstable_bars(&self) -> usize {
        self.unstable_bars
    }

    fn and<T>(
        &self,
        other: T,
    ) -> BaseStrategy<'a, AndRule<'a, E, T::EntryRule>, AndRule<'a, X, T::ExitRule>, N, CB, CS, S, TR>
    where
        T: Strategy<'a, Num = N, CostBuy = CB, CostSell = CS, Series = S, TradingRec = TR>,
    {
        let name = format!("and({:?},{:?})", self.name(), other.name());
        let unstable = self.unstable_bars.max(other.unstable_bars());

        BaseStrategy::with_unstable(
            name,
            AndRule::new(self.entry_rule.clone(), other.entry_rule()),
            AndRule::new(self.exit_rule.clone(), other.exit_rule()),
            unstable,
        )
    }

    fn or<T>(
        &self,
        other: T,
    ) -> BaseStrategy<'a, OrRule<'a, E, T::EntryRule>, OrRule<'a, X, T::ExitRule>, N, CB, CS, S, TR>
    where
        T: Strategy<'a, Num = N, CostBuy = CB, CostSell = CS, Series = S, TradingRec = TR>,
    {
        let name = format!("or({:?},{:?})", self.name(), other.name());
        let unstable = self.unstable_bars.max(other.unstable_bars());

        BaseStrategy::with_unstable(
            name,
            OrRule::new(self.entry_rule.clone(), other.entry_rule()),
            OrRule::new(self.exit_rule.clone(), other.exit_rule()),
            unstable,
        )
    }

    fn opposite(&self) -> BaseStrategy<'a, X, E, N, CB, CS, S, TR> {
        let name = format!("opposite({:?})", self.name());
        BaseStrategy::with_unstable(
            name,
            self.exit_rule.clone(),
            self.entry_rule.clone(),
            self.unstable_bars,
        )
    }
}
