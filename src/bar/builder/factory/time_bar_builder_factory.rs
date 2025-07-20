use crate::bar::base_bar_series::BaseBarSeries;
use crate::bar::builder::time_bar_builder::TimeBarBuilder;
use crate::bar::types::{BarBuilderFactory, BarSeries};
use crate::num::TrNum;

/// TimeBarBuilderFactory - 创建 TimeBarBuilder 的工厂
#[derive(Debug, Clone, Default)]
pub struct TimeBarBuilderFactory;

impl<T: TrNum> BarBuilderFactory<T> for TimeBarBuilderFactory {
    type Series = BaseBarSeries<T>;
    type Builder = TimeBarBuilder<T, Self::Series>;

    fn create_bar_builder(&self, series: &Self::Series) -> Self::Builder {
        let factory = series.num_factory();
        TimeBarBuilder::new_with_factory(factory).bind_to(series)
    }
}
