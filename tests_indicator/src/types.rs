use crate::IndicatorFactory;
use ta4r::bar::types::BarSeries;
use ta4r::indicators::Indicator;
use ta4r::indicators::types::IndicatorError;
use ta4r::num::TrNum;

#[derive(Clone, Copy)]
pub enum NumKind {
    Double,
    Decimal,
}

impl NumKind {
    pub fn name(&self) -> &'static str {
        match self {
            NumKind::Double => "DoubleNum",
            NumKind::Decimal => "DecimalNum",
        }
    }
}

pub struct TestContext<'a, T, S, I, F>
where
    T: TrNum + 'static,
    S: BarSeries<'a, T>,
    I: Indicator<Num = T> + Clone + 'static,
    F: IndicatorFactory<T, S, I>,
{
    pub kind: NumKind,
    pub factory: F,
    pub phantom: std::marker::PhantomData<(&'a T, S, I)>,
}

impl<'a, T, S, I, F> TestContext<'a, T, S, I, F>
where
    T: TrNum + 'static,
    S: BarSeries<'a, T>,
    I: Indicator<Num = T> + Clone + 'static,
    F: IndicatorFactory<T, S, I>,
{
    pub fn build_indicator(&self, series: &S, params: &[usize]) -> Result<I, IndicatorError> {
        self.factory.build(series, params)
    }
}
