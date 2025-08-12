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

pub mod fixed_transaction_cost_model;
pub mod zero_cost_model;

/// 用于封装计算成本所需的所有参数，后续方便扩展 比如解耦: cost与Position的耦合
pub struct CostContext<T: TrNum + 'static> {
    pub entry_price: T,
    pub amount: T,
    pub entry_index: Option<usize>, // 可选，因为有的方法不需要索引
    pub final_index: Option<usize>, // 指定索引
    pub is_closed: bool,
    // 后续如果有更多状态，直接加字段即可
    // pub extra_fee: T,
    // pub timestamp: u64,
    // ...
}

impl<T: TrNum> CostContext<T> {
    pub fn build(
        entry_price: T,
        amount: T,
        entry_index: Option<usize>,
        final_index: Option<usize>,
        is_closed: bool,
    ) -> Self {
        Self {
            entry_price,
            amount,
            entry_index,
            final_index,
            is_closed,
        }
    }
}
