// use std::marker::PhantomData;
// use crate::indicators::helpers::close_price_indicator::ClosePriceIndicator;
// use crate::rule::base_rule::BaseRule;
//
// pub struct AverageTrueRangeStopGainRule<'a, TR>
// where
//     TR: TradingRec<'a>,
// {
//     base: BaseRule<'a, Self>,
//     reference_price: ClosePriceIndicator<'a, TR::Num, TR::Series<'a>>,
//     atr: AtrIndicator<'a, TR::Num, TR::Series<'a>>,
//     atr_coefficient: TR::Num,
//     _marker: PhantomData<&'a TR>,
// }
//
// impl<'a, TR> AverageTrueRangeStopGainRule<'a, TR>
// where
//     TR: TradingRec<'a>,
// {
//     /// 构造函数：默认 reference_price 为 ClosePriceIndicator
//     pub fn new(series: &'a TR::Series<'a>, atr_bar_count: usize, atr_coefficient: TR::Num) -> Self {
//         let reference_price = ClosePriceIndicator::new(series);
//         let atr = ATRIndicator::new(series, atr_bar_count);
//
//         Self {
//             base: BaseRule::new("AverageTrueRangeStopGainRule"),
//             reference_price,
//             atr,
//             atr_coefficient,
//             _marker: PhantomData,
//         }
//     }
//
//     /// 判断规则是否满足（依赖交易记录）
//     pub fn is_satisfied(&self, index: usize, trading_record: Option<&TR>) -> bool {
//         let mut satisfied = false;
//
//         if let Some(record) = trading_record {
//             if let Some(position) = record.get_current_position() {
//                 if position.is_opened() {
//                     let entry_price = position.get_entry().get_net_price();
//                     let current_price = self.reference_price.get_value(index).unwrap();
//                     let gain_threshold = self.atr.get_value(index).unwrap() * self.atr_coefficient.clone();
//
//                     satisfied = if position.get_entry().is_buy() {
//                         current_price >= entry_price + gain_threshold
//                     } else {
//                         current_price <= entry_price - gain_threshold
//                     };
//                 }
//             }
//         }
//
//         self.base.trace_is_satisfied(index, satisfied);
//         satisfied
//     }
// }
