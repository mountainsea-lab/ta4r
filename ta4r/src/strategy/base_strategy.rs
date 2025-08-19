// use crate::TradingRecord;
// use crate::analysis::CostModel;
// use crate::bar::types::BarSeries;
// use crate::num::TrNum;
// use crate::rule::Rule;
// use crate::rule::and_rule::AndRule;
// use crate::rule::or_rule::OrRule;
// use crate::strategy::Strategy;
// use std::marker::PhantomData;
//
// // /// BaseStrategy: 基础策略实现（对应 Java BaseStrategy）
// // pub struct BaseStrategy<'a, E, X, N, CB, CS, S, TR>
// // where
// //     E: Rule<'a, Num = N, Series = S, TradingRec = TR>,
// //     X: Rule<'a, Num = N, Series = S, TradingRec = TR>,
// // {
// //     name: Option<String>,
// //     entry_rule: E,
// //     exit_rule: X,
// //     unstable_bars: usize,
// //     _phantom: PhantomData<(&'a N, CB, CS, S, TR)>,
// // }
// //
// // impl<'a, E, X, N, CB, CS, S, TR> BaseStrategy<'a, E, X, N, CB, CS, S, TR>
// // where
// //     E: Rule<'a, Num = N, Series = S, TradingRec = TR>,
// //     X: Rule<'a, Num = N, Series = S, TradingRec = TR>,
// // {
// //     pub fn new(entry_rule: E, exit_rule: X) -> Self {
// //         Self {
// //             name: None,
// //             entry_rule,
// //             exit_rule,
// //             unstable_bars: 0,
// //             _phantom: PhantomData,
// //         }
// //     }
// //
// //     pub fn with_name(name: impl Into<String>, entry_rule: E, exit_rule: X) -> Self {
// //         Self {
// //             name: Some(name.into()),
// //             entry_rule,
// //             exit_rule,
// //             unstable_bars: 0,
// //             _phantom: PhantomData,
// //         }
// //     }
// //
// //     pub fn with_unstable(
// //         name: impl Into<String>,
// //         entry_rule: E,
// //         exit_rule: X,
// //         unstable: usize,
// //     ) -> Self {
// //         Self {
// //             name: Some(name.into()),
// //             entry_rule,
// //             exit_rule,
// //             unstable_bars: unstable,
// //             _phantom: PhantomData,
// //         }
// //     }
// //
// //     pub fn name(&self) -> Option<&str> {
// //         self.name.as_deref()
// //     }
// //
// //     pub fn set_unstable_bars(&mut self, unstable: usize) {
// //         self.unstable_bars = unstable;
// //     }
// // }
// //
// // impl<'a, E, X, N, CB, CS, S, TR> Strategy<'a> for BaseStrategy<'a, E, X, N, CB, CS, S, TR>
// // where
// //     N: TrNum + 'static,
// //     CB: CostModel<N> + Clone,
// //     CS: CostModel<N> + Clone,
// //     S: BarSeries<'a, N> + 'a,
// //     TR: TradingRecord<'a, N, CB, CS, S>,
// //     E: Rule<'a, Num = N, CostBuy = CB, CostSell = CS, Series = S, TradingRec = TR> + Clone,
// //     X: Rule<'a, Num = N, CostBuy = CB, CostSell = CS, Series = S, TradingRec = TR> + Clone,
// // {
// //     type Num = N;
// //     type CostBuy = CB;
// //     type CostSell = CS;
// //     type Series = S;
// //     type TradingRec = TR;
// //
// //     type EntryRule = E;
// //     type ExitRule = X;
// //
// //     fn entry_rule(&self) -> Self::EntryRule {
// //         // 返回克隆 / 构造新的 entry_rule
// //         // Rule 要求实现 Clone
// //         self.entry_rule.clone()
// //     }
// //
// //     fn exit_rule(&self) -> Self::ExitRule {
// //         self.exit_rule.clone()
// //     }
// //
// //     fn unstable_bars(&self) -> usize {
// //         self.unstable_bars
// //     }
// //
// //     fn and<T>(
// //         &self,
// //         other: T,
// //     ) -> BaseStrategy<'a, AndRule<'a, E, T::EntryRule>, AndRule<'a, X, T::ExitRule>, N, CB, CS, S, TR>
// //     where
// //         T: Strategy<'a, Num = N, CostBuy = CB, CostSell = CS, Series = S, TradingRec = TR>,
// //     {
// //         let name = format!("and({:?},{:?})", self.name(), other.name());
// //         let unstable = self.unstable_bars.max(other.unstable_bars());
// //
// //         BaseStrategy::with_unstable(
// //             name,
// //             AndRule::new(self.entry_rule.clone(), other.entry_rule()),
// //             AndRule::new(self.exit_rule.clone(), other.exit_rule()),
// //             unstable,
// //         )
// //     }
// //
// //     fn or<T>(
// //         &self,
// //         other: T,
// //     ) -> BaseStrategy<'a, OrRule<'a, E, T::EntryRule>, OrRule<'a, X, T::ExitRule>, N, CB, CS, S, TR>
// //     where
// //         T: Strategy<'a, Num = N, CostBuy = CB, CostSell = CS, Series = S, TradingRec = TR>,
// //     {
// //         let name = format!("or({:?},{:?})", self.name(), other.name());
// //         let unstable = self.unstable_bars.max(other.unstable_bars());
// //
// //         BaseStrategy::with_unstable(
// //             name,
// //             OrRule::new(self.entry_rule.clone(), other.entry_rule()),
// //             OrRule::new(self.exit_rule.clone(), other.exit_rule()),
// //             unstable,
// //         )
// //     }
// //
// //     fn opposite(&self) -> BaseStrategy<'a, X, E, N, CB, CS, S, TR> {
// //         let name = format!("opposite({:?})", self.name());
// //         BaseStrategy::with_unstable(
// //             name,
// //             self.exit_rule.clone(),
// //             self.entry_rule.clone(),
// //             self.unstable_bars,
// //         )
// //     }
// // }
//
// use std::marker::PhantomData;
// use std::fmt::Debug;
//
// // ---------- 基础 trait 定义 ----------
// pub trait TrNum: Clone + Debug {}
// pub trait CostModel<N: TrNum>: Clone {}
// pub trait BarSeries<'a, N: TrNum> {}
// pub trait TradingRecord<'a, N: TrNum, CB: CostModel<N>, CS: CostModel<N>, S: BarSeries<'a, N>> {}
// pub trait Rule<'a> : Clone {
//     type Num: TrNum;
//     type CostBuy: CostModel<Self::Num>;
//     type CostSell: CostModel<Self::Num>;
//     type Series: BarSeries<'a, Self::Num>;
//     type TradingRec: TradingRecord<'a, Self::Num, Self::CostBuy, Self::CostSell, Self::Series>;
//
//     fn is_satisfied(&self) -> bool;
// }
//
// // ---------- Strategy trait ----------
// pub trait Strategy<'a>: Clone {
//     type Num: TrNum;
//     type CostBuy: CostModel<Self::Num>;
//     type CostSell: CostModel<Self::Num>;
//     type Series: BarSeries<'a, Self::Num>;
//     type TradingRec: TradingRecord<'a, Self::Num, Self::CostBuy, Self::CostSell, Self::Series>;
//
//     type EntryRule: Rule<'a, Num = Self::Num, CostBuy = Self::CostBuy, CostSell = Self::CostSell, Series = Self::Series, TradingRec = Self::TradingRec>;
//     type ExitRule: Rule<'a, Num = Self::Num, CostBuy = Self::CostBuy, CostSell = Self::CostSell, Series = Self::Series, TradingRec = Self::TradingRec>;
//
//     fn entry_rule(&self) -> Self::EntryRule;
//     fn exit_rule(&self) -> Self::ExitRule;
//     fn unstable_bars(&self) -> usize;
//
//     // 默认实现的组合逻辑
//     fn and<S2>(self, other: S2) -> CombinedStrategy<'a, Self, S2, AndRuleWrapper, AndRuleWrapper>
//     where
//         Self: Sized,
//         S2: Strategy<'a, Num = Self::Num, CostBuy = Self::CostBuy, CostSell = Self::CostSell, Series = Self::Series, TradingRec = Self::TradingRec>,
//     {
//         CombinedStrategy::new(self, other)
//     }
//
//     fn or<S2>(self, other: S2) -> CombinedStrategy<'a, Self, S2, OrRuleWrapper, OrRuleWrapper>
//     where
//         Self: Sized,
//         S2: Strategy<'a, Num = Self::Num, CostBuy = Self::CostBuy, CostSell = Self::CostSell, Series = Self::Series, TradingRec = Self::TradingRec>,
//     {
//         CombinedStrategy::new(self, other)
//     }
//
//     fn opposite(self) -> Self
//     where
//         Self: Sized,
//     {
//         self
//     }
// }
//
// // ---------- BaseStrategy 实现 ----------
// #[derive(Clone)]
// pub struct BaseStrategy<'a, E, X, N, CB, CS, S, TR>
// where
//     E: Rule<'a, Num = N, Series = S, TradingRec = TR>,
//     X: Rule<'a, Num = N, Series = S, TradingRec = TR>,
// {
//     name: Option<String>,
//     entry_rule: E,
//     exit_rule: X,
//     unstable_bars: usize,
//     _phantom: PhantomData<(&'a N, CB, CS, S, TR)>,
// }
//
// impl<'a, E, X, N, CB, CS, S, TR> BaseStrategy<'a, E, X, N, CB, CS, S, TR>
// where
//     E: Rule<'a, Num = N, Series = S, TradingRec = TR>,
//     X: Rule<'a, Num = N, Series = S, TradingRec = TR>,
// {
//     pub fn new(entry_rule: E, exit_rule: X) -> Self {
//         Self {
//             name: None,
//             entry_rule,
//             exit_rule,
//             unstable_bars: 0,
//             _phantom: PhantomData,
//         }
//     }
//
//     pub fn with_name(name: impl Into<String>, entry_rule: E, exit_rule: X) -> Self {
//         Self {
//             name: Some(name.into()),
//             entry_rule,
//             exit_rule,
//             unstable_bars: 0,
//             _phantom: PhantomData,
//         }
//     }
//
//     pub fn with_unstable(
//         name: impl Into<String>,
//         entry_rule: E,
//         exit_rule: X,
//         unstable: usize,
//     ) -> Self {
//         Self {
//             name: Some(name.into()),
//             entry_rule,
//             exit_rule,
//             unstable_bars: unstable,
//             _phantom: PhantomData,
//         }
//     }
//
//     pub fn name(&self) -> Option<&str> {
//         self.name.as_deref()
//     }
//
//     pub fn set_unstable_bars(&mut self, unstable: usize) {
//         self.unstable_bars = unstable;
//     }
//
//     // 包装 trait 默认方法，保持链式调用
//     pub fn and_strategy<S2>(self, other: S2) -> CombinedStrategy<'a, Self, S2, AndRuleWrapper, AndRuleWrapper>
//     where
//         Self: Strategy<'a> + Sized,
//         S2: Strategy<'a, Num = Self::Num, CostBuy = Self::CostBuy, CostSell = Self::CostSell, Series = Self::Series, TradingRec = Self::TradingRec>,
//     {
//         <Self as Strategy<'a>>::and(self, other)
//     }
//
//     pub fn or_strategy<S2>(self, other: S2) -> CombinedStrategy<'a, Self, S2, OrRuleWrapper, OrRuleWrapper>
//     where
//         Self: Strategy<'a> + Sized,
//         S2: Strategy<'a, Num = Self::Num, CostBuy = Self::CostBuy, CostSell = Self::CostSell, Series = Self::Series, TradingRec = Self::TradingRec>,
//     {
//         <Self as Strategy<'a>>::or(self, other)
//     }
//
//     pub fn opposite_strategy(self) -> Self
//     where
//         Self: Strategy<'a> + Sized,
//     {
//         <Self as Strategy<'a>>::opposite(self)
//     }
// }
//
// // ---------- 组合策略包装类型示例 ----------
// #[derive(Clone)]
// pub struct CombinedStrategy<'a, S1, S2, EWrapper, XWrapper> {
//     s1: S1,
//     s2: S2,
//     _phantom: PhantomData<(&'a (), EWrapper, XWrapper)>,
// }
//
// impl<'a, S1, S2, EWrapper, XWrapper> CombinedStrategy<'a, S1, S2, EWrapper, XWrapper> {
//     pub fn new(s1: S1, s2: S2) -> Self {
//         Self {
//             s1,
//             s2,
//             _phantom: PhantomData,
//         }
//     }
// }
//
// // ---------- 占位规则包装 ----------
// #[derive(Clone)]
// pub struct AndRuleWrapper;
// #[derive(Clone)]
// pub struct OrRuleWrapper;
