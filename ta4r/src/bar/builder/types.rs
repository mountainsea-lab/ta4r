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
#[cfg(any(test, feature = "enable-mocks"))]
use crate::bar::builder::mocks::mock_bar_builder::MockBarBuilder;
#[cfg(any(test, feature = "enable-mocks"))]
use crate::bar::builder::mocks::mock_bar_builder_factory::MockBarBuilderFactory;
use crate::bar::builder::tick_bar_builder::TickBarBuilder;
use crate::bar::builder::time_bar_builder::TimeBarBuilder;
use crate::bar::builder::volume_bar_builder::VolumeBarBuilder;
use crate::bar::types::{BarBuilder, BarBuilderFactory, BarSeries};
use crate::num::TrNum;
use std::cell::RefCell;
use std::fmt;
use std::marker::PhantomData;
use std::sync::{Arc, Mutex};
use time::{Duration, OffsetDateTime};

/// BarSeries类型封装多种引用方式
#[derive(Debug, Clone)]
pub enum BarSeriesRef<S> {
    /// 单线程安全访问
    Mut(Arc<RefCell<S>>),
    /// 多线程共享
    Shared(Arc<Mutex<S>>),
    /// 原始裸指针访问（零开销，但 unsafe）
    RawMut(*mut S),
    /// 未绑定
    None,
}

impl<S> Default for BarSeriesRef<S> {
    fn default() -> Self {
        BarSeriesRef::None
    }
}

impl<S> BarSeriesRef<S> {
    /// 从单线程 RefCell 创建
    pub fn from_mut(series: S) -> Self {
        BarSeriesRef::Mut(Arc::new(RefCell::new(series)))
    }

    /// 从共享 Arc<Mutex> 创建
    pub fn from_shared(shared: Arc<Mutex<S>>) -> Self {
        BarSeriesRef::Shared(shared)
    }

    /// 从裸指针创建（性能极致，但调用者必须保证唯一性）
    pub fn from_raw(ptr: *mut S) -> Self {
        BarSeriesRef::RawMut(ptr)
    }

    /// 安全访问可变引用，闭包操作统一接口
    pub fn with_mut<F, R>(&self, f: F) -> Result<R, String>
    where
        F: FnOnce(&mut S) -> R,
    {
        match self {
            BarSeriesRef::Mut(cell) => {
                let mut borrow = cell
                    .try_borrow_mut()
                    .map_err(|_| "Failed to borrow RefCell mutably".to_string())?;
                Ok(f(&mut *borrow))
            }
            BarSeriesRef::Shared(arc_mutex) => {
                let mut lock = arc_mutex
                    .lock()
                    .map_err(|_| "Failed to lock Arc<Mutex>".to_string())?;
                Ok(f(&mut *lock))
            }
            // 裸指针 unsafe
            BarSeriesRef::RawMut(ptr) => {
                if ptr.is_null() {
                    return Err("Raw pointer is null".to_string());
                }
                // 复制 ptr，然后在 unsafe 中解引用为 &mut S
                let raw_ptr = *ptr;
                let s: &mut S = unsafe { &mut *raw_ptr };
                Ok(f(s))
            }
            BarSeriesRef::None => Err("No series bound".to_string()),
        }
    }

    /// 绑定单线程 RefCell
    pub fn bind_to(&mut self, series: S) {
        *self = BarSeriesRef::Mut(Arc::new(RefCell::new(series)));
    }

    /// 绑定多线程 Arc<Mutex>
    pub fn bind_shared(&mut self, shared: Arc<Mutex<S>>) {
        *self = BarSeriesRef::Shared(shared);
    }

    /// 绑定裸指针（unsafe，调用者负责唯一性）
    pub fn bind_raw(&mut self, ptr: *mut S) {
        *self = BarSeriesRef::RawMut(ptr);
    }

    /// 获取共享 Arc 版本（仅供多线程使用）
    pub fn get_shared(&self) -> Option<Arc<Mutex<S>>> {
        match self {
            BarSeriesRef::Shared(arc_mutex) => Some(arc_mutex.clone()),
            _ => None,
        }
    }
}

