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
use std::sync::Arc;

pub mod aggregator;
pub mod analysis;
pub mod bar;
pub mod base_trading_record;
pub mod indicators;
pub mod num;
pub mod position;
pub mod rule;
pub mod strategy;
pub mod trade;

use crate::analysis::CostModel;
use crate::bar::types::BarSeries;
use crate::position::Position;
use crate::trade::{Trade, TradeType};

pub trait TradingRecord<T, CM, HM, S>
where
    T: TrNum + 'static,
    CM: CostModel<T> + Clone,
    HM: CostModel<T> + Clone,
    S: BarSeries<T> + 'static,
{
    /// 起始交易类型（买入或卖出），决定第一笔交易是买入还是卖出
    fn starting_type(&self) -> TradeType;

    /// 交易记录名称
    fn name(&self) -> &str;

    /// 在给定 index 下执行操作（入场或出场），使用默认价格和数量
    fn operate(&mut self, index: usize);

    /// 在给定 index 下执行操作（入场或出场），指定价格和数量
    fn operate_with_price_amount(&mut self, index: usize, price: T, amount: T);

    /// 入场交易（默认价格和数量），返回是否成功
    fn enter(&mut self, index: usize) -> bool;

    /// 入场交易（指定价格和数量），返回是否成功
    fn enter_with_price_amount(&mut self, index: usize, price: T, amount: T) -> bool;

    /// 退出交易（默认价格和数量），返回是否成功
    fn exit(&mut self, index: usize) -> bool;

    /// 退出交易（指定价格和数量），返回是否成功
    fn exit_with_price_amount(&mut self, index: usize, price: T, amount: T) -> bool;

    /// 当前是否持仓已关闭（无持仓或最后一个持仓已完成）
    fn is_closed(&self) -> bool;

    /// 获取交易成本模型（手续费）
    fn transaction_cost_model(&self) -> &CM;

    /// 获取持有成本模型（持仓成本，如资金占用）
    fn holding_cost_model(&self) -> &HM;

    /// 已记录的所有持仓（已完成 + 当前未完成）
    fn positions(&self) -> &[Arc<Position<T, CM, HM, S>>];

    /// 已记录持仓数量
    fn position_count(&self) -> usize {
        self.positions().len()
    }

    /// 当前持仓（可能未完成），总是返回一个引用
    fn current_position(&self) -> &Arc<Position<T, CM, HM, S>>;

    /// 最近的一个持仓（可能已关闭），如果没有则返回 None
    fn last_position(&self) -> Option<&Arc<Position<T, CM, HM, S>>> {
        self.positions().last()
    }

    /// 所有交易（包括入场和出场）
    fn trades(&self) -> &[Arc<Trade<T, CM, S>>];

    /// 最近的交易（无则返回 None）
    fn last_trade(&self) -> Option<&Arc<Trade<T, CM, S>>>;

    /// 指定类型（买入/卖出）的最近交易（无则返回 None）
    fn last_trade_of_type(&self, trade_type: TradeType) -> Option<&Arc<Trade<T, CM, S>>>;

    /// 最近的入场交易（无则返回 None）
    fn last_entry(&self) -> Option<&Arc<Trade<T, CM, S>>>;

    /// 最近的出场交易（无则返回 None）
    fn last_exit(&self) -> Option<&Arc<Trade<T, CM, S>>>;

    /// 记录起始索引（可选）
    fn start_index(&self) -> Option<usize>;

    /// 记录结束索引（可选）
    fn end_index(&self) -> Option<usize>;

    /// 结合 BarSeries 计算起始索引（取两者的最大值）
    fn start_index_with_series(&self, series: &S) -> Option<usize> {
        match (self.start_index(), series.get_begin_index()) {
            (Some(s), Some(begin)) => Some(std::cmp::max(s, begin)),
            (Some(s), None) => Some(s),
            (None, Some(begin)) => Some(begin),
            (None, None) => None,
        }
    }

    /// 结合 BarSeries 计算结束索引（取两者的最小值）
    fn end_index_with_series(&self, series: &S) -> Option<usize> {
        match (self.end_index(), series.get_end_index()) {
            (Some(e), Some(series_end)) => Some(std::cmp::min(e, series_end)),
            (Some(e), None) => Some(e),
            (None, Some(series_end)) => Some(series_end),
            (None, None) => None,
        }
    }
}
