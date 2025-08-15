use crate::TradingRecord;
use crate::rule::Rule;
use crate::rule::base_rule::BaseRule;
use crate::trade::TradeType;
use std::marker::PhantomData;

pub struct WaitForRule<'a, R>
where
    R: Rule<'a>,
{
    base: BaseRule<'a, R>,
    trade_type: TradeType,
    number_of_bars: usize,
    _marker: PhantomData<&'a R>,
}

impl<'a, R> WaitForRule<'a, R>
where
    R: Rule<'a>,
{
    pub fn new(trade_type: TradeType, number_of_bars: usize) -> Self {
        Self {
            base: BaseRule::new("WaitForRule"),
            trade_type,
            number_of_bars,
            _marker: PhantomData,
        }
    }

    fn trace_is_satisfied(&self, index: usize, is_satisfied: bool) {
        self.base.trace_is_satisfied(index, is_satisfied);
    }
}

impl<'a, R> Rule<'a> for WaitForRule<'a, R>
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
        trading_record: Option<&Self::TradingRec>,
    ) -> bool {
        let mut satisfied = false;

        if let Some(record) = trading_record {
            if let Some(last_trade) = record.get_last_trade() {
                let current_number_of_bars = index.saturating_sub(last_trade.index);
                satisfied = current_number_of_bars >= self.number_of_bars;
            }
        }

        self.trace_is_satisfied(index, satisfied);
        satisfied
    }
}
