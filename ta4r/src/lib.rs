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

mod aggregator;
pub mod bar;
pub mod indicators;
pub mod num;
mod rule;
mod trade;
mod position;

pub trait TradingRecord<T: TrNum>: Clone {
    // fn get_starting_type(&self) -> TradeType;
    // fn get_name(&self) -> &str;
    //
    // fn operate(&mut self, index: usize) {
    //     self.operate_full(index, T::nan(), T::nan());
    // }
    // fn operate_full(&mut self, index: usize, price: T, amount: T);
    //
    // fn enter(&mut self, index: usize) -> bool {
    //     self.enter_full(index, T::nan(), T::nan())
    // }
    // fn enter_full(&mut self, index: usize, price: T, amount: T) -> bool;
    //
    // fn exit(&mut self, index: usize) -> bool {
    //     self.exit_full(index, T::nan(), T::nan())
    // }
    // fn exit_full(&mut self, index: usize, price: T, amount: T) -> bool;
    //
    // fn is_closed(&self) -> bool {
    //     !self.get_current_position().is_opened()
    // }
    //
    // fn transaction_cost_model(&self) -> &dyn CostModel<T>;
    // fn holding_cost_model(&self) -> &dyn CostModel<T>;
    //
    // fn positions(&self) -> &[Position<T>];
    // fn position_count(&self) -> usize {
    //     self.positions().len()
    // }
    //
    // fn get_current_position(&self) -> &Position<T>;
    // fn get_last_position(&self) -> Option<&Position<T>>;
    //
    // fn trades(&self) -> &[Trade<T>];
    // fn last_trade(&self) -> Option<&Trade<T>>;
    // fn last_trade_of_type(&self, trade_type: TradeType) -> Option<&Trade<T>>;
    // fn last_entry(&self) -> Option<&Trade<T>>;
    // fn last_exit(&self) -> Option<&Trade<T>>;
    //
    // fn start_index(&self) -> Option<usize>;
    // fn end_index(&self) -> Option<usize>;
    //
    // fn start_index_in_series<S: BarSeries<T>>(&self, series: &S) -> usize {
    //     self.start_index()
    //         .map(|i| i.max(series.begin_index()))
    //         .unwrap_or(series.begin_index())
    // }
    //
    // fn end_index_in_series<S: BarSeries<T>>(&self, series: &S) -> usize {
    //     self.end_index()
    //         .map(|i| i.min(series.end_index()))
    //         .unwrap_or(series.end_index())
    // }
}
