use crate::bar::base_bar::BaseBar;
use crate::bar::base_bar_series_builder::BaseBarSeriesBuilder;
use crate::bar::builder::time_bar_builder::TimeBarBuilder;
use crate::bar::builder::types::BarBuilderFactories;
use crate::bar::types::{Bar, BarBuilderFactory, BarSeries, BarSeriesBuilder};
use crate::num::{NumFactory, TrNum};
use std::fmt;
use std::fmt::Debug;
use std::sync::Arc;

/// BaseBarSeries 结构体 - 使用泛型参数避免动态分发
#[derive(Debug)]
pub struct BaseBarSeriesCore<T: TrNum> {
    /// 序列的名称
    name: String,
    /// Bar 列表
    bars: Vec<BaseBar<T>>,
    /// 数值工厂
    num_factory: Arc<T::Factory>, // 用智能指针包裹
    /// 序列开始索引
    series_begin_index: i32,
    /// 序列结束索引
    series_end_index: i32,
    /// 最大 Bar 数量
    maximum_bar_count: usize,
    /// 已移除的 Bar 数量
    removed_bars_count: usize,
    /// 是否受约束（索引不能改变）
    constrained: bool,
}

#[derive(Debug)]
pub struct BaseBarSeries<T: TrNum> {
    core: BaseBarSeriesCore<T>,
    bar_builder_factory: BarBuilderFactories<T>, // 👈 使用枚举，避免递归泛型
}

impl<T: TrNum> BaseBarSeriesCore<T> {
    /// 构造函数
    pub fn new(
        name: String,
        bars: Vec<BaseBar<T>>,
        series_begin_index: i32,
        series_end_index: i32,
        constrained: bool,
        num_factory: Arc<T::Factory>,
    ) -> Result<Self, String> {
        // 验证索引
        if !bars.is_empty() {
            if series_end_index < series_begin_index - 1 {
                return Err("End index must be >= to begin index - 1".to_string());
            }
            if series_end_index >= bars.len() as i32 {
                return Err("End index must be < to the bar list size".to_string());
            }
        }

        let (begin_index, end_index, is_constrained) = if bars.is_empty() {
            (-1, -1, false)
        } else {
            (series_begin_index, series_end_index, constrained)
        };

        Ok(BaseBarSeriesCore {
            name,
            bars,
            num_factory,
            series_begin_index: begin_index,
            series_end_index: end_index,
            maximum_bar_count: usize::MAX,
            removed_bars_count: 0,
            constrained: is_constrained,
        })
    }

    /// 移除超出最大数量的 Bar
    fn remove_exceeding_bars(&mut self) {
        if self.constrained || self.bars.len() <= self.maximum_bar_count {
            return;
        }

        let bars_to_remove = self.bars.len() - self.maximum_bar_count;
        self.bars.drain(0..bars_to_remove);
        self.removed_bars_count += bars_to_remove;

        if !self.constrained {
            self.series_begin_index += bars_to_remove as i32;
        }
    }

    /// 切割 Bar 列表为子集
    fn cut_bars(bars: &[BaseBar<T>], start_index: usize, end_index: usize) -> Vec<BaseBar<T>> {
        bars[start_index..end_index].to_vec()
    }
}

impl<T: TrNum> BaseBarSeries<T> {
    pub fn new(
        name: String,
        bars: Vec<BaseBar<T>>,
        begin_index: i32,
        end_index: i32,
        constrained: bool,
        num_factory: Arc<T::Factory>,
        bar_builder_factory: BarBuilderFactories<T>,
    ) -> Result<Self, String>
    where
        T::Factory: NumFactory<T>,
    {
        let core =
            BaseBarSeriesCore::new(name, bars, begin_index, end_index, constrained, num_factory)?;

        Ok(Self {
            core,
            bar_builder_factory,
        })
    }
}

