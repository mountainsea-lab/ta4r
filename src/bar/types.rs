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

use crate::num::{NumFactory, TrNum};
use std::sync::{Arc, Mutex};
use time::{Duration, OffsetDateTime};

// Bar trait - 对应 ta4j 的 Bar 接口
pub trait Bar<T: TrNum + 'static> {
    fn get_time_period(&self) -> Duration;
    fn get_begin_time(&self) -> OffsetDateTime;
    fn get_end_time(&self) -> OffsetDateTime;
    fn get_open_price(&self) -> Option<T>;
    fn get_high_price(&self) -> Option<T>;
    fn get_low_price(&self) -> Option<T>;
    fn get_close_price(&self) -> Option<T>;
    fn get_volume(&self) -> T;
    fn get_amount(&self) -> T;
    fn get_trades(&self) -> u64;
    fn add_trade(&mut self, trade_volume: T, trade_price: T);
    fn add_price(&mut self, price: T);
    fn in_period(&self, time: OffsetDateTime) -> bool {
        time >= self.get_begin_time() && time <= self.get_end_time()
    }
}

// BarBuilder trait - 使用关联类型避免动态分发
pub trait BarBuilder<T: TrNum + 'static> {
    type Bar: Bar<T>;

    fn time_period(&mut self, period: Duration) -> &mut Self;
    fn begin_time(&mut self, time: OffsetDateTime) -> &mut Self;
    fn end_time(&mut self, time: OffsetDateTime) -> &mut Self;
    fn open_price(&mut self, price: T) -> &mut Self;
    fn high_price(&mut self, price: T) -> &mut Self;
    fn low_price(&mut self, price: T) -> &mut Self;
    fn close_price(&mut self, price: T) -> &mut Self;
    fn volume(&mut self, volume: T) -> &mut Self;
    fn amount(&mut self, amount: T) -> &mut Self;
    fn trades(&mut self, trades: u64) -> &mut Self;
    fn build(&self) -> Result<Self::Bar, String>;
    /**
     * Builds bar with {@link #build()} and adds it to series
     */
    fn add(&mut self) -> Result<(), String>;
}

// BarBuilderFactory trait - 使用关联类型
pub trait BarBuilderFactory<T: TrNum + 'static> {
    type Series: for<'a> BarSeries<'a, T>;
    type Builder<'a>: BarBuilder<T>
    where
        Self::Series: 'a;
    fn create_bar_builder<'a>(&self, series: &'a mut Self::Series) -> Self::Builder<'a>;
    fn create_bar_builder_shared(
        &self,
        shared_series: Arc<Mutex<Self::Series>>,
    ) -> Self::Builder<'static>
    where
        Self::Series: 'static;
}

// BarSeries trait - 对应 ta4j 的 BarSeries 接口
pub trait BarSeries<'a, T: TrNum + 'static> {
    type Bar: Bar<T>;
    // GAT，Builder 关联类型带生命周期参数 'b
    type Builder<'b>: BarBuilder<T, Bar = Self::Bar>
    where
        Self: 'b;
    type NumFactory: NumFactory<T>;
    type SubSeries;

    /// 返回生成此 BarSeries 中可用数字的工厂
    fn num_factory(&self) -> Arc<Self::NumFactory>;
    ///  BarSeries 中可用数字的工厂便捷访问
    fn factory_ref(&self) -> &T::Factory;

    /// 返回生成兼容 bar 的构建器，生命周期和 self 绑定
    fn bar_builder(&mut self) -> Self::Builder<'_>;

    /// 基于 Arc<Mutex<Self>> 返回一个构建器，适用于多线程共享调用 create_bar_builder_shared
    fn bar_builder_shared(&mut self, shared_series: Arc<Mutex<Self>>) -> Self::Builder<'static>
    where
        Self: Sized + 'static;

    /// 返回序列的名称
    fn get_name(&self) -> &str;

    /// 获取指定索引的 bar
    ///
    /// 由于 setMaximumBarCount 的存在，给定的索引可能在第一个索引范围内返回相同的 bar
    /// 例如：如果你用 30 个 bar 填充 BarSeries，然后应用 maximumBarCount 为 10，
    /// 前 20 个 bar 将从 BarSeries 中移除。索引从 0 到 29 仍然存在，
    /// 但从 0 到 20 返回相同的 bar。剩余的 9 个 bar 从索引 21 开始返回。
    fn get_bar(&self, index: usize) -> Option<&Self::Bar>;

    /// 返回序列的第一个 bar
    fn get_first_bar(&self) -> Option<&Self::Bar> {
        self.get_begin_index()
            .and_then(|begin_index| self.get_bar(begin_index))
    }

    fn get_last_bar(&self) -> Option<&Self::Bar> {
        self.get_end_index()
            .and_then(|end_index| self.get_bar(end_index))
    }

    /// 返回序列中 bar 的数量
    fn get_bar_count(&self) -> usize;

