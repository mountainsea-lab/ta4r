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
use crate::bar::builder::factory::tick_bar_builder_factory::TickBarBuilderFactory;
use crate::bar::builder::factory::time_bar_builder_factory::TimeBarBuilderFactory;
use crate::bar::builder::factory::volume_bar_builder_factory::VolumeBarBuilderFactory;
use crate::bar::builder::tick_bar_builder::TickBarBuilder;
use crate::bar::builder::time_bar_builder::TimeBarBuilder;
use crate::bar::builder::volume_bar_builder::VolumeBarBuilder;
use crate::bar::types::{BarBuilder, BarBuilderFactory, BarSeries};
use crate::num::TrNum;
use std::fmt;
use std::marker::PhantomData;
use time::{Duration, OffsetDateTime};

// 枚举包装不同的 BarBuilderFactory 实现
#[derive(Clone)]
pub enum BarBuilderFactories<T: TrNum> {
    TimeBarFactory(TimeBarBuilderFactory),
    TickBarFactory(TickBarBuilderFactory<T>),
    VolumeBarFactory(VolumeBarBuilderFactory),
    // 以后可能会有其他带T的变体
    _Phantom(PhantomData<T>),
}
impl<T: TrNum + 'static> BarBuilderFactory<T> for BarBuilderFactories<T> {
    // 这里使用枚举自身作为 Series 的 F 类型参数
    type Series = BaseBarSeries<T>;
    type Builder<'a>
        = BarBuilders<'a, T>
    where
        Self::Series: 'a;

    fn create_bar_builder<'a>(&self, series: &'a mut Self::Series) -> Self::Builder<'a> {
        match self {
            BarBuilderFactories::TimeBarFactory(_) => {
                let factory = series.num_factory();
                BarBuilders::Time(TimeBarBuilder::new_with_factory(factory).bind_to(series))
            }
            BarBuilderFactories::TickBarFactory(factory) => {
                BarBuilders::Tick(factory.create_bar_builder(series))
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
                f.debug_tuple("TickBarFactory").field(factory).finish()
            }
            BarBuilderFactories::VolumeBarFactory(factory) => {
                f.debug_tuple("VolumeBarFactory").field(factory).finish()
            }
            // 如果添加了其他变体，继续写匹配
            BarBuilderFactories::_Phantom(_) => {
                f.debug_tuple("_Phantom").field(&"PhantomData").finish()
            }
        }
    }
}

#[derive(Debug)]
pub enum BarBuilders<'a, T: TrNum + 'static> {
    Time(TimeBarBuilder<'a, T, BaseBarSeries<T>>),
    Tick(TickBarBuilder<'a, T, BaseBarSeries<T>>),
    Volume(VolumeBarBuilder<'a, T, BaseBarSeries<T>>),
}

impl<'a, T: TrNum + 'static> BarBuilder<T> for BarBuilders<'a, T> {
    type Bar = BaseBar<T>;

    fn time_period(self, period: Duration) -> Self {
        match self {
            Self::Time(b) => Self::Time(b.time_period(period)),
            Self::Tick(b) => Self::Tick(b.time_period(period)),
            Self::Volume(b) => Self::Volume(b.time_period(period)),
        }
    }

    fn begin_time(self, time: OffsetDateTime) -> Self {
        match self {
            Self::Time(b) => Self::Time(b.begin_time(time)),
            Self::Tick(b) => Self::Tick(b.begin_time(time)),
            Self::Volume(b) => Self::Volume(b.begin_time(time)),
        }
    }

    fn end_time(self, time: OffsetDateTime) -> Self {
        match self {
            Self::Time(b) => Self::Time(b.end_time(time)),
            Self::Tick(b) => Self::Tick(b.end_time(time)),
            Self::Volume(b) => Self::Volume(b.end_time(time)),
        }
    }

    fn open_price(self, price: T) -> Self {
        match self {
            Self::Time(b) => Self::Time(b.open_price(price)),
            Self::Tick(b) => Self::Tick(b.open_price(price)),
            Self::Volume(b) => Self::Volume(b.open_price(price)),
        }
    }

    fn high_price(self, price: T) -> Self {
        match self {
            Self::Time(b) => Self::Time(b.high_price(price)),
            Self::Tick(b) => Self::Tick(b.high_price(price)),
            Self::Volume(b) => Self::Volume(b.high_price(price)),
        }
    }

    fn low_price(self, price: T) -> Self {
        match self {
            Self::Time(b) => Self::Time(b.low_price(price)),
            Self::Tick(b) => Self::Tick(b.low_price(price)),
            Self::Volume(b) => Self::Volume(b.low_price(price)),
        }
    }

    fn close_price(self, price: T) -> Self {
        match self {
            Self::Time(b) => Self::Time(b.close_price(price)),
            Self::Tick(b) => Self::Tick(b.close_price(price)),
            Self::Volume(b) => Self::Volume(b.close_price(price)),
        }
    }

    fn volume(self, volume: T) -> Self {
        match self {
            Self::Time(b) => Self::Time(b.volume(volume)),
            Self::Tick(b) => Self::Tick(b.volume(volume)),
            Self::Volume(b) => Self::Volume(b.volume(volume)),
        }
    }

    fn amount(self, amount: T) -> Self {
        match self {
            Self::Time(b) => Self::Time(b.amount(amount)),
            Self::Tick(b) => Self::Tick(b.amount(amount)),
            Self::Volume(b) => Self::Volume(b.amount(amount)),
        }
    }

    fn trades(self, trades: u64) -> Self {
        match self {
            Self::Time(b) => Self::Time(b.trades(trades)),
            Self::Tick(b) => Self::Tick(b.trades(trades)),
            Self::Volume(b) => Self::Volume(b.trades(trades)),
        }
    }

    fn build(&self) -> Result<Self::Bar, String> {
        match self {
            Self::Time(b) => b.build(),
            Self::Tick(b) => b.build(),
            Self::Volume(b) => b.build(),
        }
    }

    fn add(&mut self) -> Result<(), String> {
        match self {
            Self::Time(b) => b.add(),
            Self::Tick(b) => b.add(),
            Self::Volume(b) => b.add(),
        }
    }
}
