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

use crate::bar::base_bar_series::BaseBarSeries;
use crate::bar::base_bar_series_builder::BaseBarSeriesBuilder;
use crate::bar::builder::types::BarBuilderFactories;
use crate::bar::types::{BarBuilder, BarSeries, BarSeriesBuilder};
use crate::num::TrNum;
use crate::num::decimal_num::DecimalNum;
use std::sync::Arc;
use time::{Duration, OffsetDateTime};

static TEST_START_TIME: OffsetDateTime = OffsetDateTime::UNIX_EPOCH;

pub struct MockBarSeriesBuilder<T: TrNum> {
    inner: BaseBarSeriesBuilder<T>,
    data: Option<Vec<f64>>,
    default_data: bool,
}

impl<T: TrNum> Default for MockBarSeriesBuilder<T> {
    fn default() -> Self {
        Self {
            inner: BaseBarSeriesBuilder::default(),
            data: None,
            default_data: false,
        }
    }
}

impl MockBarSeriesBuilder<DecimalNum> {
    pub fn new() -> Self {
        Self {
            inner: BaseBarSeriesBuilder::new(),
            data: None,
            default_data: false,
        }
    }
}

impl<T> MockBarSeriesBuilder<T>
where
    T: TrNum + 'static,
{
    pub fn with_num_factory(mut self, factory: Arc<T::Factory>) -> Self {
        self.inner = self.inner.with_num_factory(factory);
        self
    }

    pub fn with_data(mut self, data: Vec<f64>) -> Self {
        self.data = Some(data);
        self
    }

    pub fn with_data_slice(mut self, data: &[f64]) -> Self {
        self.data = Some(data.to_vec());
        self
    }

    pub fn with_default_data(mut self) -> Self {
        self.default_data = true;
        self
    }

    pub fn build(mut self) -> BaseBarSeries<T> {
        // 强制使用 MockBarBuilderFactory，如果需要
        if self.inner.bar_builder_factory.is_none() {
            self.inner = self
                .inner
                .with_bar_builder_factory(BarBuilderFactories::MockBarFactory(Default::default()));
        }

        let mut series = self.inner.build().expect("Failed to build BaseBarSeries");

        if let Some(data) = self.data.take() {
            Self::doubles_to_bars(&mut series, data);
        }

        if self.default_data {
            Self::arbitrary_bars(&mut series);
        }

        series
    }

    fn doubles_to_bars(series: &mut BaseBarSeries<T>, data: Vec<f64>) {
        let max_bars = data.len() + 1;
        for (i, &value) in data.iter().enumerate() {
            // 生成一个固定时区偏移为 UTC 的时间点，举例：1970-01-01T00:00:00Z - x分钟
            let end_time = TEST_START_TIME - Duration::minutes((max_bars - i) as i64);

            // 用 builder 构造 bar，注意处理 Result
            let res = series
                .bar_builder()
                .end_time(end_time)
                .close_price(T::from_f64(value).unwrap_or_else(|| T::zero()))
                .open_price(T::zero())
                .add();

            if let Err(e) = res {
                panic!("Failed to add bar: {}", e);
            }
        }
    }

    fn arbitrary_bars(series: &mut BaseBarSeries<T>) {
        for i in 0..5000u32 {
            let f = i as f64;

            let end_time = TEST_START_TIME - Duration::minutes((5001 - i) as i64);

            let res = series
                .bar_builder()
                .end_time(end_time)
                .open_price(T::from_f64(f).unwrap_or_else(|| T::zero()))
                .close_price(T::from_f64(f + 1.0).unwrap_or_else(|| T::zero()))
                .high_price(T::from_f64(f + 2.0).unwrap_or_else(|| T::zero()))
                .low_price(T::from_f64(f + 3.0).unwrap_or_else(|| T::zero()))
                .volume(T::from_f64(f + 4.0).unwrap_or_else(|| T::zero()))
                .amount(T::from_f64(f + 5.0).unwrap_or_else(|| T::zero()))
                .trades((f + 6.0) as u64)
                .add();

            if let Err(e) = res {
                panic!("Failed to add arbitrary bar: {}", e);
            }
        }
    }
}
