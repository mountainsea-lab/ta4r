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
use crate::bar::builder::time_bar_builder::TimeBarBuilder;
use crate::bar::types::{BarBuilder, BarSeries};
use crate::num::TrNum;
use std::sync::{Arc, Mutex};
use time::{Duration, OffsetDateTime};

static NEXT_BAR_COUNT: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

#[derive(Debug)]
pub struct MockBarBuilder<T: TrNum + 'static, S: BarSeries<T>> {
    time_bar_builder: TimeBarBuilder<T, S>,
    period_set: bool,
    end_time_set: bool,
    time_period: Option<Duration>,
    begin_time: OffsetDateTime,
}

impl<T, S> MockBarBuilder<T, S>
where
    T: TrNum + 'static,
    S: BarSeries<T>,
{
    pub fn new_with_factory(num_factory: Arc<T::Factory>) -> Self {
        Self {
            time_bar_builder: TimeBarBuilder::new_with_factory(num_factory),
            period_set: false,
            end_time_set: false,
            time_period: None,
            begin_time: OffsetDateTime::UNIX_EPOCH,
        }
    }

    pub fn bind_to(mut self, series: &mut S) -> Self {
        self.time_bar_builder = self.time_bar_builder.bind_to(series);
        self
    }

    pub fn bind_shared(mut self, series: Arc<Mutex<S>>) -> Self {
        self.time_bar_builder = self.time_bar_builder.bind_shared(series);
        self
    }
}

impl<T: TrNum + 'static, S: BarSeries<T>> BarBuilder<T> for MockBarBuilder<T, S>
where
    S: BarSeries<T, Bar = BaseBar<T>>,
{
    type Bar = BaseBar<T>;

    fn time_period(&mut self, time_period: Duration) -> &mut Self {
        self.period_set = true;
        self.time_period = Some(time_period);
        self.time_bar_builder.time_period = Some(time_period);
        self
    }

    fn begin_time(&mut self, begin_time: OffsetDateTime) -> &mut Self {
        self.begin_time = begin_time;
        self.time_bar_builder.begin_time = Some(begin_time);
        self
    }

    fn end_time(&mut self, end_time: OffsetDateTime) -> &mut Self {
        self.end_time_set = true;
        self.time_bar_builder.end_time = Some(end_time);
        self
    }

    fn open_price(&mut self, open_price: T) -> &mut Self {
        self.time_bar_builder.open_price = Some(open_price);
        self
    }

    fn high_price(&mut self, high_price: T) -> &mut Self {
        self.time_bar_builder.high_price = Some(high_price);
        self
    }

    fn low_price(&mut self, low_price: T) -> &mut Self {
        self.time_bar_builder.low_price = Some(low_price);
        self
    }

    fn close_price(&mut self, close_price: T) -> &mut Self {
        self.time_bar_builder.close_price = Some(close_price);
        self
    }

    fn volume(&mut self, volume: T) -> &mut Self {
        self.time_bar_builder.volume = Some(volume);
        self
    }

    fn amount(&mut self, amount: T) -> &mut Self {
        self.time_bar_builder.amount = Some(amount);
        self
    }

    fn trades(&mut self, trades: u64) -> &mut Self {
        self.time_bar_builder.trades = Some(trades);
        self
    }

    fn build(&self) -> Result<Self::Bar, String> {
        let mut inner = self.time_bar_builder.clone_without_series(); // TimeBarBuilder needs to be Clone
        let time_period = self.time_period.unwrap_or_else(|| {
            let default_period = Duration::days(1);
            inner.time_period(default_period);
            default_period
        });

        if !self.end_time_set {
            let count = NEXT_BAR_COUNT.fetch_add(1, std::sync::atomic::Ordering::SeqCst) + 1;
            let end_time = self.begin_time + time_period * (count as i32);
            inner.end_time(end_time);
        }

        inner.build()
    }

    fn add(&mut self) -> Result<(), String> {
        self.time_bar_builder.add() // delegate to inner TimeBarBuilder's add()
    }
}
