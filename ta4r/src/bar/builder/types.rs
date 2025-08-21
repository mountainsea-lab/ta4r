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
use crate::indicators::types::IndicatorError;
use crate::num::TrNum;
use parking_lot::RwLock;
use std::cell::RefCell;
use std::fmt;
use std::marker::PhantomData;
use std::ops::Deref;
use std::sync::Arc;
use time::{Duration, OffsetDateTime};

/// BarSeries类型封装多种引用方式
#[derive(Debug)]
pub enum BarSeriesRef<S> {
    /// 单线程安全访问
    Mut(Arc<RefCell<S>>),
    /// 多线程共享
    Shared(Arc<RwLock<S>>),
    /// 原始裸指针访问（零开销，但 unsafe）
    RawMut(*mut S),
    /// 未绑定
    None,
}

impl<S> Clone for BarSeriesRef<S> {
    fn clone(&self) -> Self {
        match self {
            BarSeriesRef::Mut(rc) => BarSeriesRef::Mut(rc.clone()), // Arc<RefCell<S>> 可以 clone
            BarSeriesRef::Shared(arc_rwlock) => BarSeriesRef::Shared(arc_rwlock.clone()), // Arc<RwLock<S>>
            BarSeriesRef::RawMut(ptr) => BarSeriesRef::RawMut(*ptr), // 仅复制指针
            BarSeriesRef::None => BarSeriesRef::None,
        }
    }
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
    pub fn from_shared(shared: Arc<RwLock<S>>) -> Self {
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
            BarSeriesRef::Shared(arc_rwlock) => {
                let mut locked = arc_rwlock.write();
                Ok(f(&mut *locked))
            }
            BarSeriesRef::RawMut(ptr) => {
                if ptr.is_null() {
                    return Err("Raw pointer is null".to_string());
                }
                let s: &mut S = unsafe { &mut **ptr };
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
    pub fn bind_shared(&mut self, shared: Arc<RwLock<S>>) {
        *self = BarSeriesRef::Shared(shared);
    }

    /// 绑定裸指针（unsafe，调用者负责唯一性）
    pub fn bind_raw(&mut self, ptr: *mut S) {
        *self = BarSeriesRef::RawMut(ptr);
    }

    /// 获取共享 Arc 版本（仅供多线程使用）
    pub fn get_shared(&self) -> Option<Arc<RwLock<S>>> {
        match self {
            BarSeriesRef::Shared(arc_mutex) => Some(arc_mutex.clone()),
            _ => None,
        }
    }

    /// 安全访问不可变引用
    pub fn with_ref<F, R>(&self, f: F) -> Result<R, String>
    where
        F: FnOnce(&S) -> R,
    {
        match self {
            BarSeriesRef::Mut(cell) => {
                let borrow = cell
                    .try_borrow()
                    .map_err(|_| "Failed to borrow RefCell immutably".to_string())?;
                Ok(f(&*borrow))
            }
            BarSeriesRef::Shared(arc_rwlock) => {
                let lock = arc_rwlock.read();
                Ok(f(&*lock))
            }
            BarSeriesRef::RawMut(ptr) => {
                if ptr.is_null() {
                    return Err("Raw pointer is null".to_string());
                }
                let s: &S = unsafe { &*(*ptr) };
                Ok(f(s))
            }
            BarSeriesRef::None => Err("No series bound".to_string()),
        }
    }

    /// 访问内部 BarSeries，并在 None 或空时返回默认值
    pub fn with_ref_or<R, F>(&self, default: R, f: F) -> R
    where
        F: FnOnce(&S) -> R,
    {
        match self {
            BarSeriesRef::Mut(rc) => f(&rc.borrow()),
            BarSeriesRef::Shared(arc_rwlock) => f(&arc_rwlock.read()),
            BarSeriesRef::RawMut(ptr) => unsafe { if ptr.is_null() { default } else { f(&**ptr) } },
            BarSeriesRef::None => default,
        }
    }

    /// 检查裸指针是否为空
    pub fn is_raw_null(&self) -> bool {
        matches!(self, BarSeriesRef::RawMut(ptr) if ptr.is_null())
    }

    /// 统一获取 begin index
    pub fn get_begin_index<F>(&self, f: F) -> Option<usize>
    where
        F: Fn(&S) -> usize,
    {
        self.with_ref(f).ok()
    }

    /// 统一获取 end index
    pub fn get_end_index<F>(&self, f: F) -> Option<usize>
    where
        F: Fn(&S) -> usize,
    {
        self.with_ref(f).ok()
    }

    /// 从内部安全提取一个值（需要返回可克隆对象）
    pub fn with_cloned<R, F>(&self, f: F) -> Result<R, String>
    where
        R: Clone,
        F: FnOnce(&S) -> &R,
    {
        self.with_ref(|s| f(s).clone())
    }
}

impl<S> fmt::Display for BarSeriesRef<S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BarSeriesRef::Mut(_) => write!(f, "BarSeriesRef::Mut"),
            BarSeriesRef::Shared(_) => write!(f, "BarSeriesRef::Shared"),
            BarSeriesRef::RawMut(_) => write!(f, "BarSeriesRef::RawMut"),
            BarSeriesRef::None => write!(f, "BarSeriesRef::None"),
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
        shared_series: Arc<RwLock<Self::Series>>,
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
