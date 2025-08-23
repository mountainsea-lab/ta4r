use crate::TradingRecord;
use crate::analysis::CostModel;
use crate::bar::types::BarSeries;
use crate::indicators::Indicator;
use crate::indicators::helpers::{
    highest_value_indicator::HighestValueIndicator, lowest_value_indicator::LowestValueIndicator,
};
use crate::num::{NumFactory, TrNum};
use crate::rule::Rule;
use crate::rule::base_rule::BaseRule;
use std::marker::PhantomData;
use std::sync::Arc;

/// TrailingStopLossRule: 移动止损规则
pub struct TrailingStopLossRule<T, CM, HM, S, I, R>
where
    T: TrNum + Clone + 'static,
    CM: CostModel<T> + Clone,
    HM: CostModel<T> + Clone,
    S: BarSeries<T> + 'static,
    I: Indicator<Num = T, Series = S, Output = T> + 'static,
    R: TradingRecord<T, CM, HM, S>,
{
    price_indicator: Arc<I>,
    bar_count: usize,
    loss_percentage: T,
    base_rule: BaseRule<Self>,
    _phantom: PhantomData<(T, CM, HM, S, R)>,
}

impl<T, CM, HM, S, I, R> TrailingStopLossRule<T, CM, HM, S, I, R>
where
    T: TrNum + Clone + 'static,
    CM: CostModel<T> + Clone,
    HM: CostModel<T> + Clone,
    S: BarSeries<T> + 'static,
    I: Indicator<Num = T, Series = S, Output = T> + 'static,
    R: TradingRecord<T, CM, HM, S>,
{
    /// 构造器：指定 bar_count
    pub fn with_bar_count(price_indicator: Arc<I>, loss_percentage: T, bar_count: usize) -> Self {
        Self {
            price_indicator,
            bar_count,
            loss_percentage,
            base_rule: BaseRule::new("TrailingStopLossRule"),
            _phantom: PhantomData,
        }
    }

    /// 构造器：不限制 bar_count（使用 usize::MAX）
    pub fn new(price_indicator: Arc<I>, loss_percentage: T) -> Self {
        Self::with_bar_count(price_indicator, loss_percentage, usize::MAX)
    }

    /// 计算买入是否触发移动止损
    fn is_buy_satisfied(&self, current_price: T, index: usize, position_index: usize) -> bool {
        let look_back = std::cmp::min(index - position_index + 1, self.bar_count);
        let highest = HighestValueIndicator::new(self.price_indicator.clone(), look_back);

        highest
            .get_value(index)
            .ok()
            .and_then(|highest_val| {
                highest
                    .bar_series()
                    .with_ref(|s| s.num_factory())
                    .ok()
                    .map(|num_factory| {
                        let hundred = num_factory.num_of_f64(100.0);
                        let ratio = (hundred.clone() - self.loss_percentage.clone()) / hundred;
                        let threshold = highest_val * ratio;
                        current_price <= threshold
                    })
            })
            .unwrap_or(false)
    }

    /// 计算卖出是否触发移动止损
    fn is_sell_satisfied(&self, current_price: T, index: usize, position_index: usize) -> bool {
        let look_back = std::cmp::min(index - position_index + 1, self.bar_count);

        let lowest = LowestValueIndicator::new(self.price_indicator.clone(), look_back);

        lowest
            .get_value(index)
            .ok()
            .and_then(|lowest_val| {
                lowest
                    .bar_series()
                    .with_ref(|s| s.num_factory())
                    .ok()
                    .map(|num_factory| {
                        let hundred = num_factory.num_of_f64(100.0);
                        let ratio = (hundred.clone() + self.loss_percentage.clone()) / hundred;
                        let threshold = lowest_val * ratio;
                        current_price >= threshold
                    })
            })
            .unwrap_or(false)
    }
}

impl<T, CM, HM, S, I, R> Rule for TrailingStopLossRule<T, CM, HM, S, I, R>
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
        let satisfied = trading_record
            .and_then(|tr| {
                let pos = tr.get_current_position();
                if pos.is_opened() {
                    pos.entry().and_then(|entry| {
                        // 安全获取当前价格，若出错返回 None
                        self.price_indicator
                            .get_value(index)
                            .ok()
                            .map(|current_price| {
                                if entry.is_buy() {
                                    self.is_buy_satisfied(current_price, index, entry.index())
                                } else {
                                    self.is_sell_satisfied(current_price, index, entry.index())
                                }
                            })
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
