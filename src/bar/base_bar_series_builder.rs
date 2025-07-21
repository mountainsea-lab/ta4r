use crate::bar::base_bar::BaseBar;
use crate::bar::base_bar_series::BaseBarSeries;
use crate::bar::builder::factory::time_bar_builder_factory::TimeBarBuilderFactory;
use crate::bar::builder::types::BarBuilderFactories;
use crate::bar::types::{BarSeries, BarSeriesBuilder};
use crate::num::decimal_num::DecimalNum;
use crate::num::decimal_num_factory::DecimalNumFactory;
use crate::num::{NumFactory, TrNum};
use std::sync::Arc;

/// BaseBarSeriesBuilder 结构体 - 使用泛型参数避免动态分发
#[derive(Debug, Clone)]
pub struct BaseBarSeriesBuilder<T: TrNum> {
    /// 序列名称
    name: Option<String>,
    /// 预设的 Bar 列表
    bars: Vec<BaseBar<T>>,
    /// 是否受约束
    constrained: bool,
    /// 最大 Bar 数量
    max_bar_count: usize,
    /// 数值工厂
    num_factory: Arc<T::Factory>,
    /// Bar 构建器工厂
    bar_builder_factory: Option<BarBuilderFactories<T>>,
}

/// 默认使用DecimalNum
impl BaseBarSeriesBuilder<DecimalNum> {
    pub fn new() -> Self {
        Self {
            name: Some("unnamed_series".to_string()),
            bars: Vec::new(),
            constrained: false,
            max_bar_count: usize::MAX,
            num_factory: Arc::new(DecimalNumFactory::instance()),
            bar_builder_factory: Some(BarBuilderFactories::TimeBarFactory(
                TimeBarBuilderFactory::default(),
            )),
        }
    }
}

/// 实现一个默认构造,支持范型
impl<T: TrNum> Default for BaseBarSeriesBuilder<T> {
    fn default() -> Self {
        Self {
            name: Some("unnamed_series".to_string()),
            bars: Vec::new(),
            constrained: false,
            max_bar_count: usize::MAX,
            num_factory: Arc::new(T::Factory::default()), // 或者其他合适的初始化方式
            bar_builder_factory: None,
        }
    }
}

impl<T: TrNum> BaseBarSeriesBuilder<T> {
    /// 设置序列名称
    pub fn with_name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    /// 设置预设的 Bar 列表
    pub fn with_bars(mut self, bars: Vec<BaseBar<T>>) -> Self {
        self.bars = bars;
        self
    }

    /// 设置是否受约束
    pub fn with_constrained(mut self, constrained: bool) -> Self {
        self.constrained = constrained;
        self
    }

    /// 设置最大 Bar 数量
    pub fn with_max_bar_count(mut self, max_bar_count: usize) -> Self {
        self.max_bar_count = max_bar_count;
        self
    }

    /// 设置数值工厂
    pub fn with_num_factory(mut self, num_factory: Arc<T::Factory>) -> Self {
        self.num_factory = num_factory;
        self
    }

    /// 设置 Bar 构建器工厂
    pub fn with_bar_builder_factory(mut self, bar_builder_factory: BarBuilderFactories<T>) -> Self {
        self.bar_builder_factory = Some(bar_builder_factory);
        self
    }
}

impl<T> BarSeriesBuilder<T> for BaseBarSeriesBuilder<T>
where
    T: TrNum + 'static,
    T::Factory: NumFactory<T>,
{
    type BarSeries = BaseBarSeries<T>;

    fn build(self) -> Result<Self::BarSeries, String> {
        // 确定序列名称
        let name = self.name.unwrap_or_else(|| "unnamed_series".to_string());

        // 确定数值工厂
        let num_factory = Arc::clone(&self.num_factory);

        // 确定 Bar 构建器工厂
        let bar_builder_factory = self.bar_builder_factory.unwrap_or_else(|| {
            BarBuilderFactories::TimeBarFactory(TimeBarBuilderFactory::default())
        });

        // 计算索引
        let (begin_index, end_index) = if self.bars.is_empty() {
            (-1, -1)
        } else {
            (0, (self.bars.len() - 1) as i32)
        };

        // 创建 BaseBarSeries
        let mut series = BaseBarSeries::new(
            name,
            self.bars,
            begin_index,
            end_index,
            self.constrained,
            num_factory,
            bar_builder_factory,
        )?;

        // 设置最大 Bar 数量
        if self.max_bar_count != usize::MAX {
            series.set_maximum_bar_count(self.max_bar_count);
        }

        Ok(series)
    }
}
