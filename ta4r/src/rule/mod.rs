use crate::analysis::CostModel;
use crate::bar::types::BarSeries;
use crate::num::TrNum;
use crate::TradingRecord;

/// 一条交易规则（Trading Rule）
///
/// 用于构建交易策略（Strategy），规则之间可以组合成更复杂的逻辑规则。
pub trait Rule<'a, N, CM, HM, S, TR>
where
    N: TrNum + 'static,
    CM: CostModel<N> + Clone,
    HM: CostModel<N> + Clone,
    S: BarSeries<'a, N>,
    TR: TradingRecord<'a, N, CM, HM, S>,
{
    /// 规则在给定索引下是否满足（不依赖交易记录）
    fn is_satisfied(&self, index: usize) -> bool {
        self.is_satisfied_with_record(index, None)
    }

    /// 规则在给定索引下是否满足（可选提供交易记录）
    fn is_satisfied_with_record(
        &self,
        index: usize,
        trading_record: Option<&TR>,
    ) -> bool;
    //
    // /// 与另一条规则组合成 AND 规则
    // fn and<R>(self, other: R) -> AndRule<'a, N, CM, HM, S, TR, Self, R>
    // where
    //     Self: Sized,
    //     R: Rule<'a, N, CM, HM, S, TR>,
    // {
    //     AndRule::new(self, other)
    // }
    //
    // /// 与另一条规则组合成 OR 规则
    // fn or<R>(self, other: R) -> OrRule<'a, N, CM, HM, S, TR, Self, R>
    // where
    //     Self: Sized,
    //     R: Rule<'a, N, CM, HM, S, TR>,
    // {
    //     OrRule::new(self, other)
    // }
    //
    // /// 与另一条规则组合成 XOR 规则
    // fn xor<R>(self, other: R) -> XorRule<'a, N, CM, HM, S, TR, Self, R>
    // where
    //     Self: Sized,
    //     R: Rule<'a, N, CM, HM, S, TR>,
    // {
    //     XorRule::new(self, other)
    // }
    //
    // /// 取反规则
    // fn negation(self) -> NotRule<'a, N, CM, HM, S, TR, Self>
    // where
    //     Self: Sized,
    // {
    //     NotRule::new(self)
    // }
}
