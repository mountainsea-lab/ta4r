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
use crate::bar::builder::factory::volume_bar_builder_factory::VolumeBarBuilderFactory;
use crate::bar::builder::types::{BarBuilderFactories, BarSeriesRef, add_to_option};
use crate::bar::types::{BarBuilder, BarSeries, BarSeriesBuilder};
use crate::num::double_num::DoubleNum;
use crate::num::double_num_factory::DoubleNumFactory;
use crate::num::{NumFactory, TrNum};
use num_traits::FromPrimitive;
use std::sync::{Arc, Mutex};
use time::{Duration, OffsetDateTime};

#[derive(Debug)]
pub struct VolumeBarBuilder<T: TrNum + 'static, S: BarSeries<T>> {
    pub(crate) num_factory: Arc<T::Factory>,
    pub(crate) volume_threshold: T,
    pub(crate) bar_series: Option<BarSeriesRef<S>>,

    pub(crate) time_period: Option<Duration>,
    pub(crate) end_time: Option<OffsetDateTime>,
    pub(crate) open_price: Option<T>,
    pub(crate) high_price: Option<T>,
    pub(crate) low_price: Option<T>,
    pub(crate) close_price: Option<T>,
    pub(crate) volume: T,
    pub(crate) amount: Option<T>,
    pub(crate) trades: u64,
}

// 针对DoubleNum的具体实现，直接调用DoubleNumFactory::instance()
impl<'a, S: BarSeries<DoubleNum>> VolumeBarBuilder<DoubleNum, S> {
    pub fn new_with_default_factory(volume_threshold: i64) -> Self {
        Self::new_with_factory(Arc::new(DoubleNumFactory::instance()), volume_threshold)
    }
}

impl<T: TrNum + 'static, S: BarSeries<T>> VolumeBarBuilder<T, S> {
    pub fn new(volume_threshold: i64) -> Self {
        Self::new_with_factory(Arc::new(T::Factory::default()), volume_threshold)
    }

    pub fn new_with_factory(num_factory: Arc<T::Factory>, volume_threshold: i64) -> Self {
        let volume_threshold = num_factory.num_of_i64(volume_threshold);
        Self {
            num_factory,
            volume_threshold,
            bar_series: None,
            time_period: None,
            end_time: None,
            open_price: None,
            high_price: Some(T::zero()),
            low_price: T::from_i64(i64::MAX),
            close_price: None,
            volume: T::zero(),
            amount: None,
            trades: 0,
        }
    }

    /// 绑定到单线程可变引用（使用 RawMut）
    pub fn bind_to(mut self, series: &mut S) -> Self {
        self.bar_series = Some(BarSeriesRef::RawMut(series as *mut S));
        self
    }

    /// 绑定到多线程共享 Arc<Mutex<S>>
    pub fn bind_shared(mut self, series: Arc<Mutex<S>>) -> Self {
        self.bar_series = Some(BarSeriesRef::Shared(series));
        self
    }

    /// 绑定到裸指针 RawMut（完全 unsafe，调用者保证唯一可变访问）
    pub fn bind_raw(mut self, ptr: *mut S) -> Self {
        self.bar_series = Some(BarSeriesRef::RawMut(ptr));
        self
    }
    // ❗ 保留 self.volume 和 self.end_time
    fn reset(&mut self) {
        self.time_period = None;
        self.open_price = None;
        self.high_price = Some(T::zero());
        self.low_price = T::from_i64(i64::MAX);
        self.close_price = None;
    }

    /// 统一访问 BarSeries 的方法，屏蔽可变引用和锁的差异
    fn with_series<F, R>(&mut self, f: F) -> Result<R, String>
    where
        F: FnOnce(&mut S) -> R,
    {
        match &mut self.bar_series {
            Some(BarSeriesRef::Mut(cell)) => {
                let mut borrow = cell
                    .try_borrow_mut()
                    .map_err(|_| "Failed to borrow RefCell mutably".to_string())?;
                Ok(f(&mut *borrow))
            }
            Some(BarSeriesRef::Shared(arc_mutex)) => {
                let mut locked = arc_mutex.lock().map_err(|_| "Failed to lock bar_series")?;
                Ok(f(&mut *locked))
            }
            Some(BarSeriesRef::RawMut(ptr)) => {
                if ptr.is_null() {
                    return Err("Raw pointer is null".to_string());
                }
                let s: &mut S = unsafe { &mut **ptr };
                Ok(f(s))
            }
            Some(BarSeriesRef::None) | None => Err("No bound bar_series".to_string()),
        }
    }
}
impl<T: TrNum + 'static, S: BarSeries<T>> BarBuilder<T> for VolumeBarBuilder<T, S>
where
    S: BarSeries<T, Bar = BaseBar<T>>,
{
    type Bar = BaseBar<T>;

    fn time_period(&mut self, time_period: Duration) -> &mut Self {
        self.time_period = Some(self.time_period.unwrap_or(Duration::ZERO) + time_period);
        self
    }

    fn begin_time(&mut self, _: OffsetDateTime) -> &mut Self {
        // 忽略调用，仅返回 self（可以在 debug 模式下打印 warning）
        log::warn!("VolumeBar can only be built from closePrice, begin_time is unused");
        self
    }

    fn end_time(&mut self, end_time: OffsetDateTime) -> &mut Self {
        self.end_time = Some(end_time);
        self
    }

    fn open_price(&mut self, _: T) -> &mut Self {
        // 忽略调用，仅返回 self（可以在 debug 模式下打印 warning）
        log::warn!("VolumeBar can only be built from closePrice, open_price is unused");
        self
    }

    fn high_price(&mut self, _: T) -> &mut Self {
        // 忽略调用，仅返回 self（可以在 debug 模式下打印 warning）
        log::warn!("VolumeBar can only be built from closePrice, high_price is unused");
        self
    }

    fn low_price(&mut self, _: T) -> &mut Self {
        // 忽略调用，仅返回 self（可以在 debug 模式下打印 warning）
        log::warn!("VolumeBar can only be built from closePrice, low_price is unused");
        self
    }

    fn close_price(&mut self, price: T) -> &mut Self {
        self.close_price = Some(price.clone());

        if self.open_price.is_none() {
            self.open_price = Some(price.clone());
        }

        match &mut self.high_price {
            Some(high) if price > *high => *high = price.clone(),
            None => self.high_price = Some(price.clone()),
            _ => {}
        }

        match &mut self.low_price {
            Some(low) if price < *low => *low = price,
            None => self.low_price = Some(price),
            _ => {}
        }

        self
    }

    fn volume(&mut self, vol: T) -> &mut Self {
        self.volume = self.volume.clone() + vol;
        self
    }

    fn amount(&mut self, amt: T) -> &mut Self {
        self.amount = add_to_option(&self.amount, amt);
        self
    }

    fn trades(&mut self, trades: u64) -> &mut Self {
        self.trades += trades;
        self
    }

    fn build(&self) -> Result<Self::Bar, String> {
        let time_period = self.time_period.unwrap_or(Duration::ZERO);
        let end_time = self.end_time.unwrap_or_else(|| OffsetDateTime::now_utc());

        // 确保所有必须字段存在
        let open_price = self.open_price.clone();
        let high_price = self.high_price.clone();
        let low_price = self.low_price.clone();
        let close_price = self.close_price.clone();

        let amount = self.amount.clone();

        BaseBar::new(
            time_period,
            end_time,
            open_price,
            high_price,
            low_price,
            close_price,
            self.volume.clone(),
            amount,
            self.trades,
        )
    }

    fn add(&mut self) -> Result<(), String> {
        if self.volume >= self.volume_threshold {
            let mut volume_remainder = T::zero();

            if self.volume > self.volume_threshold {
                volume_remainder = self.volume.clone() - self.volume_threshold.clone();
                self.volume = self.volume_threshold.clone();
            }

            if self.amount.is_none() {
                if let Some(price) = &self.close_price {
                    self.amount = Some(price.clone() * self.volume.clone());
                }
            }

            let bar = self.build()?;

            self.with_series(|series| {
                series.add_bar(bar);
            })?;

            self.volume = volume_remainder;
            self.reset();
        }

        Ok(())
    }
}

