use crate::strategy::{Strategy, StrategyDyn};

use crate::TradingRecord;
use std::fmt::Debug;

/// 枚举封装策略组合
#[derive(Clone)]
pub enum Strategies<S>
where
    S: Strategy,
{
    Base(S),
    And(Box<Strategies<S>>, Box<Strategies<S>>),
    Or(Box<Strategies<S>>, Box<Strategies<S>>),
    Opposite(Box<Strategies<S>>),
}

impl<S> Strategies<S>
where
    S: Strategy,
{
    /// 链式组合：AND
    pub fn and(self, other: Strategies<S>) -> Strategies<S> {
        Strategies::And(Box::new(self), Box::new(other))
    }

    /// 链式组合：OR
    pub fn or(self, other: Strategies<S>) -> Strategies<S> {
        Strategies::Or(Box::new(self), Box::new(other))
    }

    /// 取反
    pub fn opposite(self) -> Strategies<S> {
        Strategies::Opposite(Box::new(self))
    }

    /// 获取策略名称（递归组合）
    pub fn name(&self) -> String {
        match self {
            Strategies::Base(s) => s.name().to_string(),
            Strategies::And(left, right) => {
                format!("({} AND {})", left.name(), right.name())
            }
            Strategies::Or(left, right) => {
                format!("({} OR {})", left.name(), right.name())
            }
            Strategies::Opposite(inner) => format!("NOT ({})", inner.name()),
        }
    }

    /// 是否进入交易
    pub fn should_enter(&self, index: usize, trading_record: Option<&S::TradingRec>) -> bool {
        match self {
            Strategies::Base(s) => s.should_enter(index, trading_record),
            Strategies::And(left, right) => {
                left.should_enter(index, trading_record)
                    && right.should_enter(index, trading_record)
            }
            Strategies::Or(left, right) => {
                left.should_enter(index, trading_record)
                    || right.should_enter(index, trading_record)
            }
            Strategies::Opposite(inner) => !inner.should_enter(index, trading_record),
        }
    }

    /// 是否退出交易
    pub fn should_exit(&self, index: usize, trading_record: Option<&S::TradingRec>) -> bool {
        match self {
            Strategies::Base(s) => s.should_exit(index, trading_record),
            Strategies::And(left, right) => {
                left.should_exit(index, trading_record) && right.should_exit(index, trading_record)
            }
            Strategies::Or(left, right) => {
                left.should_exit(index, trading_record) || right.should_exit(index, trading_record)
            }
            Strategies::Opposite(inner) => !inner.should_exit(index, trading_record),
        }
    }

    /// 是否执行交易（结合 should_enter / should_exit）
    pub fn should_operate(&self, index: usize, trading_record: &S::TradingRec) -> bool {
        let position = trading_record.get_current_position();
        if position.is_new() {
            self.should_enter(index, Some(trading_record))
        } else if position.is_opened() {
            self.should_exit(index, Some(trading_record))
        } else {
            false
        }
    }
}

/// Adapter: 把泛型 Strategy 包装成 dyn StrategyDyn
pub struct DynStrategyAdapter<S: Strategy> {
    inner: S,
}

impl<S: Strategy> DynStrategyAdapter<S> {
    pub fn new(inner: S) -> Self {
        Self { inner }
    }
}

impl<S: Strategy> StrategyDyn for DynStrategyAdapter<S> {
    type TradingRec = S::TradingRec;

    fn name(&self) -> String {
        self.inner.name().to_string()
    }

    fn should_enter(&self, index: usize, trading_record: Option<&Self::TradingRec>) -> bool {
        self.inner.should_enter(index, trading_record)
    }

    fn should_exit(&self, index: usize, trading_record: Option<&Self::TradingRec>) -> bool {
        self.inner.should_exit(index, trading_record)
    }

    fn should_operate(&self, index: usize, trading_record: &Self::TradingRec) -> bool {
        self.inner.should_operate(index, trading_record)
    }
}

/// 动态分发策略组合（支持不同类型策略组合）
pub enum DynStrategies<R> {
    Base(Box<dyn StrategyDyn<TradingRec = R>>),
    And(Box<DynStrategies<R>>, Box<DynStrategies<R>>),
    Or(Box<DynStrategies<R>>, Box<DynStrategies<R>>),
    Opposite(Box<DynStrategies<R>>),
}

impl<R> DynStrategies<R> {
    /// 从泛型 Strategy 构造动态策略
    pub fn from_strategy<S>(s: S) -> Self
    where
        S: Strategy<TradingRec = R> + 'static,
    {
        DynStrategies::Base(Box::new(DynStrategyAdapter::new(s)))
    }

    /// 与另一个策略组合（AND）
    pub fn and(self, other: Self) -> Self {
        DynStrategies::And(Box::new(self), Box::new(other))
    }

    /// 与另一个策略组合（OR）
    pub fn or(self, other: Self) -> Self {
        DynStrategies::Or(Box::new(self), Box::new(other))
    }

    /// 取反策略
    pub fn opposite(self) -> Self {
        DynStrategies::Opposite(Box::new(self))
    }

    /// 获取策略名称
    pub fn name(&self) -> String {
        match self {
            DynStrategies::Base(s) => s.name(),
            DynStrategies::And(l, r) => format!("({} AND {})", l.name(), r.name()),
            DynStrategies::Or(l, r) => format!("({} OR {})", l.name(), r.name()),
            DynStrategies::Opposite(inner) => format!("NOT ({})", inner.name()),
        }
    }
    pub fn should_enter(&self, index: usize, trading_record: Option<&R>) -> bool {
        match self {
            DynStrategies::Base(s) => s.should_enter(index, trading_record),
            DynStrategies::And(l, r) => {
                l.should_enter(index, trading_record) && r.should_enter(index, trading_record)
            }
            DynStrategies::Or(l, r) => {
                l.should_enter(index, trading_record) || r.should_enter(index, trading_record)
            }
            DynStrategies::Opposite(inner) => !inner.should_enter(index, trading_record),
        }
    }

    pub fn should_exit(&self, index: usize, trading_record: Option<&R>) -> bool {
        match self {
            DynStrategies::Base(s) => s.should_exit(index, trading_record),
            DynStrategies::And(l, r) => {
                l.should_exit(index, trading_record) && r.should_exit(index, trading_record)
            }
            DynStrategies::Or(l, r) => {
                l.should_exit(index, trading_record) || r.should_exit(index, trading_record)
            }
            DynStrategies::Opposite(inner) => !inner.should_exit(index, trading_record),
        }
    }

    pub fn should_operate(&self, index: usize, trading_record: &R) -> bool {
        match self {
            DynStrategies::Base(s) => s.should_operate(index, trading_record),
            DynStrategies::And(l, r) => {
                l.should_operate(index, trading_record) && r.should_operate(index, trading_record)
            }
            DynStrategies::Or(l, r) => {
                l.should_operate(index, trading_record) || r.should_operate(index, trading_record)
            }
            DynStrategies::Opposite(inner) => !inner.should_operate(index, trading_record),
        }
    }
}
