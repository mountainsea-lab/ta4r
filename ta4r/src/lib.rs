/*!
 * MIT License
 *
 * Copyright (c) 2025 Mountainsea
 * Based on ta4j (c) 2017–2025 Ta4j Organization & respective authors (see AUTHORS)
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

use crate::num::TrNum;

pub mod aggregator;
pub mod analysis;
pub mod bar;
pub mod indicators;
pub mod num;
pub mod position;
pub mod rule;
pub mod trade;

use crate::analysis::CostModel;
use crate::bar::types::BarSeries;
use crate::position::Position;
use crate::trade::{Trade, TradeType};

pub trait TradingRecord<'a, T, CM, HM, S>
where
    T: TrNum + 'static,
    CM: CostModel<T> + Clone,
    HM: CostModel<T> + Clone,
    S: BarSeries<'a, T> + 'a,
{
    // 起始交易类型
    fn get_starting_type(&self) -> TradeType;

    // 交易记录名称
    fn get_name(&self) -> &str;

    // 在 index 下执行操作，使用默认价格和数量
    fn operate(&mut self, index: usize);

    // 在 index 下执行操作，指定价格和数量
    fn operate_with_price_amount(&mut self, index: usize, price: T, amount: T);

    // 入场交易（默认价格和数量）
    fn enter(&mut self, index: usize) -> bool;

    // 入场交易，指定价格和数量
    fn enter_with_price_amount(&mut self, index: usize, price: T, amount: T) -> bool;

    // 退出交易（默认价格和数量）
    fn exit(&mut self, index: usize) -> bool;

    // 退出交易，指定价格和数量
    fn exit_with_price_amount(&mut self, index: usize, price: T, amount: T) -> bool;

    // 当前是否持仓已关闭
    fn is_closed(&self) -> bool;

    // 获取交易成本模型
    fn get_transaction_cost_model(&self) -> &CM;

    // 获取持有成本模型
    fn get_holding_cost_model(&self) -> &HM;

    // 已记录的所有已关闭持仓
    fn get_positions(&self) -> &[Position<'a, T, CM, HM, S>];

    // 已记录持仓数量
    fn get_position_count(&self) -> usize {
        self.get_positions().len()
    }

    // 当前持仓
    fn get_current_position(&self) -> &Position<'a, T, CM, HM, S>;

    // 最近关闭的持仓（无则返回 None）
    fn get_last_position(&self) -> Option<&Position<'a, T, CM, HM, S>> {
        self.get_positions().last()
    }

    // 所有交易
    fn get_trades(&self) -> &[Trade<'a, T, CM, S>];

    // 最近交易（无则返回 None）
    fn get_last_trade(&self) -> Option<&Trade<'a, T, CM, S>>;

    // 指定类型的最近交易（无则返回 None）
    fn get_last_trade_of_type(&self, trade_type: TradeType) -> Option<&Trade<'a, T, CM, S>>;

    // 最近入场交易（无则返回 None）
    fn get_last_entry(&self) -> Option<&Trade<'a, T, CM, S>>;

    // 最近退出交易（无则返回 None）
    fn get_last_exit(&self) -> Option<&Trade<'a, T, CM, S>>;

    // 记录起始索引（可选）
    fn get_start_index(&self) -> Option<usize>;

    // 记录结束索引（可选）
    fn get_end_index(&self) -> Option<usize>;

    // 结合 BarSeries 计算起始索引
    fn get_start_index_with_series(&self, series: &S) -> Option<usize> {
        match (self.get_start_index(), series.get_begin_index()) {
            (Some(s), Some(begin)) => Some(std::cmp::max(s, begin)),
            (Some(s), None) => Some(s),
            (None, Some(begin)) => Some(begin),
            (None, None) => None,
        }
    }

    // 结合 BarSeries 计算结束索引
    fn get_end_index_with_series(&self, series: &S) -> Option<usize> {
        match (self.get_end_index(), series.get_end_index()) {
            (Some(e), Some(series_end)) => Some(std::cmp::min(e, series_end)),
            (Some(e), None) => Some(e),
            (None, Some(series_end)) => Some(series_end),
            (None, None) => None,
        }
    }
}