#[test]
fn test_volume_bar_builder_add_mut() {
    use time::{Duration, OffsetDateTime};

    let now = OffsetDateTime::now_utc();
    let one_day = Duration::days(1);

    let factory = VolumeBarBuilderFactory::<DoubleNum>::new(4.into());
    let mut series = BaseBarSeriesBuilder::<DoubleNum>::default()
        .with_bar_builder_factory(BarBuilderFactories::VolumeBarFactory(factory))
        .build()
        .unwrap();
    let mut builder = series.bar_builder();

    // -------- First bar: aggregate to volume 4 --------
    builder
        .time_period(one_day)
        .end_time(now)
        .close_price(1.into())
        .volume(1.into())
        .add()
        .unwrap();

    builder
        .time_period(one_day)
        .end_time(now + one_day)
        .close_price(2.into())
        .volume(1.into())
        .add()
        .unwrap();

    builder
        .time_period(one_day)
        .end_time(now + one_day * 2)
        .close_price(5.into())
        .volume(1.into())
        .add()
        .unwrap();

    // total volume now = 3
    // next volume = 2, will cause flush (3+2=5 > 4), flush 4, carry 1
    builder
        .time_period(one_day)
        .end_time(now + one_day * 3)
        .close_price(4.into())
        .volume(2.into())
        .add()
        .unwrap();

    // assert_eq!(series.get_bar_count(), 1); 非共享barSeries无法中间过程自由获取
    // -------- Second bar: aggregate next volume 4 --------
    builder
        .time_period(one_day)
        .end_time(now + one_day * 4)
        .close_price(2.into())
        .volume(1.into())
        .add()
        .unwrap();

    builder
        .time_period(one_day)
        .end_time(now + one_day * 5)
        .close_price(3.into())
        .volume(1.into())
        .add()
        .unwrap();

    builder
        .time_period(one_day)
        .end_time(now + one_day * 6)
        .close_price(6.into())
        .volume(1.into())
        .add()
        .unwrap();

    let bar1 = series.get_bar(0).unwrap();

    assert_eq!(bar1.volume, DoubleNum::from_f64(4.0).unwrap());
    assert_eq!(bar1.open_price, Some(1.into()));
    assert_eq!(bar1.close_price, Some(4.into()));
    assert_eq!(bar1.high_price, Some(5.into()));
    assert_eq!(bar1.low_price, Some(1.into()));
    assert_eq!(bar1.time_period, one_day * 4);
    assert_eq!(bar1.begin_time, now - one_day); // same as Java behavior
    assert_eq!(bar1.end_time, now + one_day * 3);

    assert_eq!(series.get_bar_count(), 2);
    let bar2 = series.get_bar(1).unwrap();

    assert_eq!(bar2.volume, DoubleNum::from_f64(4.0).unwrap());
    assert_eq!(bar2.open_price, Some(2.into()));
    assert_eq!(bar2.close_price, Some(6.into()));
    assert_eq!(bar2.high_price, Some(6.into()));
    assert_eq!(bar2.low_price, Some(2.into()));
    assert_eq!(bar2.time_period, one_day * 3);
    assert_eq!(bar2.begin_time, now + one_day * 3);
    assert_eq!(bar2.end_time, now + one_day * 6);
}

