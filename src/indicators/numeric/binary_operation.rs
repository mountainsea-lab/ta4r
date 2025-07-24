use crate::indicators::Indicator;
use crate::indicators::types::IndicatorError;

pub struct BinaryOperation<I, J, F>
where
    I: Indicator,
    J: Indicator<Num = I::Num>,
    F: Fn(I::Num, I::Num) -> I::Num + Copy,
{
    left: I,
    right: J,
    operator: F,
}

impl<I, J, F> BinaryOperation<I, J, F>
where
    I: Indicator,
    J: Indicator<Num = I::Num>,
    F: Fn(I::Num, I::Num) -> I::Num + Copy,
{
    pub fn new(left: I, right: J, operator: F) -> Self {
        Self { left, right, operator }
    }

    pub fn sum(left: I, right: J) -> Self {
        Self::new(left, right, |a, b| a.plus(&b))
    }

    pub fn difference(left: I, right: J) -> Self {
        Self::new(left, right, |a, b| a.minus(&b))
    }

    pub fn product(left: I, right: J) -> Self {
        Self::new(left, right, |a, b| a.multiplied_by(&b))
    }

    pub fn quotient(left: I, right: J) -> Self {
        Self::new(left, right, |a, b| a.divided_by(&b))
    }

    pub fn min(left: I, right: J) -> Self {
        Self::new(left, right, |a, b| a.min(&b))
    }

    pub fn max(left: I, right: J) -> Self {
        Self::new(left, right, |a, b| a.max(&b))
    }
}

impl<I, J, F> Indicator for BinaryOperation<I, J, F>
where
    I: Indicator,
    J: Indicator<Num = I::Num>,
    F: Fn(I::Num, I::Num) -> I::Num + Copy,
{
    type Num = I::Num;
    type Series<'a> = I::Series<'a> where I: 'a;

    fn get_value(&self, index: usize) -> Result<Self::Num, IndicatorError> {
        let left_val = self.left.get_value(index)?;
        let right_val = self.right.get_value(index)?;
        Ok((self.operator)(left_val, right_val))
    }

    fn get_bar_series(&self) -> &Self::Series<'_> {
        self.left.get_bar_series()
    }

    fn get_count_of_unstable_bars(&self) -> usize {
        0
    }
}
