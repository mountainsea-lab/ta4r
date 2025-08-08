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
use crate::bar::builder::factory::time_bar_builder_factory::TimeBarBuilderFactory;
use crate::bar::builder::types::{BarBuilderFactories, BarSeriesRef};
use crate::bar::types::{BarBuilder, BarSeries, BarSeriesBuilder};
use crate::num::TrNum;
use crate::num::double_num::DoubleNum;
use crate::num::double_num_factory::DoubleNumFactory;
use std::sync::{Arc, Mutex};
use time::{Duration, OffsetDateTime};
use time_macros::datetime;

/// TimeBarBuilder 结构体 - 使用泛型参数避免动态分发
#[derive(Debug)]
pub struct TimeBarBuilder<'a, T: TrNum + 'static, S: BarSeries<'a, T>> {
    /// 数值工厂
    pub num_factory: Arc<T::Factory>,
    /// 绑定的 BarSeries（可选，使用泛型参数）
    pub(crate) bar_series: Option<BarSeriesRef<'a, S>>,

    // Bar 构建字段
    pub time_period: Option<Duration>,
    pub begin_time: Option<OffsetDateTime>,
    pub end_time: Option<OffsetDateTime>,
    pub open_price: Option<T>,
    pub high_price: Option<T>,
    pub low_price: Option<T>,
    pub close_price: Option<T>,
    pub volume: Option<T>,
    pub amount: Option<T>,
    pub trades: Option<u64>,
}

// 针对DoubleNum的具体实现，直接调用DoubleNumFactory::instance()
impl<'a, S: BarSeries<'a, DoubleNum>> TimeBarBuilder<'a, DoubleNum, S> {
    pub fn new() -> Self {
        Self::new_with_factory(Arc::new(DoubleNumFactory::instance()))
    }
}

// 额外为泛型实现 Default trait
impl<'a, T: TrNum + 'static, S: BarSeries<'a, T>> Default for TimeBarBuilder<'a, T, S>
where
    T::Factory: Default,
{
    fn default() -> Self {
        Self::new_with_factory(Arc::new(T::Factory::default()))
    }
}

impl<'a, T: TrNum + 'static, S: BarSeries<'a, T>> TimeBarBuilder<'a, T, S> {
    /// 创建新的 TimeBarBuilder，指定数值工厂
    pub fn new_with_factory(num_factory: Arc<T::Factory>) -> Self {
        Self {
            num_factory,
            bar_series: None,
            time_period: None,
            begin_time: None,
            end_time: None,
            open_price: None,
            high_price: None,
            low_price: None,
            close_price: None,
            volume: None,
            amount: None,
            trades: None,
        }
    }

    /// 绑定到 BarSeries
    pub fn bind_to(mut self, series: &'a mut S) -> Self {
        self.bar_series = Some(BarSeriesRef::Mut(series));
        self
    }

    pub fn bind_shared(mut self, series: Arc<Mutex<S>>) -> Self {
        self.bar_series = Some(BarSeriesRef::Shared(series));
        self
    }

    /// 统一访问 BarSeries 的方法，屏蔽可变引用和锁的差异
    fn with_series<F, R>(&mut self, f: F) -> Result<R, String>
    where
        F: FnOnce(&mut S) -> R,
    {
        match &mut self.bar_series {
            Some(BarSeriesRef::Mut(series)) => Ok(f(*series)),
            Some(BarSeriesRef::Shared(arc_mutex)) => {
                let mut locked = arc_mutex.lock().map_err(|_| "Failed to lock bar_series")?;
                Ok(f(&mut *locked))
            }
            None => Err("No bound bar_series".to_string()),
        }
    }

    /// 克隆 builder，但不包含 bar_series（不可 Clone）
    pub fn clone_without_series(&self) -> Self {
        Self {
            num_factory: Arc::clone(&self.num_factory),
            bar_series: None,
            time_period: self.time_period,
            begin_time: self.begin_time,
            end_time: self.end_time,
            open_price: self.open_price.clone(),
            high_price: self.high_price.clone(),
            low_price: self.low_price.clone(),
            close_price: self.close_price.clone(),
            volume: self.volume.clone(),
            amount: self.amount.clone(),
            trades: self.trades,
        }
    }
}