/// 注意：如果使用 `bar_builder_shared(shared_series)` 构造 builder，
/// 请不要在外部提前持有 `shared_series.lock()`，
/// 否则在 builder.add() 内部可能造成死锁。
#[test]
fn test_volume_bar_builder_add_shared() {
    use std::sync::Arc;
    use time::{Duration, OffsetDateTime};

    let now = OffsetDateTime::now_utc();
    let one_day = Duration::days(1);

    let factory = VolumeBarBuilderFactory::<DoubleNum>::new(4.into());
    let mut series = BaseBarSeriesBuilder::<DoubleNum>::default()
        .with_bar_builder_factory(BarBuilderFactories::VolumeBarFactory(factory))
        .build()
        .unwrap();

    // 1. 转成共享指针
    let shared_series = series.into_shared();

    // 2. 不提前持锁，直接通过临时锁获取 builder
    let mut builder = {
        let mut locked = shared_series.lock().unwrap();
        locked.bar_builder_shared(Arc::clone(&shared_series))
    }; // 🔓 locked dropped here, 锁立即释放，避免死锁

    // -------- First bar: aggregate to volume 4 --------
    builder
        .time_period(one_day)
        .end_time(now)
        .close_price(1.into())
        .volume(1.into())
        .add()
        .unwrap();

    builder
        .time_period(one_day)
        .end_time(now + one_day)
        .close_price(2.into())
        .volume(1.into())
        .add()
        .unwrap();

    builder
        .time_period(one_day)
        .end_time(now + one_day * 2)
        .close_price(5.into())
        .volume(1.into())
        .add()
        .unwrap();

    builder
        .time_period(one_day)
        .end_time(now + one_day * 3)
        .close_price(4.into())
        .volume(2.into())
        .add()
        .unwrap();

    // 🔄 只在验证时临时加锁
    {
        let guard = shared_series.lock().unwrap();
        assert_eq!(guard.get_bar_count(), 1);
        let bar1 = guard.get_bar(0).unwrap();

        assert_eq!(bar1.volume, DoubleNum::from_f64(4.0).unwrap());
        assert_eq!(bar1.open_price, Some(1.into()));
        assert_eq!(bar1.close_price, Some(4.into()));
        assert_eq!(bar1.high_price, Some(5.into()));
        assert_eq!(bar1.low_price, Some(1.into()));
        assert_eq!(bar1.time_period, one_day * 4);
        assert_eq!(bar1.begin_time, now - one_day);
        assert_eq!(bar1.end_time, now + one_day * 3);
    }

    // -------- Second bar: aggregate next volume 4 --------
    builder
        .time_period(one_day)
        .end_time(now + one_day * 4)
        .close_price(2.into())
        .volume(1.into())
        .add()
        .unwrap();

    builder
        .time_period(one_day)
        .end_time(now + one_day * 5)
        .close_price(3.into())
        .volume(1.into())
        .add()
        .unwrap();

    builder
        .time_period(one_day)
        .end_time(now + one_day * 6)
        .close_price(6.into())
        .volume(1.into())
        .add()
        .unwrap();

    {
        let guard = shared_series.lock().unwrap();
        assert_eq!(guard.get_bar_count(), 2);
        let bar2 = guard.get_bar(1).unwrap();

        assert_eq!(bar2.volume, DoubleNum::from_f64(4.0).unwrap());
        assert_eq!(bar2.open_price, Some(2.into()));
        assert_eq!(bar2.close_price, Some(6.into()));
        assert_eq!(bar2.high_price, Some(6.into()));
        assert_eq!(bar2.low_price, Some(2.into()));
        assert_eq!(bar2.time_period, one_day * 3);
        assert_eq!(bar2.begin_time, now + one_day * 3);
        assert_eq!(bar2.end_time, now + one_day * 6);
    }
}
