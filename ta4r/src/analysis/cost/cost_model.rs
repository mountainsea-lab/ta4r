use crate::analysis::cost::CostModel;
use crate::num::TrNum;
use crate::position::Position;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ZeroCostModel;

impl<T: TrNum> CostModel<T> for ZeroCostModel {
    fn calculate_with_index(&self, _position: &Position<T, Self, Self>, _final_index: usize) -> T {
        T::zero()
    }

    fn calculate_position(&self, _position: &Position<T, Self, Self>) -> T {
        T::zero()
    }

    fn calculate_trade(&self, _price: T, _amount: T) -> T {
        T::zero()
    }

    fn equals(&self, _other: &Self) -> bool {
        true
    }
}
