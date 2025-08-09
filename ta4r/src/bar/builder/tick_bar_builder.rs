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
use crate::bar::builder::factory::tick_bar_builder_factory::TickBarBuilderFactory;
use crate::bar::builder::types::{BarBuilderFactories, BarSeriesRef, add_to_option};
use crate::bar::types::{BarBuilder, BarSeries, BarSeriesBuilder};
use crate::num::TrNum;
use crate::num::decimal_num::DecimalNum;
use std::sync::{Arc, Mutex};
use time::{Duration, OffsetDateTime};

/// TickBarBuilder 结构体 - 使用泛型参数避免动态分发
#[derive(Debug)]
pub struct TickBarBuilder<'a, T: TrNum + 'static, S: BarSeries<'a, T>> {
    /// 数值工厂
    num_factory: Arc<T::Factory>,
    /// 触发新 Bar 的交易次数阈值
    tick_count: u64,
    /// 当前已处理的交易次数
    passed_ticks_count: u64,
    pub(crate) bar_series: Option<BarSeriesRef<'a, S>>,
    // Bar 构建字段
    time_period: Option<Duration>,
    end_time: Option<OffsetDateTime>,
    open_price: Option<T>,
    high_price: Option<T>,
    low_price: Option<T>,
    close_price: Option<T>,
    volume: Option<T>,
    amount: Option<T>,
    trades: u64,
}

impl<'a, T: TrNum + 'static, S: BarSeries<'a, T>> TickBarBuilder<'a, T, S> {
    /// 创建新的 TickBarBuilder，使用默认数值工厂
    pub fn new(tick_count: u64) -> Self
    where
        T::Factory: Default,
    {
        Self::new_with_factory(Arc::new(T::Factory::default()), tick_count)
    }

