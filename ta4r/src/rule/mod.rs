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

pub mod and_rule;
pub mod average_true_range_stop_gain_rule;
pub mod base_rule;
pub mod bool_rule;
pub mod chain_rule;
pub mod fixed_rule;
pub mod helper;
pub mod just_once_rule;
pub mod not_rule;
pub mod or_rule;
pub mod wait_for_rule;
pub mod xor_rule;

use crate::TradingRecord;
use crate::analysis::CostModel;
use crate::bar::types::BarSeries;
use crate::num::TrNum;
use crate::rule::and_rule::AndRule;
use crate::rule::not_rule::NotRule;
use crate::rule::or_rule::OrRule;
use crate::rule::xor_rule::XorRule;

/// 一条交易规则（Trading Rule）
///
/// 用于构建交易策略（Strategy），规则之间可以组合成更复杂的逻辑规则。
pub trait Rule {
    /// 数值类型（例如 DecimalNum、f64 等）
    type Num: TrNum + 'static;
    /// 买入成本模型
    type CostBuy: CostModel<Self::Num> + Clone;
    /// 卖出成本模型
    type CostSell: CostModel<Self::Num> + Clone;
    /// Bar 序列类型
    type Series: BarSeries<Self::Num> + 'static;
    /// 交易记录类型
    type TradingRec: TradingRecord<Self::Num, Self::CostBuy, Self::CostSell, Self::Series>;

    /// 规则在给定索引下是否满足（不依赖交易记录）
    fn is_satisfied(&self, index: usize) -> bool {
        self.is_satisfied_with_record(index, None)
    }

    /// 规则在给定索引下是否满足（可选提供交易记录）
    fn is_satisfied_with_record(
        &self,
        index: usize,
        trading_record: Option<&Self::TradingRec>,
    ) -> bool;

    /// 与另一条规则组合成 AND 规则
    fn and<R>(self, other: R) -> AndRule<Self, R>
    where
        Self: Sized,
        R: Rule<
                Num = Self::Num,
                CostBuy = Self::CostBuy,
                CostSell = Self::CostSell,
                Series = Self::Series,
                TradingRec = Self::TradingRec,
            >,
    {
        AndRule::new(self, other)
    }

    /// 与另一条规则组合成 OR 规则
    fn or<R>(self, other: R) -> OrRule<Self, R>
    where
        Self: Sized,
        R: Rule<
                Num = Self::Num,
                CostBuy = Self::CostBuy,
                CostSell = Self::CostSell,
                Series = Self::Series,
                TradingRec = Self::TradingRec,
            >,
    {
        OrRule::new(self, other)
    }

    /// 与另一条规则组合成 XOR 规则
    fn xor<R>(self, other: R) -> XorRule<Self, R>
    where
        Self: Sized,
        R: Rule<
                Num = Self::Num,
                CostBuy = Self::CostBuy,
                CostSell = Self::CostSell,
                Series = Self::Series,
                TradingRec = Self::TradingRec,
            >,
    {
        XorRule::new(self, other)
    }

    /// 取反规则
    fn negation(self) -> NotRule<Self>
    where
        Self: Sized,
    {
        NotRule::new(self)
    }
}
