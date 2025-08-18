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
use crate::rule::{Rule, base_rule::BaseRule};

/// BooleanRule: 总是返回固定 true/false 的规则
pub struct BooleanRule<'a, R>
where
    R: Rule<'a>,
{
    base: BaseRule<'a, R>,
    satisfied: bool,
}

impl<'a, R> BooleanRule<'a, R>
where
    R: Rule<'a>,
{
    /// 构造函数
    pub fn new(satisfied: bool) -> Self {
        Self {
            base: BaseRule::new("BooleanRule"),
            satisfied,
        }
    }

    /// 总是返回 true 的静态实例
    pub fn true_rule() -> Self {
        Self::new(true)
    }

    /// 总是返回 false 的静态实例
    pub fn false_rule() -> Self {
        Self::new(false)
    }
}

impl<'a, R> Rule<'a> for BooleanRule<'a, R>
where
    R: Rule<'a>,
{
    type Num = R::Num;
    type CostBuy = R::CostBuy;
    type CostSell = R::CostSell;
    type Series = R::Series;
    type TradingRec = R::TradingRec;

    fn is_satisfied_with_record(
        &self,
        index: usize,
        _trading_record: Option<&Self::TradingRec>,
    ) -> bool {
        self.base.trace_is_satisfied(index, self.satisfied);
        self.satisfied
    }
}