    /// 创建新的 TickBarBuilder，指定数值工厂
    pub fn new_with_factory(num_factory: Arc<T::Factory>, tick_count: u64) -> Self {
        Self {
            num_factory,
            tick_count,
            passed_ticks_count: 0,
            bar_series: None,
            time_period: None,
            end_time: None,
            open_price: None,
            high_price: None,
            low_price: None,
            close_price: None,
            volume: None,
            amount: None,
            trades: 0u64,
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

    /// 重置构建器状态
    fn reset(&mut self) {
        self.time_period = None;
        self.open_price = None;
        self.high_price = None;
        self.low_price = T::from_i64(i64::MAX);
        self.close_price = None;
        self.volume = None;
        self.amount = None;
        self.trades = 0u64;
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
}

impl<'a, T: TrNum + 'static, S: BarSeries<'a, T>> BarBuilder<T> for TickBarBuilder<'a, T, S>
where
    S: BarSeries<'a, T, Bar = BaseBar<T>>,
{
    type Bar = BaseBar<T>;
    fn time_period(&mut self, time_period: Duration) -> &mut Self {
        self.time_period = match self.time_period {
            Some(existing) => Some(existing + time_period),
            None => Some(time_period),
        };
        self
    }

    fn begin_time(&mut self, _time: OffsetDateTime) -> &mut Self {
        panic!("TickBar can only be built from closePrice");
    }

    fn end_time(&mut self, end_time: OffsetDateTime) -> &mut Self {
        self.end_time = Some(end_time);
        self
    }

    fn open_price(&mut self, _open_price: T) -> &mut Self {
        panic!("TickBar can only be built from closePrice");
    }

    fn high_price(&mut self, _high_price: T) -> &mut Self {
        panic!("TickBar can only be built from closePrice");
    }

    fn low_price(&mut self, _low_price: T) -> &mut Self {
        panic!("TickBar can only be built from closePrice");
    }

    fn close_price(&mut self, tick_price: T) -> &mut Self {
        // move 一次 tick_price，拆成多个变量避免多次 clone
        let price = tick_price;

        self.close_price = Some(price.clone());

        if self.open_price.is_none() {
            self.open_price = Some(price.clone());
        }

        match &mut self.high_price {
            Some(high) => {
                if price > *high {
                    *high = price.clone();
                }
            }
            None => {
                self.high_price = Some(price.clone());
            }
        }

        match &mut self.low_price {
            Some(low) => {
                if price < *low {
                    *low = price;
                }
            }
            None => {
                self.low_price = Some(price);
            }
        }

        self
    }

    fn volume(&mut self, volume: T) -> &mut Self {
        self.volume = add_to_option(&self.volume, volume);
        self
    }

    fn amount(&mut self, amount: T) -> &mut Self {
        self.amount = add_to_option(&self.amount, amount);
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

        let volume = self.volume.clone().unwrap_or_else(|| T::zero());
        let amount = self.amount.clone();
        let trades = self.trades;

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
        self.passed_ticks_count += 1;

        if self.passed_ticks_count % self.tick_count == 0 {
            // 计算 amount，如果为空且 volume 和 close_price 都有值
            if self.amount.is_none() {
                if let (Some(close_price), Some(volume)) = (&self.close_price, &self.volume) {
                    self.amount = Some(close_price.clone() * volume.clone());
                }
            }

            let bar = self.build()?;

            self.with_series(|series| {
                series.add_bar(bar);
            })?;

            self.reset();
        }

        Ok(())
    }
}

#[test]
fn test_tick_bar_builder_add() {
    use crate::num::decimal_num::DecimalNum;

    use time::Duration;

    let tick_factory = TickBarBuilderFactory::<DecimalNum>::new(5);
    let mut series = BaseBarSeriesBuilder::<DecimalNum>::default()
        .with_bar_builder_factory(BarBuilderFactories::TickBarFactory(tick_factory))
        .build()
        .unwrap();

    let now = OffsetDateTime::now_utc();
    let one_day = Duration::days(1);
    // ---- 第一组 Tick (1~5) ----
    {
        // 获取可变的 builder
        let mut builder = series.bar_builder();
        // Tick 1~5
        builder
            .time_period(one_day)
            .end_time(now)
            .close_price(DecimalNum::from(1))
            .volume(DecimalNum::from(1))
            .add()
            .unwrap();

        builder
            .time_period(one_day)
            .end_time(now + one_day)
            .close_price(DecimalNum::from(2))
            .volume(DecimalNum::from(1))
            .add()
            .unwrap();

        builder
            .time_period(one_day)
            .end_time(now + one_day * 2)
            .close_price(DecimalNum::from(5))
            .volume(DecimalNum::from(1))
            .trades(9)
            .add()
            .unwrap();

        builder
            .time_period(one_day)
            .end_time(now + one_day * 3)
            .close_price(DecimalNum::from(1))
            .volume(DecimalNum::from(1))
            .add()
            .unwrap();

        builder
            .time_period(one_day)
            .end_time(now + one_day * 4)
            .close_price(DecimalNum::from(4))
            .volume(DecimalNum::from(2))
            .trades(1)
            .add()
            .unwrap();
    }
    assert_eq!(series.get_bar_count(), 1);
    let bar1 = series.get_bar(0).unwrap();

    assert_eq!(bar1.volume, DecimalNum::from(6));
    assert_eq!(bar1.open_price, Some(DecimalNum::from(1)));
    assert_eq!(bar1.close_price, Some(DecimalNum::from(4)));
    assert_eq!(bar1.high_price, Some(DecimalNum::from(5)));
    assert_eq!(bar1.low_price, Some(DecimalNum::from(1)));
    assert_eq!(bar1.time_period, one_day * 5);
    assert_eq!(bar1.begin_time, now - one_day); // 可选：实现 begin_time 推断
    assert_eq!(bar1.end_time, now + one_day * 4);
    assert_eq!(bar1.amount, Some(DecimalNum::from(24)));
    assert_eq!(bar1.trades, 10);
    // ---- 第二组 Tick (6~10) ----
    {
        let mut builder = series.bar_builder();

        // Tick 6~10
        builder
            .time_period(one_day)
            .end_time(now + one_day * 5)
            .close_price(DecimalNum::from(2))
            .volume(DecimalNum::from(1))
            .amount(DecimalNum::from(24))
            .add()
            .unwrap();

        builder
            .time_period(one_day)
            .end_time(now + one_day * 6)
            .close_price(DecimalNum::from(3))
            .volume(DecimalNum::from(1))
            .add()
            .unwrap();

        builder
            .time_period(one_day)
            .end_time(now + one_day * 7)
            .close_price(DecimalNum::from(6))
            .volume(DecimalNum::from(2))
            .add()
            .unwrap();

        builder
            .time_period(one_day)
            .end_time(now + one_day * 8)
            .close_price(DecimalNum::from(2))
            .volume(DecimalNum::from(1))
            .add()
            .unwrap();

        builder
            .time_period(one_day)
            .end_time(now + one_day * 9)
            .close_price(DecimalNum::from(5))
            .volume(DecimalNum::from(2))
            .trades(100)
            .add()
            .unwrap();
    }
    assert_eq!(series.get_bar_count(), 2);
    let bar2 = series.get_bar(1).unwrap();
    assert_eq!(bar2.volume, DecimalNum::from(7));
    assert_eq!(bar2.open_price, Some(DecimalNum::from(2)));
    assert_eq!(bar2.close_price, Some(DecimalNum::from(5)));
    assert_eq!(bar2.high_price, Some(DecimalNum::from(6)));
    assert_eq!(bar2.low_price, Some(DecimalNum::from(2)));
    assert_eq!(bar2.time_period, one_day * 5);
    assert_eq!(bar2.begin_time, now + one_day * 4); // or +5 - 1d，视 begin_time 逻辑
    assert_eq!(bar2.end_time, now + one_day * 9);
    assert_eq!(bar2.amount, Some(DecimalNum::from(24)));
    assert_eq!(bar2.trades, 100);
}