    /// 如果序列为空则返回 true，否则返回 false
    fn is_empty(&self) -> bool {
        self.get_bar_count() == 0
    }

    /// 返回原始 bar 数据
    ///
    /// 警告：此方法应谨慎使用！
    /// 返回用于内部存储 Bar 的当前列表对象。它可能是：
    /// - 如果设置了 maximumBarCount，则为缩短的 bar 列表
    /// - 如果是受约束的 bar 序列，则为扩展的 bar 列表
    fn get_bar_data(&self) -> &[Self::Bar];

    /// 返回序列的开始索引
    fn get_begin_index(&self) -> Option<usize>;

    /// 返回序列的结束索引
    fn get_end_index(&self) -> Option<usize>;

    /// 返回序列周期的描述（例如 "from 2014-01-21T12:00:00Z to 2014-01-21T12:15:00Z"）
    /// 时间为 UTC
    fn get_series_period_description(&self) -> String {
        if !self.get_bar_data().is_empty() {
            if let (Some(first_bar), Some(last_bar)) = (self.get_first_bar(), self.get_last_bar()) {
                let first_time = first_bar.get_end_time();
                let last_time = last_bar.get_end_time();
                return format!("{:?} - {:?}", first_time, last_time);
            }
        }
        String::new()
    }

    /// 返回系统默认时区中的序列周期描述
    fn get_series_period_description_in_system_time_zone(&self) -> String {
        // 在 Rust 中，OffsetDateTime 默认使用系统时区
        self.get_series_period_description()
    }

    /// 返回最大 bar 数量
    fn get_maximum_bar_count(&self) -> usize;

    /// 设置序列中将保留的最大 bar 数量
    ///
    /// 如果向序列添加新 bar 使得 bar 数量超过最大 bar 计数，
    /// 则序列中的第一个 bar 将自动移除，确保不超过最大 bar 计数。
    /// bar 序列的索引不会改变。
    fn set_maximum_bar_count(&mut self, maximum_bar_count: usize) -> Result<(), String>;

    /// 返回已移除的 bar 数量
    fn get_removed_bars_count(&self) -> usize;

    /// 在序列末尾添加 bar
    ///
    /// beginIndex 如果尚未初始化则设置为 0
    /// endIndex 如果尚未初始化则设置为 0，或者如果它匹配序列的末尾则递增
    /// 超出的 bar 将被移除
    fn add_bar(&mut self, bar: Self::Bar) {
        let _ = self.add_bar_with_replace(bar, false);
    }

    /// 在序列末尾添加 bar
    ///
    /// replace: true 表示替换最新的 bar。一些交易所在相应期间内
    /// 连续提供新的 bar 数据，例如在 1 分钟持续时间内每 1 秒
    fn add_bar_with_replace(&mut self, bar: Self::Bar, replace: bool) -> Result<(), String>;

    /// 添加交易并更新最后一个 bar 的收盘价
    fn add_trade_with_numbers(&mut self, trade_volume: i64, trade_price: i64) {
        let volume = self.num_factory().num_of_i64(trade_volume);
        let price = self.num_factory().num_of_i64(trade_price);
        self.add_trade(volume, price);
    }

    /// 添加交易并更新最后一个 bar 的收盘价
    fn add_trade(&mut self, trade_volume: T, trade_price: T);

    /// 更新最后一个 bar 的收盘价。开盘价、最高价和最低价也会根据需要更新
    fn add_price(&mut self, price: T);

    /// 更新最后一个 bar 的收盘价（从数字类型）
    fn add_price_with_number(&mut self, price: i64) {
        let num_price = self.num_factory().num_of_i64(price);
        self.add_price(num_price);
    }

    /// 返回一个新的 BarSeries 实例（"子序列"），它是此 BarSeries 实例的子集
    ///
    /// 它包含此实例的 startIndex（包含）和 endIndex（不包含）之间所有 Bar 的副本。
    /// 此实例及其子序列的索引可能不同，即子序列的索引 0 将是此实例的 startIndex。
    /// 如果 startIndex < this.seriesBeginIndex，则子序列将从此实例的第一个可用 bar 开始。
    /// 如果 endIndex > this.seriesEndIndex，则子序列将在此实例的最后一个可用 bar 结束。
    fn get_sub_series(
        &self,
        start_index: usize,
        end_index: usize,
    ) -> Result<Self::SubSeries, String>;
}

// BarSeriesBuilder trait - 对应 ta4j 的 BarSeriesBuilder 接口
pub trait BarSeriesBuilder<T: TrNum + 'static> {
    type BarSeries: for<'a> BarSeries<'a, T>;
    fn build(self) -> Result<Self::BarSeries, String>;
}

// BarAggregator trait - 对应 ta4j 的 BarAggregator 接口
pub trait BarAggregator<T: TrNum + 'static, B: Bar<T>> {
    fn aggregate(&self, bars: &[B]) -> Vec<B>;
}
