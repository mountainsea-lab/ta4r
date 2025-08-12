use crate::analysis::cost::CostContext;
use crate::num::TrNum;

pub mod cost;

pub trait CostModel<T: TrNum + 'static> {
    /// 计算给定持仓的成本
    fn calculate_with_index(&self, ctx: &CostContext<T>) -> T;

    /// 计算持仓的当前成本
    fn calculate_position(&self, ctx: &CostContext<T>) -> T;

    /// 计算单次交易的成本
    fn calculate_trade(&self, price: &T, amount: &T) -> T;

    /// 判断两个成本模型是否相等
    fn equals(&self, other: &Self) -> bool;
}
