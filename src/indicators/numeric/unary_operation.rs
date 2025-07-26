use crate::indicators::types::{BinaryOp, IndicatorError};
use crate::indicators::Indicator;
use crate::num::TrNum;
use std::marker::PhantomData;

/// 延迟执行的二元操作指标（不缓存）
// pub struct BinaryOperationIndicator<'s, T, L, R, Op>
// where
//     T: TrNum,
//     L: Indicator<Num = T> + 's,
//     R: Indicator<Num = T> + 's,
//     Op: Fn(T, T) -> T,
// {
//     left: L,
//     right: R,
//     op: Op,
//     _phantom: PhantomData<&'s T>,
// }
//
// impl<'s, T, L, R, Op> BinaryOperationIndicator<'s, T, L, R, Op>
// where
//     T: TrNum,
//     L: Indicator<Num = T> + 's,
//     R: Indicator<Num = T> + 's,
//     Op: Fn(T, T) -> T,
// {
//     pub fn new(left: L, right: R, op: Op) -> Self {
//         Self {
//             left,
//             right,
//             op,
//             _phantom: PhantomData,
//         }
//     }
// }
//
// impl<'s, T, L, R, Op> Indicator for BinaryOperationIndicator<'s, T, L, R, Op>
// where
//     T: TrNum,
//     L: Indicator<Num = T> + 's,
//     R: Indicator<Num = T> + 's,
//     Op: Fn(T, T) -> T,
// {
//     type Num = T;
//     type Series<'x> = L::Series<'x> where Self: 'x;
//
//     fn get_value(&self, index: usize) -> Result<T, IndicatorError> {
//         let n1 = self.left.get_value(index)?;
//         let n2 = self.right.get_value(index)?;
//         Ok((self.op)(n1, n2))
//     }
//
//     fn get_bar_series(&self) -> &Self::Series<'_> {
//         self.left.get_bar_series()
//     }
//
//     fn get_count_of_unstable_bars(&self) -> usize {
//         0
//     }
// }
//


