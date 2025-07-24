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
use crate::num::{NumFactory, TrNum};
use std::sync::Arc;
use time::{Duration, OffsetDateTime};

/// Heikin-Ashi Bar 构建器
pub struct HeikinAshiBarBuilder<'a, T: TrNum + 'static, S: BarSeries<'a, T>> {
    time_bar_builder: TimeBarBuilder<'a, T, S>,
    previous_heikin_ashi_open_price: Option<T>,
    previous_heikin_ashi_close_price: Option<T>,
}

impl<'a, T: TrNum + 'static, S: BarSeries<'a, T>> BarBuilder<T> for HeikinAshiBarBuilder<'a, T, S>
where
    S: BarSeries<'a, T, Bar = BaseBar<T>>,
{
    type Bar = BaseBar<T>;

    fn time_period(mut self, time_period: Duration) -> Self {
        self.time_bar_builder.time_period = Some(time_period);
        self
    }

    fn begin_time(mut self, begin_time: OffsetDateTime) -> Self {
        self.time_bar_builder.begin_time = Some(begin_time);
        self
    }

    fn end_time(mut self, end_time: OffsetDateTime) -> Self {
        self.time_bar_builder.end_time = Some(end_time);
        self
    }

    fn open_price(mut self, open_price: T) -> Self {
        self.time_bar_builder.open_price = Some(open_price);
        self
    }

    fn high_price(mut self, high_price: T) -> Self {
        self.time_bar_builder.high_price = Some(high_price);
        self
    }

    fn low_price(mut self, low_price: T) -> Self {
        self.time_bar_builder.low_price = Some(low_price);
        self
    }

    fn close_price(mut self, close_price: T) -> Self {
        self.time_bar_builder.close_price = Some(close_price);
        self
    }

    fn volume(mut self, volume: T) -> Self {
        self.time_bar_builder.volume = Some(volume);
        self
    }

    fn amount(mut self, amount: T) -> Self {
        self.time_bar_builder.amount = Some(amount);
        self
    }

    fn trades(mut self, trades: u64) -> Self {
        self.time_bar_builder.trades = Some(trades);
        self
    }

    fn build(&self) -> Result<Self::Bar, String> {
        let factory = &self.time_bar_builder.num_factory;

        if let (Some(open), Some(high), Some(low), Some(close)) = (
            &self.time_bar_builder.open_price,
            &self.time_bar_builder.high_price,
            &self.time_bar_builder.low_price,
            &self.time_bar_builder.close_price,
        ) {
            if let (Some(prev_open), Some(prev_close)) = (
                &self.previous_heikin_ashi_open_price,
                &self.previous_heikin_ashi_close_price,
            ) {
                let four = factory.num_of_i64(4);
                let two = factory.num_of_i64(2);

                let ha_close = open
                    .add_ref(high)
                    .add_ref(low)
                    .add_ref(close)
                    .divided_by_ref(&four)
                    .map_err(|e| e.to_string())?;

                let ha_open = prev_open
                    .add_ref(prev_close)
                    .divided_by_ref(&two)
                    .map_err(|e| e.to_string())?;

                let ha_high = high.max(&ha_open).max(&ha_close);
                let ha_low = low.min(&ha_open).min(&ha_close);

                return BaseBar::new(
                    self.time_bar_builder
                        .time_period
                        .ok_or("Missing time_period")?,
                    self.time_bar_builder.end_time.ok_or("Missing end_time")?,
                    ha_open,
                    ha_high,
                    ha_low,
                    ha_close,
                    self.time_bar_builder
                        .volume
                        .clone()
                        .ok_or("Missing volume")?,
                    self.time_bar_builder
                        .amount
                        .clone()
                        .unwrap_or_else(|| T::zero()),
                    self.time_bar_builder.trades.unwrap_or(0),
                );
            }
        }

        // fallback
        self.time_bar_builder.build()
    }

    fn add(&mut self) -> Result<(), String> {
        self.time_bar_builder.add()
    }
}

impl<'a, T: TrNum + 'static, S: BarSeries<'a, T>> HeikinAshiBarBuilder<'a, T, S> {
    pub fn new_with_factory(num_factory: Arc<T::Factory>) -> Self {
        Self {
            time_bar_builder: TimeBarBuilder::new_with_factory(num_factory),
            previous_heikin_ashi_open_price: None,
            previous_heikin_ashi_close_price: None,
        }
    }
    pub fn bind_to(mut self, series: &'a mut S) -> Self {
        self.time_bar_builder = self.time_bar_builder.bind_to(series);
        self
    }

    pub fn previous_heikin_ashi_open_price(mut self, previous_open_price: T) -> Self {
        self.previous_heikin_ashi_open_price = Some(previous_open_price);
        self
    }

    pub fn previous_heikin_ashi_close_price(mut self, previous_close_price: T) -> Self {
        self.previous_heikin_ashi_close_price = Some(previous_close_price);
        self
    }
}
