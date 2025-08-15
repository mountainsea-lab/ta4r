use crate::rule::Rule;
use crate::rule::base_rule::BaseRule;
use std::cell::Cell;
use std::marker::PhantomData;

/// 一次性规则：首次满足后，之后永远返回 false
pub struct JustOnceRule<'a, R>
where
    R: Rule<'a>,
{
    base: BaseRule<'a, R>,
    rule: Option<R>,
    satisfied: Cell<bool>,
    _marker: PhantomData<&'a R>,
}

impl<'a, R> JustOnceRule<'a, R>
where
    R: Rule<'a>,
{
    /// 构造器：带子规则
    pub fn new(rule: R) -> Self {
        Self {
            base: BaseRule::new("JustOnceRule"),
            rule: Some(rule),
            satisfied: Cell::new(false),
            _marker: PhantomData,
        }
    }

    /// 构造器：无子规则，第一次调用总是返回 true
    pub fn once() -> Self {
        Self {
            base: BaseRule::new("JustOnceRule"),
            rule: None,
            satisfied: Cell::new(false),
            _marker: PhantomData,
        }
    }

    fn trace_is_satisfied(&self, index: usize, is_satisfied: bool) {
        self.base.trace_is_satisfied(index, is_satisfied);
    }
}

impl<'a, R> Rule<'a> for JustOnceRule<'a, R>
where
    R: Rule<'a>,
{
    type Num = R::Num;
    type CostBuy = R::CostBuy;
    type CostSell = R::CostSell;
    type Series = R::Series;
    type TradingRec = R::TradingRec;

    fn is_satisfied_with_record(
        &self,
        index: usize,
        trading_record: Option<&R::TradingRec>,
    ) -> bool {
        if self.satisfied.get() {
            return false; // 已满足，直接返回 false
        }

        let result = self.rule.as_ref().map_or_else(
            || {
                // 没有子规则，第一次直接返回 true
                self.satisfied.set(true);
                self.trace_is_satisfied(index, true);
                true
            },
            |r| r.is_satisfied_with_record(index, trading_record),
        );

        self.satisfied.set(result);
        result
    }
}