// 枚举包装不同的 BarBuilderFactory 实现
#[derive(Clone)]
pub enum BarBuilderFactories<T: TrNum> {
    TimeBarFactory(TimeBarBuilderFactory<T>),
    TickBarFactory(TickBarBuilderFactory<T>),
    VolumeBarFactory(VolumeBarBuilderFactory<T>),
    #[cfg(feature = "enable-mocks")]
    MockBarFactory(MockBarBuilderFactory<T>),
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
        = BarBuilders<T>
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
            #[cfg(feature = "enable-mocks")]
            BarBuilderFactories::MockBarFactory(factory) => {
                BarBuilders::Mock(factory.create_bar_builder(series))
            }
            _ => unreachable!("Unsupported BarBuilderFactories variant"),
        }
    }

    fn create_bar_builder_shared(
        &self,
        num_factory: Arc<T::Factory>,
        shared_series: Arc<Mutex<Self::Series>>,
    ) -> Self::Builder<'static>
    where
        Self::Series: 'static,
    {
        match self {
            BarBuilderFactories::TimeBarFactory(factory) => {
                BarBuilders::Time(factory.create_bar_builder_shared(num_factory, shared_series))
            }
            BarBuilderFactories::TickBarFactory(factory) => {
                BarBuilders::Tick(factory.create_bar_builder_shared(num_factory, shared_series))
            }
            BarBuilderFactories::VolumeBarFactory(factory) => {
                BarBuilders::Volume(factory.create_bar_builder_shared(num_factory, shared_series))
            }
            #[cfg(feature = "enable-mocks")]
            BarBuilderFactories::MockBarFactory(factory) => {
                BarBuilders::Mock(factory.create_bar_builder_shared(num_factory, shared_series))
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
            #[cfg(feature = "enable-mocks")]
            BarBuilderFactories::MockBarFactory(factory) => {
                f.debug_tuple("MockBarFactory").field(factory).finish()
            }
            // 如果添加了其他变体，继续写匹配
            BarBuilderFactories::_Phantom(_) => {
                f.debug_tuple("_Phantom").field(&"PhantomData").finish()
            }
        }
    }
}

#[derive(Debug)]
pub enum BarBuilders<T: TrNum + 'static> {
    Time(TimeBarBuilder<T, BaseBarSeries<T>>),
    Tick(TickBarBuilder<T, BaseBarSeries<T>>),
    Volume(VolumeBarBuilder<T, BaseBarSeries<T>>),
    #[cfg(feature = "enable-mocks")]
    Mock(MockBarBuilder<T, BaseBarSeries<T>>),
}

impl<T: TrNum + 'static> BarBuilder<T> for BarBuilders<T> {
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
            #[cfg(feature = "enable-mocks")]
            Self::Mock(b) => {
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
            #[cfg(feature = "enable-mocks")]
            Self::Mock(b) => {
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
            #[cfg(feature = "enable-mocks")]
            Self::Mock(b) => {
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
            #[cfg(feature = "enable-mocks")]
            Self::Mock(b) => {
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
            #[cfg(feature = "enable-mocks")]
            Self::Mock(b) => {
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
            #[cfg(feature = "enable-mocks")]
            Self::Mock(b) => {
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
            #[cfg(feature = "enable-mocks")]
            Self::Mock(b) => {
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
            #[cfg(feature = "enable-mocks")]
            Self::Mock(b) => {
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
            #[cfg(feature = "enable-mocks")]
            Self::Mock(b) => {
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
            #[cfg(feature = "enable-mocks")]
            Self::Mock(b) => {
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
            #[cfg(feature = "enable-mocks")]
            Self::Mock(b) => b.build(),
        }
    }

    fn add(&mut self) -> Result<(), String> {
        match self {
            Self::Time(b) => b.add(),
            Self::Tick(b) => b.add(),
            Self::Volume(b) => b.add(),
            #[cfg(feature = "enable-mocks")]
            Self::Mock(b) => b.add(),
        }
    }
}

pub fn add_to_option<T: TrNum>(opt: &Option<T>, val: T) -> Option<T> {
    Some(opt.clone().unwrap_or_else(T::zero) + val)
}
