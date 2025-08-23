pub mod and_strategy;
pub mod base_strategy;
pub mod opposite_strategy;
pub mod or_strategy;

use crate::TradingRecord;
use crate::analysis::CostModel;
use crate::bar::types::BarSeries;
use crate::num::TrNum;
use crate::rule::Rule;
use crate::strategy::and_strategy::AndStrategy;
use crate::strategy::opposite_strategy::OppositeStrategy;
use crate::strategy::or_strategy::OrStrategy;

pub trait Strategy {
    type Num: TrNum + 'static;
    type CostBuy: CostModel<Self::Num> + Clone;
    type CostSell: CostModel<Self::Num> + Clone;
    type Series: BarSeries<Self::Num> + 'static;
    type TradingRec: TradingRecord<Self::Num, Self::CostBuy, Self::CostSell, Self::Series>;

    type EntryRule: Rule<
            Num = Self::Num,
            CostBuy = Self::CostBuy,
            CostSell = Self::CostSell,
            Series = Self::Series,
            TradingRec = Self::TradingRec,
        >;
    type ExitRule: Rule<
            Num = Self::Num,
            CostBuy = Self::CostBuy,
            CostSell = Self::CostSell,
            Series = Self::Series,
            TradingRec = Self::TradingRec,
        >;

    fn entry_rule(&self) -> Self::EntryRule;
    fn exit_rule(&self) -> Self::ExitRule;

    fn unstable_bars(&self) -> usize;

    fn should_enter(&self, index: usize, trading_record: Option<&Self::TradingRec>) -> bool {
        index >= self.unstable_bars()
            && self
                .entry_rule()
                .is_satisfied_with_record(index, trading_record)
    }

    fn should_exit(&self, index: usize, trading_record: Option<&Self::TradingRec>) -> bool {
        index >= self.unstable_bars()
            && self
                .exit_rule()
                .is_satisfied_with_record(index, trading_record)
    }

    fn should_operate(&self, index: usize, trading_record: &Self::TradingRec) -> bool {
        let position = trading_record.get_current_position();
        if position.is_new() {
            self.should_enter(index, Some(trading_record))
        } else if position.is_opened() {
            self.should_exit(index, Some(trading_record))
        } else {
            false
        }
    }

    // 组合策略（静态分发）
    fn and<S>(&self, other: S) -> AndStrategy<Self, S>
    where
        Self: Clone,
        S: Strategy<
                Num = Self::Num,
                CostBuy = Self::CostBuy,
                CostSell = Self::CostSell,
                Series = Self::Series,
                TradingRec = Self::TradingRec,
            >,
    {
        AndStrategy {
            left: self.clone(),
            right: other,
        }
    }

    fn or<S>(&self, other: S) -> OrStrategy<Self, S>
    where
        Self: Clone,
        S: Strategy<
                Num = Self::Num,
                CostBuy = Self::CostBuy,
                CostSell = Self::CostSell,
                Series = Self::Series,
                TradingRec = Self::TradingRec,
            >,
    {
        OrStrategy {
            left: self.clone(),
            right: other,
        }
    }

    fn opposite(&self) -> OppositeStrategy<Self>
    where
        Self: Clone,
    {
        OppositeStrategy {
            strategy: self.clone(),
        }
    }
}
