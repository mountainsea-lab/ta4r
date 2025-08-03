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
use crate::bar::builder::types::add_to_option;
use crate::bar::types::{BarBuilder, BarSeries};
use crate::num::double_num::DoubleNum;
use crate::num::double_num_factory::DoubleNumFactory;
use crate::num::{NumFactory, TrNum};
use std::sync::Arc;
use time::{Duration, OffsetDateTime};

#[derive(Debug)]
pub struct VolumeBarBuilder<'a, T: TrNum + 'static, S: BarSeries<'a, T>> {
    num_factory: Arc<T::Factory>,
    volume_threshold: T,
    bar_series: Option<&'a mut S>,

    time_period: Option<Duration>,
    end_time: Option<OffsetDateTime>,
    open_price: Option<T>,
    high_price: Option<T>,
    low_price: Option<T>,
    close_price: Option<T>,
    volume: T,
    amount: Option<T>,
    trades: u64,
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

    pub fn bind_to(mut self, bar_series: &'a mut S) -> Self {
        self.bar_series = Some(bar_series);
        self
    }

    fn reset(&mut self) {
        self.time_period = None;
        self.end_time = None;
        self.open_price = None;
        self.high_price = Some(T::zero());
        self.low_price = T::from_i64(i64::MAX);
        self.close_price = None;
        self.volume = T::zero();
        self.amount = None;
        self.trades = 0;
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
            if let Some(ref mut series) = self.bar_series {
                series.add_bar(bar);
            }

            self.volume = volume_remainder;
            self.reset();
        }

        Ok(())
    }
}
