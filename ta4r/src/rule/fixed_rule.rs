use crate::rule::Rule;
use crate::rule::base_rule::BaseRule;
use std::marker::PhantomData;

pub struct FixedRule<'a, R>
where
    R: Rule<'a>,
{
    base: BaseRule<'a, R>,
    indexes: Vec<usize>,
    _marker: PhantomData<&'a R>,
}

impl<'a, R> FixedRule<'a, R>
where
    R: Rule<'a>,
{
    pub fn new(indexes: &[usize]) -> Self {
        Self {
            base: BaseRule::new("FixedRule"),
            indexes: indexes.to_vec(),
            _marker: PhantomData,
        }
    }

    fn trace_is_satisfied(&self, index: usize, is_satisfied: bool) {
        self.base.trace_is_satisfied(index, is_satisfied);
    }
}

impl<'a, R> Rule<'a> for FixedRule<'a, R>
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
        _trading_record: Option<&Self::TradingRec>,
    ) -> bool {
        let satisfied = self.indexes.iter().any(|&i| i == index);
        self.trace_is_satisfied(index, satisfied);
        satisfied
    }
}
