use crate::rule::Rule;
use crate::rule::base_rule::BaseRule;

pub struct AndRule<'a, L, R>
where
    L: Rule<'a>,
    R: Rule<
            'a,
            Num = L::Num,
            CostBuy = L::CostBuy,
            CostSell = L::CostSell,
            Series = L::Series,
            TradingRec = L::TradingRec,
        >,
{
    base: BaseRule<'a, L>,
    left: L,
    right: R,
}

impl<'a, L, R> AndRule<'a, L, R>
where
    L: Rule<'a>,
    R: Rule<
            'a,
            Num = L::Num,
            CostBuy = L::CostBuy,
            CostSell = L::CostSell,
            Series = L::Series,
            TradingRec = L::TradingRec,
        >,
{
    pub fn new(left: L, right: R) -> Self {
        Self {
            base: BaseRule::new("AndRule"),
            left,
            right,
        }
    }
}

impl<'a, L, R> Rule<'a> for AndRule<'a, L, R>
where
    L: Rule<'a>,
    R: Rule<
            'a,
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

    fn is_satisfied_with_record(
        &self,
        index: usize,
        trading_record: Option<&Self::TradingRec>,
    ) -> bool {
        let satisfied = self.left.is_satisfied_with_record(index, trading_record)
            && self.right.is_satisfied_with_record(index, trading_record);
        self.base.trace_is_satisfied(index, satisfied);
        satisfied
    }
}
