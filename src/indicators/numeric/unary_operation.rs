use crate::indicators::Indicator;
use crate::indicators::types::IndicatorError;
use crate::num::TrNum;

pub struct UnaryOperation<I, F>
where
    I: Indicator,
    F: Fn(I::Num) -> I::Num + Copy,
{
    operand: I,
    operator: F,
}

impl<I, F> UnaryOperation<I, F>
where
    I: Indicator,
    F: Fn(I::Num) -> I::Num + Copy,
{
    pub fn new(operand: I, operator: F) -> Self {
        Self { operand, operator }
    }

    pub fn sqrt(operand: I) -> Self
    where
        I::Num: TrNum, // 约束 Num 实现 sqrt
    {
        Self::new(operand, |n| n.sqrt())
    }

    pub fn abs(operand: I) -> Self
    where
        I::Num: TrNum, // 约束 Num 实现 abs
    {
        Self::new(operand, |n| n.abs())
    }
}

impl<I, F> Indicator for UnaryOperation<I, F>
where
    I: Indicator,
    F: Fn(I::Num) -> I::Num + Copy,
{
    type Num = I::Num;
    type Series<'a> = I::Series<'a> where I: 'a;

    fn get_value(&self, index: usize) -> Result<Self::Num, IndicatorError> {
        let val = self.operand.get_value(index)?;
        Ok((self.operator)(val))
    }

    fn get_bar_series(&self) -> &Self::Series<'_> {
        self.operand.get_bar_series()
    }

    fn get_count_of_unstable_bars(&self) -> usize {
        0 // 或根据 operand 决定
    }
}
