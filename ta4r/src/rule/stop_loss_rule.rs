use crate::TradingRecord;
use crate::analysis::CostModel;
use crate::bar::types::BarSeries;
use crate::indicators::Indicator;
use crate::indicators::helpers::close_price_indicator::ClosePriceIndicator;
use crate::num::{NumFactory, TrNum};
use crate::rule::Rule;
use crate::rule::base_rule::BaseRule;
use std::marker::PhantomData;
use std::sync::Arc;

/// StopLossRule: 止损规则
pub struct StopLossRule<T, CM, HM, S, R>
where
    T: TrNum + Clone + 'static,
    CM: CostModel<T> + Clone,
    HM: CostModel<T> + Clone,
    S: BarSeries<T> + 'static,
    R: TradingRecord<T, CM, HM, S>,
{
    close_price: Arc<ClosePriceIndicator<T, S>>,
    loss_percentage: T,
    hundred: T,
    base_rule: BaseRule<Self>,
    _phantom: PhantomData<(CM, HM, S, R)>,
}

impl<T, CM, HM, S, R> StopLossRule<T, CM, HM, S, R>
where
    T: TrNum + Clone + 'static,
    CM: CostModel<T> + Clone,
    HM: CostModel<T> + Clone,
    S: BarSeries<T> + 'static,
    R: TradingRecord<T, CM, HM, S>,
{
    /// 构造器：使用 TrNum
    pub fn with_loss_num(close_price: Arc<ClosePriceIndicator<T, S>>, loss_percentage: T) -> Self {
        let hundred = close_price
            .bar_series()
            .with_ref(|s| s.num_factory())
            .expect("num_factory get failed")
            .num_of_f64(100.0);

        Self {
            close_price,
            loss_percentage,
            hundred,
            base_rule: BaseRule::new("StopLossRule"),
            _phantom: PhantomData,
        }
    }

    /// 构造器：使用 f64
    pub fn with_loss_f64(
        close_price: Arc<ClosePriceIndicator<T, S>>,
        loss_percentage: f64,
    ) -> Self {
        let num_factory = close_price
            .bar_series()
            .with_ref(|s| s.num_factory())
            .expect("num_factory get failed");
        let loss_num = num_factory.num_of_f64(loss_percentage);
        Self::with_loss_num(close_price, loss_num)
    }

    /// 买入止损是否满足
    fn is_buy_stop_satisfied(&self, entry_price: T, current_price: T) -> bool {
        let ratio = self.hundred.clone() - self.loss_percentage.clone();
        let ratio = ratio / self.hundred.clone();
        let threshold = entry_price * ratio;
        current_price <= threshold
    }

    /// 卖出止损是否满足
    fn is_sell_stop_satisfied(&self, entry_price: T, current_price: T) -> bool {
        let ratio = self.hundred.clone() + self.loss_percentage.clone();
        let ratio = ratio / self.hundred.clone();
        let threshold = entry_price * ratio;
        current_price >= threshold
    }
}

impl<T, CM, HM, S, R> Rule for StopLossRule<T, CM, HM, S, R>
where
    T: TrNum + Clone + 'static,
    CM: CostModel<T> + Clone,
    HM: CostModel<T> + Clone,
    S: BarSeries<T> + 'static,
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
        let satisfied = trading_record
            .and_then(|tr| {
                let pos = tr.get_current_position();
                if pos.is_opened() {
                    pos.entry().and_then(|entry| {
                        let entry_price = entry.net_price();
                        let current_price = self.close_price.get_value(index).ok()?;
                        let result = if entry.is_buy() {
                            self.is_buy_stop_satisfied(entry_price.clone(), current_price)
                        } else {
                            self.is_sell_stop_satisfied(entry_price.clone(), current_price)
                        };
                        Some(result)
                    })
                } else {
                    None
                }
            })
            .unwrap_or(false);

        self.base_rule.trace_is_satisfied(index, satisfied);
        satisfied
    }
}
