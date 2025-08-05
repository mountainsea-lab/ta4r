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

use std::sync::{Arc, Mutex};
use time::{Duration, OffsetDateTime};
use crate::bar::base_bar::BaseBar;
use crate::bar::builder::types::{add_to_option, BarBuilderFactories, BarSeriesRef};
use crate::bar::types::{BarBuilder, BarSeries, BarSeriesBuilder};
use crate::num::double_num::DoubleNum;
use crate::num::double_num_factory::DoubleNumFactory;
use crate::num::{NumFactory, TrNum};
use crate::bar::base_bar_series_builder::BaseBarSeriesBuilder;
use crate::bar::builder::factory::volume_bar_builder_factory::VolumeBarBuilderFactory;

// #[derive(Debug)]
// pub struct VolumeBarBuilder<'a, T: TrNum + 'static, S: BarSeries<'a, T>> {
//     num_factory: Arc<T::Factory>,
//     volume_threshold: T,
//     bar_series: Option<&'a mut S>,
//
//     time_period: Option<Duration>,
//     end_time: Option<OffsetDateTime>,
//     open_price: Option<T>,
//     high_price: Option<T>,
//     low_price: Option<T>,
//     close_price: Option<T>,
//     volume: T,
//     amount: Option<T>,
//     trades: u64,
// }
//
// // 针对DoubleNum的具体实现，直接调用DoubleNumFactory::instance()
// impl<'a, S: BarSeries<'a, DoubleNum>> VolumeBarBuilder<'a, DoubleNum, S> {
//     pub fn new_with_default_factory(volume_threshold: i64) -> Self {
//         Self::new_with_factory(Arc::new(DoubleNumFactory::instance()), volume_threshold)
//     }
// }
//
// impl<'a, T: TrNum + 'static, S: BarSeries<'a, T>> VolumeBarBuilder<'a, T, S> {
//     pub fn new(volume_threshold: i64) -> Self {
//         Self::new_with_factory(Arc::new(T::Factory::default()), volume_threshold)
//     }
//
//     pub fn new_with_factory(num_factory: Arc<T::Factory>, volume_threshold: i64) -> Self {
//         let volume_threshold = num_factory.num_of_i64(volume_threshold);
//         Self {
//             num_factory,
//             volume_threshold,
//             bar_series: None,
//             time_period: None,
//             end_time: None,
//             open_price: None,
//             high_price: Some(T::zero()),
//             low_price: T::from_i64(i64::MAX),
//             close_price: None,
//             volume: T::zero(),
//             amount: None,
//             trades: 0,
//         }
//     }
//
//     pub fn bind_to(mut self, bar_series: &'a mut S) -> Self {
//         self.bar_series = Some(bar_series);
//         self
//     }
//
//     // ❗ 保留 self.volume 和 self.end_time
//     fn reset(&mut self) {
//
//         self.time_period = None;
//         self.open_price = None;
//         self.high_price = Some(T::zero());
//         self.low_price = T::from_i64(i64::MAX);
//         self.close_price = None;
//
//     }
//
// }
// impl<'a, T: TrNum + 'static, S: BarSeries<'a, T>> BarBuilder<T> for VolumeBarBuilder<'a, T, S>
// where
//     S: BarSeries<'a, T, Bar = BaseBar<T>>,
// {
//     type Bar = BaseBar<T>;
//
//     fn time_period(&mut self, time_period: Duration) -> &mut Self {
//         self.time_period = Some(self.time_period.unwrap_or(Duration::ZERO) + time_period);
//         self
//     }
//
//     fn begin_time(&mut self, _: OffsetDateTime) -> &mut Self {
//         panic!("VolumeBar can only be built from closePrice");
//     }
//
//     fn end_time(&mut self, end_time: OffsetDateTime) -> &mut Self {
//         self.end_time = Some(end_time);
//         self
//     }
//
//     fn open_price(&mut self, _: T) -> &mut Self {
//         panic!("VolumeBar can only be built from closePrice");
//     }
//
//     fn high_price(&mut self, _: T) -> &mut Self {
//         panic!("VolumeBar can only be built from closePrice");
//     }
//
//     fn low_price(&mut self, _: T) -> &mut Self {
//         panic!("VolumeBar can only be built from closePrice");
//     }
//
//     fn close_price(&mut self, price: T) -> &mut Self {
//         self.close_price = Some(price.clone());
//
//         if self.open_price.is_none() {
//             self.open_price = Some(price.clone());
//         }
//
//         match &mut self.high_price {
//             Some(high) if price > *high => *high = price.clone(),
//             None => self.high_price = Some(price.clone()),
//             _ => {}
//         }
//
//         match &mut self.low_price {
//             Some(low) if price < *low => *low = price,
//             None => self.low_price = Some(price),
//             _ => {}
//         }
//
//         self
//     }
//
//     fn volume(&mut self, vol: T) -> &mut Self {
//         self.volume = self.volume.clone() + vol;
//         self
//     }
//
//     fn amount(&mut self, amt: T) -> &mut Self {
//         self.amount = add_to_option(&self.amount, amt);
//         self
//     }
//
//     fn trades(&mut self, trades: u64) -> &mut Self {
//         self.trades += trades;
//         self
//     }
//
//     fn build(&self) -> Result<Self::Bar, String> {
//         let time_period = self.time_period.unwrap_or(Duration::ZERO);
//         let end_time = self.end_time.unwrap_or_else(|| OffsetDateTime::now_utc());
//
//         // 确保所有必须字段存在
//         let open_price = self.open_price.clone().ok_or("Missing open_price")?;
//         let high_price = self.high_price.clone().ok_or("Missing high_price")?;
//         let low_price = self.low_price.clone().ok_or("Missing low_price")?;
//         let close_price = self.close_price.clone().ok_or("Missing close_price")?;
//
//         let amount = self.amount.clone();
//
//         BaseBar::new(
//             time_period,
//             end_time,
//             open_price,
//             high_price,
//             low_price,
//             close_price,
//             self.volume.clone(),
//             amount,
//             self.trades,
//         )
//     }
//
//     fn add(&mut self) -> Result<(), String> {
//         if self.volume >= self.volume_threshold {
//             let mut volume_remainder = T::zero();
//
//             if self.volume > self.volume_threshold {
//                 volume_remainder = self.volume.clone() - self.volume_threshold.clone();
//                 self.volume = self.volume_threshold.clone();
//             }
//
//             if self.amount.is_none() {
//                 if let Some(price) = &self.close_price {
//                     self.amount = Some(price.clone() * self.volume.clone());
//                 }
//             }
//
//             let bar = self.build()?;
//             if let Some(ref mut series) = self.bar_series {
//                 series.add_bar(bar);
//             }
//
//             self.volume = volume_remainder;
//             self.reset();
//         }
//
//         Ok(())
//     }
// }
//
// #[test]
// fn test_volume_bar_builder_add() {
//     use crate::num::decimal_num::DecimalNum;
//
//     use time::Duration;
//     use time::OffsetDateTime;
//
//     let factory = VolumeBarBuilderFactory::<DecimalNum>::new(4.into());
//     let mut series = BaseBarSeriesBuilder::<DecimalNum>::default()
//         .with_bar_builder_factory(BarBuilderFactories::VolumeBarFactory(factory))
//         .build()
//         .unwrap();
//
//     let now = OffsetDateTime::now_utc();
//     let one_day = Duration::days(1);
//
//
//     {
//         let mut builder = series.bar_builder();
//
//         // -------- First bar: aggregate to volume 4 --------
//         builder
//             .time_period(one_day)
//             .end_time(now)
//             .close_price(1.into())
//             .volume(1.into())
//             .add()
//             .unwrap();
//
//         builder
//             .time_period(one_day)
//             .end_time(now + one_day)
//             .close_price(2.into())
//             .volume(1.into())
//             .add()
//             .unwrap();
//
//         builder
//             .time_period(one_day)
//             .end_time(now + one_day * 2)
//             .close_price(5.into())
//             .volume(1.into())
//             .add()
//             .unwrap();
//
//         // total volume now = 3
//         // next volume = 2, will cause flush (3+2=5 > 4), flush 4, carry 1
//         builder
//             .time_period(one_day)
//             .end_time(now + one_day * 3)
//             .close_price(4.into())
//             .volume(2.into())
//             .add()
//             .unwrap();
//
//         // -------- Second bar: aggregate next volume 4 --------
//         builder
//             .time_period(one_day)
//             .end_time(now + one_day * 4)
//             .close_price(2.into())
//             .volume(1.into())
//             .add()
//             .unwrap();
//
//         builder
//             .time_period(one_day)
//             .end_time(now + one_day * 5)
//             .close_price(3.into())
//             .volume(1.into())
//             .add()
//             .unwrap();
//
//         builder
//             .time_period(one_day)
//             .end_time(now + one_day * 6)
//             .close_price(6.into())
//             .volume(1.into())
//             .add()
//             .unwrap();
//     }
//
//     // assert_eq!(series.get_bar_count(), 1);
//     let bar1 = series.get_bar(0).unwrap();
//
//     assert_eq!(bar1.volume, DecimalNum::from(4));
//     assert_eq!(bar1.open_price, Some(1.into()));
//     assert_eq!(bar1.close_price, Some(4.into()));
//     assert_eq!(bar1.high_price, Some(5.into()));
//     assert_eq!(bar1.low_price, Some(1.into()));
//     assert_eq!(bar1.time_period, one_day * 4);
//     assert_eq!(bar1.begin_time, now - one_day); // same as Java behavior
//     assert_eq!(bar1.end_time, now + one_day * 3);
//
//     // // -------- Second bar: aggregate next volume 4 --------
//     // {
//     //     let mut builder = series.bar_builder();
//     //
//     //     builder
//     //         .time_period(one_day)
//     //         .end_time(now + one_day * 4)
//     //         .close_price(2.into())
//     //         .volume(1.into())
//     //         .add()
//     //         .unwrap();
//     //
//     //     builder
//     //         .time_period(one_day)
//     //         .end_time(now + one_day * 5)
//     //         .close_price(3.into())
//     //         .volume(1.into())
//     //         .add()
//     //         .unwrap();
//     //
//     //     builder
//     //         .time_period(one_day)
//     //         .end_time(now + one_day * 6)
//     //         .close_price(6.into())
//     //         .volume(1.into())
//     //         .add()
//     //         .unwrap();
//     // }
//
//     assert_eq!(series.get_bar_count(), 2);
//     let bar2 = series.get_bar(1).unwrap();
//
//     assert_eq!(bar2.volume, DecimalNum::from(4));
//     assert_eq!(bar2.open_price, Some(2.into()));
//     assert_eq!(bar2.close_price, Some(6.into()));
//     assert_eq!(bar2.high_price, Some(6.into()));
//     assert_eq!(bar2.low_price, Some(2.into()));
//     assert_eq!(bar2.time_period, one_day * 3);
//     assert_eq!(bar2.begin_time, now + one_day * 3);
//     assert_eq!(bar2.end_time, now + one_day * 6);
// }

