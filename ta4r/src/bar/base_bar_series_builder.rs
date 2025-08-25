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
use crate::bar::base_bar_series::BaseBarSeries;
use crate::bar::builder::types::BarBuilderFactories;
use crate::bar::types::{BarSeries, BarSeriesBuilder};
use crate::num::decimal_num::DecimalNum;
use crate::num::decimal_num_factory::DecimalNumFactory;
use crate::num::{NumFactory, TrNum};
use std::sync::Arc;

/// BaseBarSeriesBuilder 结构体 - 使用泛型参数避免动态分发
#[derive(Debug, Clone)]
pub struct BaseBarSeriesBuilder<T: TrNum> {
    name: Option<String>,
    pub bars: Vec<BaseBar<T>>,
    constrained: bool,
    max_bar_count: usize,
    num_factory: Arc<T::Factory>,
    pub bar_builder_factory: Option<BarBuilderFactories<T>>,
}

/// 默认 DecimalNum 构造
impl BaseBarSeriesBuilder<DecimalNum> {
    pub fn new() -> Self {
        Self {
            name: Some("unnamed_series".to_string()),
            bars: Vec::new(),
            constrained: false,
            max_bar_count: usize::MAX,
            num_factory: Arc::new(DecimalNumFactory::instance()),
            bar_builder_factory: Some(BarBuilderFactories::TimeBarFactory(Default::default())),
        }
    }
}

impl<T: TrNum> Default for BaseBarSeriesBuilder<T> {
    fn default() -> Self {
        Self {
            name: Some("unnamed_series".to_string()),
            bars: Vec::new(),
            constrained: false,
            max_bar_count: usize::MAX,
            num_factory: Arc::new(T::Factory::default()),
            bar_builder_factory: None,
        }
    }
}

impl<T: TrNum> BaseBarSeriesBuilder<T> {
    // =========================
    // 链式 Self 风格（实盘/生产）
    // =========================
    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn with_bars(mut self, bars: Vec<BaseBar<T>>) -> Self {
        self.bars = bars;
        self
    }

    pub fn with_constrained(mut self, constrained: bool) -> Self {
        self.constrained = constrained;
        self
    }

    pub fn with_max_bar_count(mut self, max_bar_count: usize) -> Self {
        self.max_bar_count = max_bar_count;
        self
    }

    pub fn with_num_factory(mut self, num_factory: Arc<T::Factory>) -> Self {
        self.num_factory = num_factory;
        self
    }

    pub fn with_bar_builder_factory(mut self, bar_builder_factory: BarBuilderFactories<T>) -> Self {
        self.bar_builder_factory = Some(bar_builder_factory);
        self
    }

    // =========================
    // &mut Self 风格（测试/Mock）
    // =========================
    pub fn set_name(&mut self, name: impl Into<String>) -> &mut Self {
        self.name = Some(name.into());
        self
    }

    pub fn set_bars(&mut self, bars: Vec<BaseBar<T>>) -> &mut Self {
        self.bars = bars;
        self
    }

    pub fn set_constrained(&mut self, constrained: bool) -> &mut Self {
        self.constrained = constrained;
        self
    }

    pub fn set_max_bar_count(&mut self, max_bar_count: usize) -> &mut Self {
        self.max_bar_count = max_bar_count;
        self
    }

    pub fn set_num_factory(&mut self, num_factory: Arc<T::Factory>) -> &mut Self {
        self.num_factory = num_factory;
        self
    }

    pub fn set_bar_builder_factory(
        &mut self,
        bar_builder_factory: BarBuilderFactories<T>,
    ) -> &mut Self {
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

    /// build 使用 move 语义（实盘/生产）
    fn build(&self) -> Result<Self::BarSeries, String> {
        let name = self
            .name
            .clone()
            .unwrap_or_else(|| "unnamed_series".to_string());
        let num_factory = Arc::clone(&self.num_factory);
        let bar_builder_factory = self
            .bar_builder_factory
            .clone()
            .unwrap_or_else(|| BarBuilderFactories::TimeBarFactory(Default::default()));

        let (begin_index, end_index) = if self.bars.is_empty() {
            (None, None)
        } else {
            (Some(0), Some(self.bars.len() - 1))
        };

        let mut series = BaseBarSeries::new(
            name,
            self.bars.clone(),
            begin_index,
            end_index,
            self.constrained,
            num_factory,
            bar_builder_factory,
        )?;

        if self.max_bar_count != usize::MAX {
            series.set_maximum_bar_count(self.max_bar_count)?;
        }

        Ok(series)
    }
}
