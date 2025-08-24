use crate::TradingRecord;
use crate::analysis::CostModel;
use crate::bar::types::BarSeries;
use crate::indicators::Indicator;
use crate::indicators::atr_indicator::ATRIndicator;
use crate::indicators::helpers::close_price_indicator::ClosePriceIndicator;
use crate::indicators::helpers::constant_indicator::ConstantIndicator;
use crate::indicators::helpers::highest_value_indicator::HighestValueIndicator;
use crate::indicators::helpers::lowest_value_indicator::LowestValueIndicator;
use crate::indicators::helpers::tr_indicator::TRIndicator;
use crate::indicators::numeric::binary_operation::BinaryOperation;
use crate::indicators::types::IndicatorError;
use crate::num::TrNum;
use crate::num::types::{trnum_add, trnum_sub};
use crate::rule::Rule;
use crate::rule::base_rule::BaseRule;
use crate::trade::TradeType;
use parking_lot::RwLock;
use std::marker::PhantomData;
use std::sync::Arc;

pub struct AverageTrueRangeTrailingStopLossRule<T, CM, HM, S, I, R>
where
    T: TrNum + Clone + 'static,
    CM: CostModel<T> + Clone,
    HM: CostModel<T> + Clone,
    S: BarSeries<T> + 'static,
    I: Indicator<Num = T, Series = S, Output = T> + 'static,
    R: TradingRecord<T, CM, HM, S>,
{
    stop_loss_threshold: Arc<BinaryOperation<T, ATRIndicator<T, S>, ConstantIndicator<T, S>>>,
    reference_price: Arc<I>,
    base_rule: BaseRule<Self>,
    _phantom: PhantomData<(CM, HM, R)>,
}

impl<T, CM, HM, S, I, R> AverageTrueRangeTrailingStopLossRule<T, CM, HM, S, I, R>
where
    T: TrNum + Clone + 'static,
    CM: CostModel<T> + Clone,
    HM: CostModel<T> + Clone,
    S: BarSeries<T> + 'static,
    I: Indicator<Num = T, Series = S, Output = T> + 'static,
    R: TradingRecord<T, CM, HM, S>,
{
    pub fn new(
        series: Arc<RwLock<S>>,
        reference_price: Arc<I>,
        atr_bar_count: usize,
        atr_coefficient: T,
    ) -> Result<Self, IndicatorError> {
        let tr_indicator = Arc::new(TRIndicator::from_shared(series.clone()));
        let atr_indicator = Arc::new(ATRIndicator::from_tr(tr_indicator, atr_bar_count));
        let right_indicator = Arc::new(ConstantIndicator::new(
            atr_indicator.bar_series(),
            atr_coefficient,
        ));
        let stop_loss_threshold = Arc::new(BinaryOperation::product(
            atr_indicator.clone(),
            right_indicator,
        ));

        Ok(Self {
            stop_loss_threshold,
            reference_price,
            base_rule: BaseRule::new("AverageTrueRangeTrailingStopLossRule"),
            _phantom: PhantomData,
        })
    }

    pub fn new_with_close_price(
        series: Arc<RwLock<S>>,
        atr_bar_count: usize,
        atr_coefficient: T,
    ) -> Result<Self, IndicatorError>
    where
        I: From<Arc<ClosePriceIndicator<T, S>>>,
    {
        let close_price_indicator = Arc::new(ClosePriceIndicator::from_shared(series.clone()));
        let reference_price = Arc::new(I::from(close_price_indicator));
        Self::new(series, reference_price, atr_bar_count, atr_coefficient)
    }
}

impl<T, CM, HM, S, I, R> Rule for AverageTrueRangeTrailingStopLossRule<T, CM, HM, S, I, R>
where
    T: TrNum + Clone + 'static,
    CM: CostModel<T> + Clone,
    HM: CostModel<T> + Clone,
    S: BarSeries<T> + 'static,
    I: Indicator<Num = T, Series = S, Output = T> + 'static,
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
            let current_pos = record.current_position();
            if current_pos.is_opened() {
                if let Some(entry_trade) = current_pos.entry() {
                    let entry_price = entry_trade.net_price();
                    let current_price = match self.reference_price.get_value(index) {
                        Ok(p) => p,
                        Err(_) => return false,
                    };
                    let threshold = match self.stop_loss_threshold.get_value(index) {
                        Ok(t) => t,
                        Err(_) => return false,
                    };

                    let bars_since_entry = index - entry_trade.index() + 1;

                    satisfied = match entry_trade.trade_type() {
                        TradeType::Buy => {
                            let highest_ref = HighestValueIndicator::new(
                                self.reference_price.clone(),
                                bars_since_entry,
                            );
                            let target = trnum_sub(
                                &entry_price.max(&highest_ref.get_value(index).unwrap()),
                                &threshold,
                            );
                            current_price <= target
                        }
                        TradeType::Sell => {
                            let lowest_ref = LowestValueIndicator::new(
                                self.reference_price.clone(),
                                bars_since_entry,
                            );
                            let target = trnum_add(
                                &entry_price.min(&lowest_ref.get_value(index).unwrap()),
                                &threshold,
                            );
                            current_price >= target
                        }
                    };
                }
            }
        }

        self.base_rule.trace_is_satisfied(index, satisfied);
        satisfied
    }
}
