/*!
 * MIT License
 *
 * Copyright (c) 2025 Mountainsea
 * Based on ta4j (c) 2017–2025 Ta4j Organization & respective authors (see AUTHORS)
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */
use crate::bar::base_bar::BaseBar;
use crate::bar::base_bar_series_builder::BaseBarSeriesBuilder;
use crate::bar::builder::types::{BarBuilderFactories, BarBuilders};
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
    series_begin_index: Option<usize>,
    /// 序列结束索引
    series_end_index: Option<usize>,
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
        series_begin_index: Option<usize>,
        series_end_index: Option<usize>,
        constrained: bool,
        num_factory: Arc<T::Factory>,
    ) -> Result<Self, String> {
        let (begin_index, end_index, is_constrained) = if bars.is_empty() {
            (None, None, false)
        } else {
            let bar_len = bars.len();

            // 验证索引
            let begin = series_begin_index.unwrap_or(0);
            let end = series_end_index.unwrap_or(bar_len - 1);

            if end + 1 < begin {
                return Err(format!(
                    "End index must be >= begin index - 1 (begin={}, end={})",
                    begin, end
                ));
            }

            if end >= bar_len {
                return Err(format!(
                    "End index must be < bar list size (end={}, len={})",
                    end, bar_len
                ));
            }

            (Some(begin), Some(end), constrained)
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

        self.series_begin_index = self
            .series_begin_index
            .map(|idx| idx.saturating_add(bars_to_remove));
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
        begin_index: Option<usize>,
        end_index: Option<usize>,
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

impl<'a, T: TrNum + 'static> BarSeries<'a, T> for BaseBarSeries<T>
where
    T::Factory: NumFactory<T>,
{
    type Bar = BaseBar<T>;

    // 关联类型 Builder 改成带生命周期参数的 GAT
    type Builder<'b>
        = BarBuilders<'b, T>
    where
        Self: 'b;

    type NumFactory = T::Factory;

    type SubSeries = BaseBarSeries<T>;

    fn num_factory(&self) -> Arc<Self::NumFactory> {
        self.core.num_factory.clone()
    }

    fn factory_ref(&self) -> &T::Factory {
        self.core.num_factory.as_ref()
    }

    fn bar_builder(&mut self) -> Self::Builder<'_> {
        let factory = self.bar_builder_factory.clone(); // 避免双借用
        factory.create_bar_builder(self)
    }

    fn get_name(&self) -> &str {
        &self.core.name
    }

    fn get_bar(&self, index: usize) -> Option<&Self::Bar> {
        if self.core.bars.is_empty() {
            return None;
        }

        if index < self.core.removed_bars_count {
            return None; // 索引已被移除，返回 None 更合逻辑
        }

        let inner_index = index - self.core.removed_bars_count;
        self.core.bars.get(inner_index)
    }

    fn get_bar_count(&self) -> usize {
        // 如果 series_end_index 或 series_begin_index 任何一个是 None，说明无效或者空序列，直接返回 0
        let end_index = match self.core.series_end_index {
            Some(e) => e,
            None => return 0,
        };
        let begin_index = match self.core.series_begin_index {
            Some(b) => b,
            None => return 0,
        };

        let start_index = std::cmp::max(self.core.removed_bars_count, begin_index);
        if end_index < start_index {
            0
        } else {
            end_index - start_index + 1
        }
    }

    fn is_empty(&self) -> bool {
        self.get_bar_count() == 0
    }

    fn get_bar_data(&self) -> &[Self::Bar] {
        &self.core.bars
    }

    fn get_begin_index(&self) -> Option<usize> {
        self.core.series_begin_index
    }

    fn get_end_index(&self) -> Option<usize> {
        self.core.series_end_index
    }

    fn get_maximum_bar_count(&self) -> usize {
        self.core.maximum_bar_count
    }

    fn set_maximum_bar_count(&mut self, maximum_bar_count: usize) -> Result<(), String> {
        if self.core.constrained {
            return Err("Cannot set a maximum bar count on a constrained bar series".into());
        }
        if maximum_bar_count == 0 {
            return Err("Maximum bar count must be strictly positive".into());
        }
        self.core.maximum_bar_count = maximum_bar_count;
        self.core.remove_exceeding_bars();
        Ok(())
    }

    fn get_removed_bars_count(&self) -> usize {
        self.core.removed_bars_count
    }

    fn add_bar_with_replace(&mut self, bar: Self::Bar, replace: bool) -> Result<(), String> {
        if self.core.constrained {
            return Err("Cannot add a bar to a constrained bar series".into());
        }

        if replace && !self.core.bars.is_empty() {
            self.core.bars.pop();
        }

        self.core.bars.push(bar);

        if self.core.series_begin_index.is_none() {
            self.core.series_begin_index = Some(0);
        }

        self.core.series_end_index = Some(match self.core.series_end_index {
            Some(end) => end.saturating_add(1),
            None => 0,
        });

        self.core.remove_exceeding_bars();
        Ok(())
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
                self.get_end_index().map(|i| i + 1).unwrap_or(0),
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
            "BaseBarSeries{{name: {}, bars: {}, begin: {:?}, end: {:?}}}",
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
