use crate::indicators::helpers::constant_indicator::ConstantIndicator;
use crate::indicators::Indicator;
use crate::indicators::numeric::binary_operation::BinaryOperation;
use crate::indicators::numeric::unary_operation::UnaryOperation;
use crate::indicators::types::IndicatorError;

pub struct NumericIndicator<I>
where
    I: Indicator,
{
    delegate: I,
}

impl<I> NumericIndicator<I>
where
    I: Indicator,
{
    pub fn of(indicator: I) -> Self {
        Self { delegate: indicator }
    }

    pub fn plus<J>(self, other: J) -> NumericIndicator<BinaryOperation<I, J, fn(I::Num, I::Num) -> I::Num>>
    where
        J: Indicator<Num = I::Num>,
        I::Num: Copy,
    {
        NumericIndicator::of(BinaryOperation::sum(self.delegate, other))
    }

    pub fn plus_num(self, n: I::Num) -> NumericIndicator<BinaryOperation<I, ConstantIndicator<I::Num, Series>, fn(I::Num, I::Num) -> I::Num>>
    where
        I::Num: Copy,
    {
        let constant = ConstantIndicator::new(self.delegate.get_bar_series(), n);
        self.plus(constant)
    }

    // 其它操作类似实现，如 minus, multiplied_by, divided_by, min, max

    pub fn abs(self) -> NumericIndicator<UnaryOperation<I, fn(I::Num) -> I::Num>>
    where
        I::Num: Copy,
    {
        NumericIndicator::of(UnaryOperation::abs(self.delegate))
    }

    pub fn sqrt(self) -> NumericIndicator<UnaryOperation<I, fn(I::Num) -> I::Num>>
    where
        I::Num: Copy,
    {
        NumericIndicator::of(UnaryOperation::sqrt(self.delegate))
    }

    // ...squared(), sma(), ema() 等指标生成器可类似实现

    pub fn get_value(&self, index: usize) -> Result<I::Num, IndicatorError> {
        self.delegate.get_value(index)
    }

    pub fn get_bar_series(&self) -> &I::Series<'_> {
        self.delegate.get_bar_series()
    }
}
