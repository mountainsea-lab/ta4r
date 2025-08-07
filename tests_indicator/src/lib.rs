use ta4r::bar::types::BarSeries;
use ta4r::indicators::Indicator;
use ta4r::indicators::types::IndicatorError;
use ta4r::num::TrNum;

pub mod types;

pub trait IndicatorFactory<'a, T, S, I>
where
    T: TrNum + 'static,
    S: BarSeries<'a, T>,
    I: Indicator<Num = T> + Clone + 'static,
{
    fn build(&self, series: &'a S, params: &[usize]) -> Result<I, IndicatorError>;
}
