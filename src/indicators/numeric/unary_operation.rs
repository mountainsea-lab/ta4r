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


// BinaryOperation 结构体
#[derive(Clone)]
pub struct BinaryOperation<T, L, R>
where
    T: TrNum,
    L: Indicator<Num = T>,
    R: Indicator<Num = T>,
{
    left: L,
    right: R,
    operator: BinaryOp<T>,
    series: L::Series<'static>, // 这里稍复杂，后面说明
    _marker: PhantomData<T>,
}

// 关于 series 字段
// 由于 GAT 绑定生命周期，通常 `series` 应该是引用，且生命周期和 BinaryOperation 绑定
// 这里示范更简单的实现，series 改成引用，结构体带生命周期参数

#[derive(Clone)]
pub struct BinaryOperationRef<'a, T, L, R>
where
    T: TrNum,
    L: Indicator<Num = T> + 'a,
    R: Indicator<Num = T> + 'a,
{
    left: L,
    right: R,
    operator: BinaryOp<T>,
    series: &'a L::Series<'a>, // 绑定生命周期
    _marker: PhantomData<T>,
}

impl<'a, T, L, R> BinaryOperationRef<'a, T, L, R>
where
    T: TrNum,
    L: Indicator<Num = T> + 'a,
    R: Indicator<Num = T> + 'a,
{
    pub fn new_simple(left: L, right: R, op: fn(&T, &T) -> T) -> Self {
        let series = left.get_bar_series();
        Self {
            left,
            right,
            operator: BinaryOp::Simple(op),
            series,
            _marker: PhantomData,
        }
    }

    pub fn new_fallible(left: L, right: R, op: fn(&T, &T) -> Result<T, IndicatorError>) -> Self {
        let series = left.get_bar_series();
        Self {
            left,
            right,
            operator: BinaryOp::Fallible(op),
            series,
            _marker: PhantomData,
        }
    }

    pub fn get_value(&self, index: usize) -> Result<T, IndicatorError> {
        let left_val = self.left.get_value(index)?;
        let right_val = self.right.get_value(index)?;
        match self.operator {
            BinaryOp::Simple(op) => Ok(op(&left_val, &right_val)),
            BinaryOp::Fallible(op) => op(&left_val, &right_val),
        }
    }

    pub fn get_bar_series(&self) -> &L::Series<'a> {
        self.series
    }

    pub fn get_count_of_unstable_bars(&self) -> usize {
        // 取两个指标的最大不稳定条数
        usize::max(self.left.get_count_of_unstable_bars(), self.right.get_count_of_unstable_bars())
    }
}

// 实现 Indicator trait
impl<'a, T, L, R> Indicator for BinaryOperationRef<'a, T, L, R>
where
    T: TrNum,
    L: Indicator<Num = T> + 'a,
    R: Indicator<Num = T> + 'a,
{
    type Num = T;
    type Series<'s> = L::Series<'s> where Self: 's;

    fn get_value(&self, index: usize) -> Result<Self::Num, IndicatorError> {
        self.get_value(index)
    }

    fn get_bar_series(&self) -> &Self::Series<'_> {
        self.get_bar_series()
    }

    fn get_count_of_unstable_bars(&self) -> usize {
        self.get_count_of_unstable_bars()
    }
}