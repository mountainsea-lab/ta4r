use crate::TradingRecord;
use crate::analysis::CostModel;
use crate::bar::types::BarSeries;
use crate::indicators::Indicator;
use crate::indicators::atr_indicator::ATRIndicator;
use crate::num::TrNum;
use crate::num::types::{trnum_add, trnum_sub};
use crate::rule::Rule;
use crate::rule::base_rule::BaseRule;
use crate::trade::TradeType;
use std::marker::PhantomData;
use std::sync::Arc;

pub struct AverageTrueRangeStopGainRule<T, CM, HM, S, I, R>
where
    T: TrNum + Clone + 'static,
    CM: CostModel<T> + Clone,
    HM: CostModel<T> + Clone,
    S: BarSeries<T> + 'static,
    I: Indicator<Num = T, Series = S, Output = T>,
    R: TradingRecord<T, CM, HM, S>,
{
    stop_gain_threshold: Arc<ATRIndicator<T, S>>,
    reference_price: Arc<I>,
    base_rule: BaseRule<Self>,
    _phantom: PhantomData<(CM, HM, R)>,
}

impl<T, CM, HM, S, I, R> Rule for AverageTrueRangeStopGainRule<T, CM, HM, S, I, R>
where
    T: TrNum + Clone + 'static,
    CM: CostModel<T> + Clone,
    HM: CostModel<T> + Clone,
    S: BarSeries<T> + 'static,
    I: Indicator<Num = T, Series = S, Output = T>,
    R: TradingRecord<T, CM, HM, S>,
{
    type Num = T;
    type CostBuy = CM;
    type CostSell = HM;
    type Series = S;
    type TradingRec = R;
    fn is_satisfied_with_record(
        &self,
        index: usize,
        trading_record: Option<&Self::TradingRec>,
    ) -> bool {
        let mut satisfied = false;

        if let Some(record) = trading_record {
            let current_position = record.get_current_position();
            if current_position.is_opened() {
                if let Some(entry_trade) = current_position.entry() {
                    let entry_price = entry_trade.net_price(); // 引用
                    // 直接获取值
                    let current_price_val = match self.reference_price.get_value(index) {
                        Ok(p) => p,
                        Err(_) => return false,
                    };
                    let threshold_val = match self.stop_gain_threshold.get_value(index) {
                        Ok(t) => t,
                        Err(_) => return false,
                    };

                    satisfied = match entry_trade.trade_type() {
                        TradeType::Buy => {
                            let target = trnum_add(entry_price, &threshold_val);
                            current_price_val >= target
                        }
                        TradeType::Sell => {
                            let target = trnum_sub(entry_price, &threshold_val);
                            current_price_val <= target
                        }
                    };
                }
            }
        }

        self.base_rule.trace_is_satisfied(index, satisfied);
        satisfied
    }
}