#[derive(Debug)]
pub struct VolumeBarBuilder<'a, T: TrNum + 'static, S: BarSeries<'a, T>> {
    pub(crate) num_factory: Arc<T::Factory>,
    pub(crate) volume_threshold: T,
    pub(crate) bar_series: Option<BarSeriesRef<'a, S>>,

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
impl<'a, S: BarSeries<'a, DoubleNum>> VolumeBarBuilder<'a, DoubleNum, S> {
    pub fn new_with_default_factory(volume_threshold: i64) -> Self {
        Self::new_with_factory(Arc::new(DoubleNumFactory::instance()), volume_threshold)
    }
}

impl<'a, T: TrNum + 'static, S: BarSeries<'a, T>> VolumeBarBuilder<'a, T, S> {
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

    /// 绑定可变引用的 BarSeries
    pub fn bind_to(&mut self, series: &'a mut S) -> &mut Self {
        self.bar_series = Some(BarSeriesRef::Mut(series));
        self
    }

    /// 绑定 Arc<Mutex<S>>
    pub fn bind_shared(&mut self, series: Arc<Mutex<S>>) -> &mut Self {
        self.bar_series = Some(BarSeriesRef::Shared(series));
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
            Some(BarSeriesRef::Mut(series)) => Ok(f(*series)),
            Some(BarSeriesRef::Shared(arc_mutex)) => {
                let mut locked = arc_mutex.lock().map_err(|_| "Failed to lock bar_series")?;
                Ok(f(&mut *locked))
            }
            None => Err("No bound bar_series".to_string()),
        }
    }



}
impl<'a, T: TrNum + 'static, S: BarSeries<'a, T>> BarBuilder<T> for VolumeBarBuilder<'a, T, S>
where
    S: BarSeries<'a, T, Bar = BaseBar<T>>,
{
    type Bar = BaseBar<T>;

    fn time_period(&mut self, time_period: Duration) -> &mut Self {
        self.time_period = Some(self.time_period.unwrap_or(Duration::ZERO) + time_period);
        self
    }

    fn begin_time(&mut self, _: OffsetDateTime) -> &mut Self {
        panic!("VolumeBar can only be built from closePrice");
    }

    fn end_time(&mut self, end_time: OffsetDateTime) -> &mut Self {
        self.end_time = Some(end_time);
        self
    }

    fn open_price(&mut self, _: T) -> &mut Self {
        panic!("VolumeBar can only be built from closePrice");
    }

    fn high_price(&mut self, _: T) -> &mut Self {
        panic!("VolumeBar can only be built from closePrice");
    }

    fn low_price(&mut self, _: T) -> &mut Self {
        panic!("VolumeBar can only be built from closePrice");
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
        let open_price = self.open_price.clone().ok_or("Missing open_price")?;
        let high_price = self.high_price.clone().ok_or("Missing high_price")?;
        let low_price = self.low_price.clone().ok_or("Missing low_price")?;
        let close_price = self.close_price.clone().ok_or("Missing close_price")?;

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
fn test_volume_bar_builder_add() {
    use crate::num::decimal_num::DecimalNum;
    use std::sync::{Arc, Mutex};
    use time::{Duration, OffsetDateTime};

    let factory = VolumeBarBuilderFactory::<DoubleNum>::new(4.into());
    let series = BaseBarSeriesBuilder::<DoubleNum>::default()
        .with_bar_builder_factory(BarBuilderFactories::VolumeBarFactory(factory))
        .build()
        .unwrap();

    // 用 Arc<Mutex> 包装 series
    let shared_series = Arc::new(Mutex::new(series));

    let now = OffsetDateTime::now_utc();
    let one_day = Duration::days(1);

    // 创建 builder 并传入 Arc<Mutex<series>>
    let mut builder = {
        let num_factory = {
            let locked = shared_series.lock().unwrap();
            Arc::clone(&locked.num_factory())
        };
        VolumeBarBuilder {
            num_factory,
            volume_threshold: 4.into(),
            bar_series: Some(Arc::clone(&shared_series)),
        }
    };
    let mut builder = shared_series.lock().unwrap().bar_builder();

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

    // 断言通过锁访问 series 内部状态
    let locked_series = shared_series.lock().unwrap();

    let bar1 = locked_series.get_bar(0).unwrap();

    assert_eq!(bar1.volume, DecimalNum::from(4));
    assert_eq!(bar1.open_price, Some(1.into()));
    assert_eq!(bar1.close_price, Some(4.into()));
    assert_eq!(bar1.high_price, Some(5.into()));
    assert_eq!(bar1.low_price, Some(1.into()));
    assert_eq!(bar1.time_period, one_day * 4);
    assert_eq!(bar1.begin_time, now - one_day); // same as Java behavior
    assert_eq!(bar1.end_time, now + one_day * 3);

    let bar2 = locked_series.get_bar(1).unwrap();

    assert_eq!(bar2.volume, DecimalNum::from(4));
    assert_eq!(bar2.open_price, Some(2.into()));
    assert_eq!(bar2.close_price, Some(6.into()));
    assert_eq!(bar2.high_price, Some(6.into()));
    assert_eq!(bar2.low_price, Some(2.into()));
    assert_eq!(bar2.time_period, one_day * 3);
    assert_eq!(bar2.begin_time, now + one_day * 3);
    assert_eq!(bar2.end_time, now + one_day * 6);
}


