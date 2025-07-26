use crate::indicators::Indicator;
use crate::indicators::types::{BinaryOp, IndicatorError};
use crate::num::TrNum;
use std::marker::PhantomData;
// pub struct BinaryOperation<I, J, F>
// where
//     I: Indicator,
//     J: Indicator<Num = I::Num>,
//     F: Fn(I::Num, I::Num) -> I::Num + Copy,
// {
//     left: I,
//     right: J,
//     operator: F,
// }
//
// impl<I, J, F> BinaryOperation<I, J, F>
// where
//     I: Indicator,
//     J: Indicator<Num = I::Num>,
//     F: Fn(I::Num, I::Num) -> I::Num + Copy,
// {
//     pub fn new(left: I, right: J, operator: F) -> Self {
//         Self { left, right, operator }
//     }
//
//     pub fn sum(left: I, right: J) -> Self {
//         Self::new(left, right, |a, b| a.plus(&b))
//     }
//
//     pub fn difference(left: I, right: J) -> Self {
//         Self::new(left, right, |a, b| a.minus(&b))
//     }
//
//     pub fn product(left: I, right: J) -> Self {
//         Self::new(left, right, |a, b| a.multiplied_by(&b))
//     }
//
//     pub fn quotient(left: I, right: J) -> Self {
//         Self::new(left, right, |a, b| a.divided_by(&b))
//     }
//
//     pub fn min(left: I, right: J) -> Self {
//         Self::new(left, right, |a, b| a.min(&b))
//     }
//
//     pub fn max(left: I, right: J) -> Self {
//         Self::new(left, right, |a, b| a.max(&b))
//     }
// }
//
// impl<I, J, F> Indicator for BinaryOperation<I, J, F>
// where
//     I: Indicator,
//     J: Indicator<Num = I::Num>,
//     F: Fn(I::Num, I::Num) -> I::Num + Copy,
// {
//     type Num = I::Num;
//     type Series<'a> = I::Series<'a> where I: 'a;
//
//     fn get_value(&self, index: usize) -> Result<Self::Num, IndicatorError> {
//         let left_val = self.left.get_value(index)?;
//         let right_val = self.right.get_value(index)?;
//         Ok((self.operator)(left_val, right_val))
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

// 关于 series 字段
// 由于 GAT 绑定生命周期，通常 `series` 应该是引用，且生命周期和 BinaryOperation 绑定
// 这里示范更简单的实现，series 改成引用，结构体带生命周期参数
pub struct BinaryOperation<T, L, R>
where
    T: TrNum,
    L: Indicator<Num = T>,
    R: Indicator<Num = T>,
{
    left: L,
    right: R,
    operator: BinaryOp<T>,
    _marker: PhantomData<T>,
}

impl<T, L, R> BinaryOperation<T, L, R>
where
    T: TrNum + 'static,
    L: Indicator<Num = T>,
    R: Indicator<Num = T>,
{
    pub fn new_simple(left: L, right: R, op: fn(&T, &T) -> T) -> Self {
        Self {
            left,
            right,
            operator: BinaryOp::Simple(op),
            _marker: PhantomData,
        }
    }

    pub fn new_fallible(left: L, right: R, op: fn(&T, &T) -> Result<T, IndicatorError>) -> Self {
        Self {
            left,
            right,
            operator: BinaryOp::Fallible(op),
            _marker: PhantomData,
        }
    }

    pub fn sum(left: L, right: R) -> Self {
        fn plus<T: TrNum>(a: &T, b: &T) -> T {
            a.plus(b)
        }
        Self::new_simple(left, right, plus)
    }

    pub fn difference(left: L, right: R) -> Self {
        fn minus<T: TrNum>(a: &T, b: &T) -> T {
            a.minus(b)
        }
        Self::new_simple(left, right, minus)
    }

    pub fn product(left: L, right: R) -> Self {
        fn multiply<T: TrNum>(a: &T, b: &T) -> T {
            a.multiplied_by(b)
        }
        Self::new_simple(left, right, multiply)
    }

    pub fn quotient(left: L, right: R) -> Self {
        fn divide<T: TrNum>(a: &T, b: &T) -> Result<T, IndicatorError> {
            a.divided_by(b).map_err(IndicatorError::NumError)
        }
        Self::new_fallible(left, right, divide)
    }

    pub fn min(left: L, right: R) -> Self {
        fn min_fn<T: TrNum>(a: &T, b: &T) -> T {
            a.min(b)
        }
        Self::new_simple(left, right, min_fn)
    }

    pub fn max(left: L, right: R) -> Self {
        fn max_fn<T: TrNum>(a: &T, b: &T) -> T {
            a.max(b)
        }
        Self::new_simple(left, right, max_fn)
    }

    pub fn get_value(&self, index: usize) -> Result<T, IndicatorError> {
        let left_val = self.left.get_value(index)?;
        let right_val = self.right.get_value(index)?;
        match self.operator {
            BinaryOp::Simple(op) => Ok(op(&left_val, &right_val)),
            BinaryOp::Fallible(op) => op(&left_val, &right_val),
        }
    }

    pub fn get_count_of_unstable_bars(&self) -> usize {
        usize::max(
            self.left.get_count_of_unstable_bars(),
            self.right.get_count_of_unstable_bars(),
        )
    }
}

impl<T, L, R> Indicator for BinaryOperation<T, L, R>
where
    T: TrNum + 'static,
    L: Indicator<Num = T>,
    R: Indicator<Num = T>,
{
    type Num = T;

    type Series<'s>
        = L::Series<'s>
    where
        Self: 's;

    fn get_value(&self, index: usize) -> Result<Self::Num, IndicatorError> {
        self.get_value(index)
    }

    fn get_bar_series(&self) -> &Self::Series<'_> {
        self.left.get_bar_series()
    }

    fn get_count_of_unstable_bars(&self) -> usize {
        self.left
            .get_count_of_unstable_bars()
            .max(self.right.get_count_of_unstable_bars())
    }
}
