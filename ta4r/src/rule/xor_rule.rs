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
use crate::rule::Rule;
use crate::rule::base_rule::BaseRule;

/// 一个 XOR 组合规则
///
/// 仅当两个规则中**只有一个**满足时返回 true；
/// 如果都不满足或都满足则返回 false。
pub struct XorRule<'a, L, R>
where
    L: Rule<'a>,
    R: Rule<
            'a,
            Num = L::Num,
            CostBuy = L::CostBuy,
            CostSell = L::CostSell,
            Series = L::Series,
            TradingRec = L::TradingRec,
        >,
{
    base: BaseRule<'a, L>,
    left: L,
    right: R,
}

impl<'a, L, R> XorRule<'a, L, R>
where
    L: Rule<'a>,
    R: Rule<
            'a,
            Num = L::Num,
            CostBuy = L::CostBuy,
            CostSell = L::CostSell,
            Series = L::Series,
            TradingRec = L::TradingRec,
        >,
{
    pub fn new(left: L, right: R) -> Self {
        Self {
            base: BaseRule::new("XorRule"),
            left,
            right,
        }
    }

    /// 获取左侧规则
    pub fn left(&self) -> &L {
        &self.left
    }

    /// 获取右侧规则
    pub fn right(&self) -> &R {
        &self.right
    }
}

impl<'a, L, R> Rule<'a> for XorRule<'a, L, R>
where
    L: Rule<'a>,
    R: Rule<
            'a,
            Num = L::Num,
            CostBuy = L::CostBuy,
            CostSell = L::CostSell,
            Series = L::Series,
            TradingRec = L::TradingRec,
        >,
{
    type Num = L::Num;
    type CostBuy = L::CostBuy;
    type CostSell = L::CostSell;
    type Series = L::Series;
    type TradingRec = L::TradingRec;

    fn is_satisfied_with_record(
        &self,
        index: usize,
        trading_record: Option<&Self::TradingRec>,
    ) -> bool {
        let left_satisfied = self.left.is_satisfied_with_record(index, trading_record);
        let right_satisfied = self.right.is_satisfied_with_record(index, trading_record);
        let satisfied = left_satisfied != right_satisfied; // 逻辑异或

        self.base.trace_is_satisfied(index, satisfied);
        satisfied
    }
}
