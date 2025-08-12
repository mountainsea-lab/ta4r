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
use crate::analysis::CostModel;
use crate::analysis::cost::CostContext;
use crate::num::TrNum;

// FixedTransactionCostModel 实现
#[derive(Debug, Clone)]
pub struct FixedTransactionCostModel<T: TrNum + 'static> {
    pub fee_per_trade: T,
}

impl<T: TrNum + 'static> FixedTransactionCostModel<T> {
    pub fn new(fee_per_trade: T) -> Self {
        Self { fee_per_trade }
    }
}

impl<T: TrNum + 'static> CostModel<T> for FixedTransactionCostModel<T> {
    // 固定费用模型，持仓成本与索引无关，直接调用 calculate_position
    fn calculate_with_index(&self, ctx: &CostContext<T>) -> T {
        self.calculate_position(ctx)
    }

    // 计算持仓成本，开仓费用1倍，平仓费用2倍
    fn calculate_position(&self, ctx: &CostContext<T>) -> T {
        let multiplier = if ctx.is_closed {
            T::from_i32(2).unwrap_or_else(|| T::one())
        } else {
            T::one()
        };
        self.fee_per_trade.multiplied_by(&multiplier)
    }

    // 单笔交易固定费用，不考虑价格和数量
    fn calculate_trade(&self, _price: &T, _amount: &T) -> T {
        self.fee_per_trade.clone()
    }

    // 判断两个模型是否相等，基于fee_per_trade比较
    fn equals(&self, other: &Self) -> bool {
        self.fee_per_trade == other.fee_per_trade
    }
}
