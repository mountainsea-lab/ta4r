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
use std::cell::Cell;

/// 一次性规则：首次满足后，之后永远返回 false
pub struct JustOnceRule<R>
where
    R: Rule,
{
    base: BaseRule,
    rule: Option<R>,
    satisfied: Cell<bool>,
}

impl<R> JustOnceRule<R>
where
    R: Rule,
{
    /// 构造器：带子规则
    pub fn new(rule: R) -> Self {
        Self {
            base: BaseRule::new("JustOnceRule"),
            rule: Some(rule),
            satisfied: Cell::new(false),
        }
    }

    /// 构造器：无子规则，第一次调用总是返回 true
    pub fn once() -> Self {
        Self {
            base: BaseRule::new("JustOnceRule"),
            rule: None,
            satisfied: Cell::new(false),
        }
    }

    fn trace_is_satisfied(&self, index: usize, is_satisfied: bool) {
        self.base.trace_is_satisfied(index, is_satisfied);
    }
}

impl<R> Clone for JustOnceRule<R>
where
    R: Rule,
{
    fn clone(&self) -> Self {
        Self {
            base: self.base.clone(),
            rule: self.rule.clone(),
            satisfied: Cell::new(self.satisfied.get()),
        }
    }
}

impl<R> Rule for JustOnceRule<R>
where
    R: Rule,
{
    type Num = R::Num;
    type CostBuy = R::CostBuy;
    type CostSell = R::CostSell;
    type Series = R::Series;
    type TradingRec = R::TradingRec;

    fn is_satisfied_with_record(
        &self,
        index: usize,
        trading_record: Option<&R::TradingRec>,
    ) -> bool {
        if self.satisfied.get() {
            return false; // 已满足，直接返回 false
        }

        let result = self.rule.as_ref().map_or_else(
            || {
                // 没有子规则，第一次直接返回 true
                self.satisfied.set(true);
                self.trace_is_satisfied(index, true);
                true
            },
            |r| r.is_satisfied_with_record(index, trading_record),
        );

        self.satisfied.set(result);
        result
    }
}
