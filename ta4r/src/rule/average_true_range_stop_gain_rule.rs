// use crate::TradingRecord;
// use crate::analysis::CostModel;
// use crate::bar::types::BarSeries;
// use crate::indicators::Indicator;
// use crate::indicators::atr_indicator::ATRIndicator;
// use crate::num::TrNum;
// use crate::num::types::{trnum_add, trnum_sub};
// use crate::rule::Rule;
// use crate::rule::base_rule::BaseRule;
// use crate::trade::TradeType;
// use std::marker::PhantomData;
// use std::sync::Arc;
//
// pub struct AverageTrueRangeStopGainRule<T, CM, HM, S, I, R>
// where
//     T: TrNum + Clone + 'static,
//     CM: CostModel<T> + Clone,
//     HM: CostModel<T> + Clone,
//     S: BarSeries<T> + 'static,
//     I: Indicator<Num = T, Series = S, Output = T>,
//     R: TradingRecord<T, CM, HM, S>,
// {
//     stop_gain_threshold: Arc<ATRIndicator<T, S>>,
//     reference_price: Arc<I>,
//     base_rule: BaseRule<Self>,
//     _phantom: PhantomData<(CM, HM, R)>,
// }
//
// impl<T, CM, HM, S, I, R> Rule for AverageTrueRangeStopGainRule<T, CM, HM, S, I, R>
// where
//     T: TrNum + Clone + 'static,
//     CM: CostModel<T> + Clone,
//     HM: CostModel<T> + Clone,
//     S: BarSeries<T> + 'static,
//     I: Indicator<Num = T, Series = S, Output = T>,
//     R: TradingRecord<T, CM, HM, S>,
// {
//     type Num = T;
//     type CostBuy = CM;
//     type CostSell = HM;
//     type Series = S;
//     type TradingRec = R;
//     fn is_satisfied_with_record(
//         &self,
//         index: usize,
//         trading_record: Option<&Self::TradingRec>,
//     ) -> bool {
//         let mut satisfied = false;
//
//         if let Some(record) = trading_record {
//             let current_position = record.get_current_position();
//             if current_position.is_opened() {
//                 if let Some(entry_trade) = current_position.entry() {
//                     let entry_price = entry_trade.net_price(); // 引用
//                     // 直接获取值
//                     let current_price_val = match self.reference_price.get_value(index) {
//                         Ok(p) => p,
//                         Err(_) => return false,
//                     };
//                     let threshold_val = match self.stop_gain_threshold.get_value(index) {
//                         Ok(t) => t,
//                         Err(_) => return false,
//                     };
//
//                     satisfied = match entry_trade.trade_type() {
//                         TradeType::Buy => {
//                             let target = trnum_add(entry_price, &threshold_val);
//                             current_price_val >= target
//                         }
//                         TradeType::Sell => {
//                             let target = trnum_sub(entry_price, &threshold_val);
//                             current_price_val <= target
//                         }
//                     };
//                 }
//             }
//         }
//
//         self.base_rule.trace_is_satisfied(index, satisfied);
//         satisfied
//     }
// }

/*!
 * AverageTrueRangeStopGainRule
 *
 * Rust 实盘规则实现，功能对应 Java ta4j 版本：
 * - ATR 指标乘系数作为止盈阈值
 * - 可指定参考价格，默认使用收盘价
 * - 零克隆获取净成交价格，支持任意 TrNum 类型
 * - 支持 TradingRecord 检查当前仓位
 */
use crate::TradingRecord;
use crate::analysis::CostModel;
use crate::bar::types::BarSeries;
use crate::indicators::Indicator;
use crate::indicators::atr_indicator::ATRIndicator;
use crate::indicators::helpers::close_price_indicator::ClosePriceIndicator;
use crate::indicators::helpers::constant_indicator::ConstantIndicator;
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

pub struct AverageTrueRangeStopGainRule<T, CM, HM, S, I, R>
where
    T: TrNum + Clone + 'static,
    CM: CostModel<T> + Clone,
    HM: CostModel<T> + Clone,
    S: BarSeries<T> + 'static,
    I: Indicator<Num = T, Series = S, Output = T> + 'static,
    R: TradingRecord<T, CM, HM, S>,
{
    stop_gain_threshold: Arc<BinaryOperation<T, ATRIndicator<T, S>, ConstantIndicator<T, S>>>,
    reference_price: Arc<I>,
    base_rule: BaseRule<Self>,
    _phantom: PhantomData<(CM, HM, R)>,
}

impl<T, CM, HM, S, I, R> AverageTrueRangeStopGainRule<T, CM, HM, S, I, R>
where
    T: TrNum + Clone + 'static,
    CM: CostModel<T> + Clone,
    HM: CostModel<T> + Clone,
    S: BarSeries<T> + 'static,
    I: Indicator<Num = T, Series = S, Output = T> + 'static,
    R: TradingRecord<T, CM, HM, S>,
{
    /// 通用构造函数：指定参考价格和 ATR 系数
    // pub fn new(
    //     series: Arc<RwLock<S>>,
    //     reference_price: Arc<I>,
    //     atr_bar_count: usize,
    //     atr_coefficient: T,
    // ) -> Result<Self, IndicatorError> {
    //     let tr_indicator = Arc::new(TRIndicator::from_shared(series.clone()));
    //     let atr_indicator = Arc::new(ATRIndicator::from_tr(tr_indicator, atr_bar_count));
    //
    //     // 将 atr_coefficient 包装为 ConstantIndicator
    //     let right_indicator = ConstantIndicator::new(atr_indicator.bar_series(), atr_coefficient);
    //     let stop_gain_threshold =
    //         BinaryOperation::product(&*atr_indicator, &right_indicator);
    //     Ok(Self {
    //         stop_gain_threshold: Arc::new(stop_gain_threshold),
    //         reference_price,
    //         base_rule: BaseRule::new("AverageTrueRangeStopGainRule"),
    //         _phantom: PhantomData,
    //     })
    // }

    pub fn new(
        series: Arc<RwLock<S>>,
        reference_price: Arc<I>,
        atr_bar_count: usize,
        atr_coefficient: T,
    ) -> Result<Self, IndicatorError> {
        // 构造 TR 指标
        let tr_indicator = Arc::new(TRIndicator::from_shared(series.clone()));

        // 构造 ATR 指标
        let atr_indicator = Arc::new(ATRIndicator::from_tr(tr_indicator, atr_bar_count));

        // 将 atr_coefficient 包装为常数指标
        let right_indicator = Arc::new(ConstantIndicator::new(
            atr_indicator.bar_series(),
            atr_coefficient,
        ));

        // 构造二元运算指标（ATR * 常数）
        let stop_gain_threshold = Arc::new(BinaryOperation::product(
            atr_indicator.clone(),
            right_indicator,
        ));

        Ok(Self {
            stop_gain_threshold,
            reference_price,
            base_rule: BaseRule::new("AverageTrueRangeStopGainRule"),
            _phantom: PhantomData,
        })
    }

    /// 构造函数：默认参考价格为 ClosePriceIndicator
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

impl<T, CM, HM, S, I, R> Rule for AverageTrueRangeStopGainRule<T, CM, HM, S, I, R>
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
