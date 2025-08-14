use crate::rule::base_rule::BaseRule;
use crate::rule::Rule;

pub struct BooleanRule<'a, N, CB, CS, S, TR>
where
    N: 'a,
    CB: 'a,
    CS: 'a,
    S: 'a,
    TR: 'a,
{
    base: BaseRule<'a, Self>,
    satisfied: bool,
    _marker: std::marker::PhantomData<(&'a N, CB, CS, S, TR)>,
}

impl<'a, N, CB, CS, S, TR> BooleanRule<'a, N, CB, CS, S, TR> {
    pub const fn new(satisfied: bool) -> Self {
        Self {
            base: BaseRule::new("BooleanRule"),
            satisfied,
            _marker: std::marker::PhantomData,
        }
    }

    pub const TRUE: Self = Self::new(true);
    pub const FALSE: Self = Self::new(false);
}

impl<'a, N, CB, CS, S, TR> Rule<'a> for BooleanRule<'a, N, CB, CS, S, TR> {
    type Num = N;
    type CostBuy = CB;
    type CostSell = CS;
    type Series = S;
    type TradingRec = TR;

    fn is_satisfied_with_record(
        &self,
        index: usize,
        _trading_record: Option<&Self::TradingRec>,
    ) -> bool {
        self.base.trace_is_satisfied(index, self.satisfied);
        self.satisfied
    }
}
