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
use crate::aggregator::{BarAggregator, BarSeriesAggregator};
use crate::bar::base_bar::BaseBar;
use crate::bar::base_bar_series::BaseBarSeries;
use crate::bar::base_bar_series_builder::BaseBarSeriesBuilder;
use crate::bar::types::{BarSeries, BarSeriesBuilder};
use crate::num::TrNum;
use std::marker::PhantomData;

pub struct BaseBarSeriesAggregator<T, BA>
where
    T: TrNum + 'static,
    BA: BarAggregator<T, Bar = BaseBar<T>>,
{
    bar_aggregator: BA,
    _marker: PhantomData<T>, // 添加这个字段来“使用”T
}

impl<T, BA> BaseBarSeriesAggregator<T, BA>
where
    T: TrNum + 'static,
    BA: BarAggregator<T, Bar = BaseBar<T>>,
{
    pub fn new(bar_aggregator: BA) -> Self {
        Self {
            bar_aggregator,
            _marker: PhantomData,
        }
    }
}

impl<T, BA> BarSeriesAggregator<T> for BaseBarSeriesAggregator<T, BA>
where
    T: TrNum + 'static,
    BA: BarAggregator<T, Bar = BaseBar<T>>,
{
    type Bar = BaseBar<T>;
    type Series = BaseBarSeries<T>;

    fn aggregate_with_name(
        &self,
        series: &BaseBarSeries<T>,
        aggregated_series_name: &str,
    ) -> Result<Self::Series, String> {
        let bars = series.get_bar_data();
        let aggregated_bars = self.bar_aggregator.aggregate(bars);

        BaseBarSeriesBuilder::<T>::default()
            .with_name(aggregated_series_name.to_string())
            .with_bars(aggregated_bars?)
            .build()
    }
}
