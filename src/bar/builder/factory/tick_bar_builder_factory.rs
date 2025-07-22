use crate::bar::base_bar_series::BaseBarSeries;
use crate::bar::builder::tick_bar_builder::TickBarBuilder;
use crate::bar::types::{BarBuilderFactory, BarSeries};
use crate::num::TrNum;

/// TickBarBuilderFactory - 创建 TickBarBuilder 的工厂
/// TickBarBuilderFactory - 创建 TickBarBuilder 的工厂
#[derive(Debug, Clone, Default)]
pub struct TickBarBuilderFactory {
    tick_count: u64,
}

impl TickBarBuilderFactory {
    pub fn new(tick_count: u64) -> Self {
        Self { tick_count }
    }
}

impl<T: TrNum + 'static> BarBuilderFactory<T> for TickBarBuilderFactory {
    type Series = BaseBarSeries<T>;

    type Builder<'a>
        = TickBarBuilder<'a, T, Self::Series>
    where
        Self::Series: 'a;

    fn create_bar_builder<'a>(&self, series: &'a mut Self::Series) -> Self::Builder<'a> {
        let factory = series.num_factory();
        TickBarBuilder::new_with_factory(factory, self.tick_count).bind_to(series)
    }
}
