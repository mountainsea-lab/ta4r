use crate::num::TrNum;
use crate::position::Position;

pub mod cost_model;

pub trait CostModel<T: TrNum> {
    /// 计算给定 Position 到指定索引的成本
    fn calculate_with_index(&self, position: &Position<T, Self, Self>, final_index: usize) -> T
    where
        Self: Sized;

    /// 计算给定 Position 的成本
    fn calculate_position(&self, position: &Position<T, Self, Self>) -> T
    where
        Self: Sized;

    /// 计算单次交易的成本
    fn calculate_trade(&self, price: T, amount: T) -> T
    where
        Self: Sized;

    /// 判断两个成本模型是否相等
    fn equals(&self, other: &Self) -> bool
    where
        Self: Sized;
}