impl<'a, T: TrNum + 'static, S: BarSeries<'a, T>> BarBuilder<T> for TimeBarBuilder<'a, T, S>
where
    S: BarSeries<'a, T, Bar = BaseBar<T>>,
{
    type Bar = BaseBar<T>;

    fn time_period(&mut self, time_period: Duration) -> &mut Self {
        self.time_period = Some(time_period);
        self
    }

    fn begin_time(&mut self, begin_time: OffsetDateTime) -> &mut Self {
        self.begin_time = Some(begin_time);
        self
    }

    fn end_time(&mut self, end_time: OffsetDateTime) -> &mut Self {
        self.end_time = Some(end_time);
        self
    }

    fn open_price(&mut self, open_price: T) -> &mut Self {
        self.open_price = Some(open_price);
        self
    }

    fn high_price(&mut self, high_price: T) -> &mut Self {
        self.high_price = Some(high_price);
        self
    }

    fn low_price(&mut self, low_price: T) -> &mut Self {
        self.low_price = Some(low_price);
        self
    }

    fn close_price(&mut self, close_price: T) -> &mut Self {
        self.close_price = Some(close_price);
        self
    }

    fn volume(&mut self, volume: T) -> &mut Self {
        self.volume = Some(volume);
        self
    }

    fn amount(&mut self, amount: T) -> &mut Self {
        self.amount = Some(amount);
        self
    }

    fn trades(&mut self, trades: u64) -> &mut Self {
        self.trades = Some(trades);
        self
    }

    fn build(&self) -> Result<Self::Bar, String> {
        let time_period = self.time_period.unwrap_or(Duration::ZERO);
        let end_time = self.end_time.unwrap_or_else(|| OffsetDateTime::now_utc());

        // 确保所有必须字段存在
        let open_price = self.open_price.clone().ok_or("Missing open_price")?;
        let high_price = self.high_price.clone().ok_or("Missing high_price")?;
        let low_price = self.low_price.clone().ok_or("Missing low_price")?;
        let close_price = self.close_price.clone().ok_or("Missing close_price")?;

        let volume = self.volume.clone().unwrap_or_else(|| T::zero());
        let amount = self.amount.clone();
        let trades = self.trades.unwrap_or(0);
        // 构建 BaseBar，对应 Java 版本的 build 方法
        BaseBar::new(
            time_period,
            end_time,
            open_price,
            high_price,
            low_price,
            close_price,
            volume,
            amount,
            trades,
        )
    }

    fn add(&mut self) -> Result<(), String> {
        // 自动推导 amount = close_price * volume
        if self.amount.is_none() {
            if let (Some(ref close), Some(ref volume)) =
                (self.close_price.as_ref(), self.volume.as_ref())
            {
                self.amount = Some(close.multiplied_by(volume));
            }
        }

        let bar = self.build()?;

        self.with_series(|series| {
            series.add_bar(bar);
        })?;

        Ok(())
    }
}

#[test]
fn test_time_bar_builder_build() {
    use crate::num::decimal_num::DecimalNum;
    // 构造时间

    let begin_time = datetime!(2014-06-25 00:00:00 UTC);
    let end_time = datetime!(2014-06-25 01:00:00 UTC);
    let duration = end_time - begin_time;

    // 创建 TimeBarBuilderFactory 并构造 bar_series
    let time_factory = TimeBarBuilderFactory::<DecimalNum>::new();
    let mut series = BaseBarSeriesBuilder::<DecimalNum>::default()
        .with_bar_builder_factory(BarBuilderFactories::TimeBarFactory(time_factory))
        .build()
        .unwrap();

    // 获取 builder
    let mut builder = series.bar_builder();

    builder
        .time_period(duration)
        .end_time(end_time)
        .open_price(DecimalNum::from(101))
        .high_price(DecimalNum::from(103))
        .low_price(DecimalNum::from(100))
        .close_price(DecimalNum::from(102))
        .trades(4)
        .volume(DecimalNum::from(40))
        .amount(DecimalNum::from(4020))
        .add()
        .unwrap();

    // 验证结果
    assert_eq!(series.get_bar_count(), 1);
    let bar = series.get_bar(0).unwrap();

    assert_eq!(bar.time_period, duration);
    assert_eq!(bar.begin_time, begin_time);
    assert_eq!(bar.end_time, end_time);
    assert_eq!(bar.open_price, Some(DecimalNum::from(101)));
    assert_eq!(bar.high_price, Some(DecimalNum::from(103)));
    assert_eq!(bar.low_price, Some(DecimalNum::from(100)));
    assert_eq!(bar.close_price, Some(DecimalNum::from(102)));
    assert_eq!(bar.volume, DecimalNum::from(40));
    assert_eq!(bar.amount, Some(DecimalNum::from(4020)));
    assert_eq!(bar.trades, 4);
}
