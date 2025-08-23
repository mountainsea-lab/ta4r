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

/// 一个取反（NOT）规则
///
/// 当目标规则不满足时返回 `true`；
/// 当目标规则满足时返回 `false`。
pub struct NotRule<L>
where
    L: Rule,
{
    base: BaseRule<L>,
    rule_to_negate: L,
}

impl<L> NotRule<L>
where
    L: Rule,
{
    /// 创建一个 NOT 规则
    pub fn new(rule_to_negate: L) -> Self {
        Self {
            base: BaseRule::new("NotRule"),
            rule_to_negate,
        }
    }

    /// 获取被取反的规则
    pub fn rule_to_negate(&self) -> &L {
        &self.rule_to_negate
    }
}

impl<L> Rule for NotRule<L>
where
    L: Rule,
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
        let satisfied = !self
            .rule_to_negate
            .is_satisfied_with_record(index, trading_record);
        self.base.trace_is_satisfied(index, satisfied);
        satisfied
    }
}
