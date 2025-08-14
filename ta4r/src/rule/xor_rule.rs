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