impl<T: TrNum> BarSeries<T> for BaseBarSeries<T>
where
    T::Factory: NumFactory<T>,
{
    type Bar = BaseBar<T>;
    type Builder = TimeBarBuilder<T, Self>; // 暂时只支持 TimeBarBuilder
    type NumFactory = T::Factory;
    type SubSeries = BaseBarSeries<T>; // 子序列同样使用枚举封装后的类型
    fn num_factory(&self) -> &Self::NumFactory {
        &self.core.num_factory
    }

    fn bar_builder(&self) -> Self::Builder {
        self.bar_builder_factory.create_bar_builder(self)
    }

    fn get_name(&self) -> &str {
        &self.core.name
    }

    fn get_bar(&self, index: usize) -> Option<&Self::Bar> {
        let inner_index = if index >= self.core.removed_bars_count {
            index - self.core.removed_bars_count
        } else {
            if self.core.bars.is_empty() {
                return None;
            }
            0 // 返回第一个可用的 Bar，对应 ta4j 的行为
        };

        self.core.bars.get(inner_index)
    }

    fn get_bar_count(&self) -> usize {
        if self.core.series_end_index < 0 {
            return 0;
        }
        let start_index = std::cmp::max(
            self.core.removed_bars_count as i32,
            self.core.series_begin_index,
        );
        (self.core.series_end_index - start_index + 1) as usize
    }

    fn is_empty(&self) -> bool {
        self.get_bar_count() == 0
    }

    fn get_bar_data(&self) -> &[Self::Bar] {
        &self.core.bars
    }

    fn get_begin_index(&self) -> i32 {
        self.core.series_begin_index
    }

    fn get_end_index(&self) -> i32 {
        self.core.series_end_index
    }

    fn get_maximum_bar_count(&self) -> usize {
        self.core.maximum_bar_count
    }

    fn set_maximum_bar_count(&mut self, maximum_bar_count: usize) {
        if self.core.constrained {
            panic!("Cannot set a maximum bar count on a constrained bar series");
        }
        if maximum_bar_count == 0 {
            panic!("Maximum bar count must be strictly positive");
        }
        self.core.maximum_bar_count = maximum_bar_count;
        self.core.remove_exceeding_bars();
    }

    fn get_removed_bars_count(&self) -> usize {
        self.core.removed_bars_count
    }

    fn add_bar_with_replace(&mut self, bar: Self::Bar, replace: bool) {
        if self.core.constrained {
            panic!("Cannot add a bar to a constrained bar series");
        }

        if replace && !self.core.bars.is_empty() {
            self.core.bars.pop();
        }

        self.core.bars.push(bar);

        if self.core.series_begin_index == -1 {
            self.core.series_begin_index = 0;
            self.core.series_end_index = 0;
        } else if !self.core.constrained {
            self.core.series_end_index = (self.core.bars.len() as i32) - 1;
        }

        self.core.remove_exceeding_bars();
    }

    fn add_trade(&mut self, trade_volume: T, trade_price: T) {
        if let Some(last_bar) = self.core.bars.last_mut() {
            last_bar.add_trade(trade_volume, trade_price);
        }
    }

    fn add_price(&mut self, price: T) {
        if let Some(last_bar) = self.core.bars.last_mut() {
            last_bar.add_price(price);
        }
    }

    fn get_sub_series(
        &self,
        start_index: usize,
        end_index: usize,
    ) -> Result<Self::SubSeries, String> {
        if start_index >= end_index {
            return Err(format!(
                "the endIndex: {} must be greater than startIndex: {}",
                end_index, start_index
            ));
        }

        if !self.core.bars.is_empty() {
            let start = start_index.saturating_sub(self.core.removed_bars_count);
            let end = std::cmp::min(
                end_index.saturating_sub(self.core.removed_bars_count),
                self.get_end_index() as usize + 1,
            );

            let sub_bars = BaseBarSeriesCore::cut_bars(&self.core.bars, start, end);

            BaseBarSeriesBuilder::<T>::default() // 保证类型一致
                .with_name(self.core.name.clone())
                .with_bars(sub_bars)
                .with_num_factory(Arc::clone(&self.core.num_factory))
                .with_bar_builder_factory(self.bar_builder_factory.clone())
                .build()
        } else {
            BaseBarSeriesBuilder::<T>::default() // 保证类型一致
                .with_name(self.core.name.clone())
                .with_num_factory(Arc::clone(&self.core.num_factory))
                .with_bar_builder_factory(self.bar_builder_factory.clone())
                .build()
        }
    }
}

// 实现 Display trait
impl<T: TrNum> fmt::Display for BaseBarSeries<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "BaseBarSeries{{name: {}, bars: {}, begin: {}, end: {}}}",
            self.core.name,
            self.core.bars.len(),
            self.core.series_begin_index,
            self.core.series_end_index
        )
    }
}

impl<T: TrNum> Clone for BaseBarSeriesCore<T> {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            bars: self.bars.clone(),
            num_factory: Arc::clone(&self.num_factory), // 只 clone Arc，没要求 Factory: Clone
            series_begin_index: self.series_begin_index,
            series_end_index: self.series_end_index,
            maximum_bar_count: self.maximum_bar_count,
            removed_bars_count: self.removed_bars_count,
            constrained: self.constrained,
        }
    }
}

impl<T: TrNum> Clone for BaseBarSeries<T> {
    fn clone(&self) -> Self {
        Self {
            core: self.core.clone(),
            bar_builder_factory: self.bar_builder_factory.clone(),
        }
    }
}
