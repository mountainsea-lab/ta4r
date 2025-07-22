use crate::bar::base_bar_series::BaseBarSeries;
use crate::bar::builder::volume_bar_builder::VolumeBarBuilder;
use crate::bar::types::{BarBuilderFactory, BarSeries};
use crate::num::TrNum;

/// VolumeBarBuilderFactory - 创建 VolumeBarBuilder 的工厂（单例复用）
#[derive(Debug, Clone)]
pub struct VolumeBarBuilderFactory {
    volume_threshold: i64,
}

impl VolumeBarBuilderFactory {
    pub fn new(volume_threshold: i64) -> Self {
        Self { volume_threshold }
    }
}

impl<T: TrNum + 'static> BarBuilderFactory<T> for VolumeBarBuilderFactory {
    type Series = BaseBarSeries<T>;
    type Builder<'a>
        = VolumeBarBuilder<'a, T, BaseBarSeries<T>>
    where
        Self::Series: 'a;

    fn create_bar_builder<'a>(&self, series: &'a mut Self::Series) -> Self::Builder<'a> {
        let factory = series.num_factory();
        VolumeBarBuilder::new_with_factory(factory, self.volume_threshold.clone()).bind_to(series)
    }
}
