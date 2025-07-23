use crate::bar::base_bar_series::BaseBarSeries;
use crate::bar::builder::heikin_ashi_bar_builder::HeikinAshiBarBuilder;
use crate::bar::types::{BarBuilderFactory, BarSeries};
use crate::num::TrNum;

#[derive(Debug, Clone, Default)]
pub struct HeikinAshiBarBuilderFactory;

impl<T: TrNum + 'static> BarBuilderFactory<T> for HeikinAshiBarBuilderFactory {
    type Series = BaseBarSeries<T>;
    type Builder<'a>
        = HeikinAshiBarBuilder<'a, T, Self::Series>
    where
        Self::Series: 'a;

    fn create_bar_builder<'a>(&self, series: &'a mut Self::Series) -> Self::Builder<'a> {
        let factory = series.num_factory();
        HeikinAshiBarBuilder::new_with_factory(factory).bind_to(series)
    }
}
