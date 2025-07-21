use crate::bar::base_bar_series::BaseBarSeries;
use crate::bar::builder::time_bar_builder::TimeBarBuilder;
use crate::bar::types::{BarBuilderFactory, BarSeries};
use crate::num::TrNum;

/// TimeBarBuilderFactory - 创建 TimeBarBuilder 的工厂
#[derive(Debug, Clone, Default)]
pub struct TimeBarBuilderFactory;

impl<T: TrNum + 'static> BarBuilderFactory<T> for TimeBarBuilderFactory {
    type Series = BaseBarSeries<T>;
    // GAT 的合法实现写法（注意这里声明了一个 GAT）
    type Builder<'a>
        = TimeBarBuilder<'a, T, Self::Series>
    where
        Self::Series: 'a;

    fn create_bar_builder<'a>(&self, series: &'a mut Self::Series) -> Self::Builder<'a> {
        let factory = series.num_factory();
        TimeBarBuilder::new_with_factory(factory).bind_to(series)
    }
}
