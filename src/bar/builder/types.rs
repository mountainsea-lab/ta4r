use crate::bar::base_bar_series::BaseBarSeries;
use crate::bar::builder::factory::tick_bar_builder_factory::TickBarBuilderFactory;
use crate::bar::builder::factory::time_bar_builder_factory::TimeBarBuilderFactory;
use crate::bar::builder::factory::volume_bar_builder_factory::VolumeBarBuilderFactory;
use crate::bar::builder::time_bar_builder::TimeBarBuilder;
use crate::bar::types::{BarBuilderFactory, BarSeries};
use crate::num::TrNum;
use std::fmt;
use std::marker::PhantomData;

// 枚举包装不同的 BarBuilderFactory 实现
#[derive(Clone)]
pub enum BarBuilderFactories<T: TrNum> {
    TimeBarFactory(TimeBarBuilderFactory),
    TickBarFactory(TickBarBuilderFactory),
    VolumeBarFactory(VolumeBarBuilderFactory),
    // 以后可能会有其他带T的变体
    _Phantom(PhantomData<T>),
}

impl<T: TrNum + 'static> BarBuilderFactory<T> for BarBuilderFactories<T> {
    // 这里使用枚举自身作为 Series 的 F 类型参数
    type Series = BaseBarSeries<T>;
    // Builder 先写成 TimeBarBuilder，后续扩展需重新设计
    type Builder<'a>
        = TimeBarBuilder<'a, T, Self::Series>
    where
        Self::Series: 'a;

    fn create_bar_builder<'a>(&self, series: &'a mut Self::Series) -> Self::Builder<'a> {
        match self {
            BarBuilderFactories::TimeBarFactory(_) => {
                let factory = series.num_factory();
                TimeBarBuilder::new_with_factory(factory).bind_to(series)
            }
            //         // BarBuilderFactories::Other(factory) => factory.create_bar_builder(series),
            _ => unreachable!("Unsupported BarBuilderFactories variant"),
        }
    }
}

impl<T: TrNum> fmt::Debug for BarBuilderFactories<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BarBuilderFactories::TimeBarFactory(factory) => {
                f.debug_tuple("TimeBarFactory").field(factory).finish()
            }
            BarBuilderFactories::TickBarFactory(factory) => {
                f.debug_tuple("TimeBarFactory").field(factory).finish()
            }
            BarBuilderFactories::VolumeBarFactory(factory) => {
                f.debug_tuple("TimeBarFactory").field(factory).finish()
            }
            // 如果添加了其他变体，继续写匹配
            BarBuilderFactories::_Phantom(_) => {
                f.debug_tuple("_Phantom").field(&"PhantomData").finish()
            }
        }
    }
}
