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
pub mod base_bar_series_aggregator;
pub mod duration_bar_aggregator;
mod heikin_ashi_bar_aggregator;
pub mod types;

use crate::bar::types::{Bar, BarSeries};
use crate::num::TrNum;

pub trait BarAggregator<T: TrNum + 'static> {
    type Bar: Bar<T>;

    /// 将输入的一批 Bar 聚合为新的 Bar 序列
    /// 传入是对输入 Bar 的借用切片
    fn aggregate(&self, bars: &[Self::Bar]) -> Result<Vec<Self::Bar>, String>;
}

pub trait BarSeriesAggregator<T: TrNum + 'static> {
    type Bar: Bar<T>;
    type Series: for<'a> BarSeries<'a, T, Bar = Self::Bar>;

    /// 使用默认名称聚合整个 BarSeries，返回新的 BarSeries
    fn aggregate(&self, series: &Self::Series) -> Result<Self::Series, String> {
        let default_name = series.get_name();
        self.aggregate_with_name(series, default_name)
    }

    /// 聚合整个 BarSeries，返回指定名称的新 BarSeries
    fn aggregate_with_name(
        &self,
        series: &Self::Series,
        aggregated_series_name: &str,
    ) -> Result<Self::Series, String>;
}
