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
use std::sync::{Arc, Mutex};
use time::{Duration, OffsetDateTime};

/// BarSeries类型封装两种引用方式
#[derive(Debug)]
pub enum BarSeriesRef<'a, S> {
    Mut(&'a mut S),
    Shared(Arc<Mutex<S>>),
}

// 枚举包装不同的 BarBuilderFactory 实现
#[derive(Clone)]
pub enum BarBuilderFactories<T: TrNum> {
    TimeBarFactory(TimeBarBuilderFactory<T>),
    TickBarFactory(TickBarBuilderFactory<T>),
    VolumeBarFactory(VolumeBarBuilderFactory<T>),
    // 以后可能会有其他带T的变体
    _Phantom(PhantomData<T>),
}

impl<T: TrNum> Default for BarBuilderFactories<T> {
    fn default() -> Self {
        BarBuilderFactories::_Phantom(PhantomData)
    }
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
            BarBuilderFactories::TimeBarFactory(factory) => {
                BarBuilders::Time(factory.create_bar_builder(series))
            }
            BarBuilderFactories::TickBarFactory(factory) => {
                BarBuilders::Tick(factory.create_bar_builder(series))
            }
            BarBuilderFactories::VolumeBarFactory(factory) => {
                BarBuilders::Volume(factory.create_bar_builder(series))
            }
            _ => unreachable!("Unsupported BarBuilderFactories variant"),
        }
    }

    fn create_bar_builder_shared(
        &self,
        shared_series: Arc<Mutex<Self::Series>>,
    ) -> Self::Builder<'static>
    where
        Self::Series: 'static,
    {
        match self {
            BarBuilderFactories::TimeBarFactory(factory) => {
                BarBuilders::Time(factory.create_bar_builder_shared(shared_series))
            }
            BarBuilderFactories::TickBarFactory(factory) => {
                BarBuilders::Tick(factory.create_bar_builder_shared(shared_series))
            }
            BarBuilderFactories::VolumeBarFactory(factory) => {
                BarBuilders::Volume(factory.create_bar_builder_shared(shared_series))
            }
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

    fn time_period(&mut self, period: Duration) -> &mut Self {
        match self {
            Self::Time(b) => {
                b.time_period(period);
                self
            }
            Self::Tick(b) => {
                b.time_period(period);
                self
            }
            Self::Volume(b) => {
                b.time_period(period);
                self
            }
        }
    }

    fn begin_time(&mut self, time: OffsetDateTime) -> &mut Self {
        match self {
            Self::Time(b) => {
                b.begin_time(time);
                self
            }
            Self::Tick(b) => {
                b.begin_time(time);
                self
            }
            Self::Volume(b) => {
                b.begin_time(time);
                self
            }
        }
    }

    fn end_time(&mut self, time: OffsetDateTime) -> &mut Self {
        match self {
            Self::Time(b) => {
                b.end_time(time);
                self
            }
            Self::Tick(b) => {
                b.end_time(time);
                self
            }
            Self::Volume(b) => {
                b.end_time(time);
                self
            }
        }
    }

    fn open_price(&mut self, price: T) -> &mut Self {
        match self {
            Self::Time(b) => {
                b.open_price(price);
                self
            }
            Self::Tick(b) => {
                b.open_price(price);
                self
            }
            Self::Volume(b) => {
                b.open_price(price);
                self
            }
        }
    }

    fn high_price(&mut self, price: T) -> &mut Self {
        match self {
            Self::Time(b) => {
                b.high_price(price);
                self
            }
            Self::Tick(b) => {
                b.high_price(price);
                self
            }
            Self::Volume(b) => {
                b.high_price(price);
                self
            }
        }
    }

    fn low_price(&mut self, price: T) -> &mut Self {
        match self {
            Self::Time(b) => {
                b.low_price(price);
                self
            }
            Self::Tick(b) => {
                b.low_price(price);
                self
            }
            Self::Volume(b) => {
                b.low_price(price);
                self
            }
        }
    }

    fn close_price(&mut self, price: T) -> &mut Self {
        match self {
            Self::Time(b) => {
                b.close_price(price);
                self
            }
            Self::Tick(b) => {
                b.close_price(price);
                self
            }
            Self::Volume(b) => {
                b.close_price(price);
                self
            }
        }
    }

    fn volume(&mut self, volume: T) -> &mut Self {
        match self {
            Self::Time(b) => {
                b.volume(volume);
                self
            }
            Self::Tick(b) => {
                b.volume(volume);
                self
            }
            Self::Volume(b) => {
                b.volume(volume);
                self
            }
        }
    }

    fn amount(&mut self, amount: T) -> &mut Self {
        match self {
            Self::Time(b) => {
                b.amount(amount);
                self
            }
            Self::Tick(b) => {
                b.amount(amount);
                self
            }
            Self::Volume(b) => {
                b.amount(amount);
                self
            }
        }
    }

    fn trades(&mut self, trades: u64) -> &mut Self {
        match self {
            Self::Time(b) => {
                b.trades(trades);
                self
            }
            Self::Tick(b) => {
                b.trades(trades);
                self
            }
            Self::Volume(b) => {
                b.trades(trades);
                self
            }
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

pub fn add_to_option<T: TrNum>(opt: &Option<T>, val: T) -> Option<T> {
    Some(opt.clone().unwrap_or_else(T::zero) + val)
}
