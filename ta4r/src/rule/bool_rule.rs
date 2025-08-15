use crate::rule::{Rule, base_rule::BaseRule};

/// BooleanRule: 总是返回固定 true/false 的规则
pub struct BooleanRule<'a, T>
where
    T: Rule<'a>,
{
    base: BaseRule<'a, T>,
    satisfied: bool,
}

impl<'a, T> BooleanRule<'a, T>
where
    T: Rule<'a>,
{
    /// 构造函数
    pub fn new(satisfied: bool) -> Self {
        Self {
            base: BaseRule::new("BooleanRule"),
            satisfied,
        }
    }

    /// 总是返回 true 的静态实例
    pub fn true_rule() -> Self {
        Self::new(true)
    }

    /// 总是返回 false 的静态实例
    pub fn false_rule() -> Self {
        Self::new(false)
    }
}

impl<'a, T> Rule<'a> for BooleanRule<'a, T>
where
    T: Rule<'a>,
{
    type Num = T::Num;
    type CostBuy = T::CostBuy;
    type CostSell = T::CostSell;
    type Series = T::Series;
    type TradingRec = T::TradingRec;

    fn is_satisfied_with_record(
        &self,
        index: usize,
        _trading_record: Option<&Self::TradingRec>,
    ) -> bool {
        self.base.trace_is_satisfied(index, self.satisfied);
        self.satisfied
    }
}
