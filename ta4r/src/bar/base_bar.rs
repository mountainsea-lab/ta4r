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

use crate::bar::builder::types::add_to_option;
use crate::bar::types::Bar;
use crate::num::TrNum;
use std::fmt;
use std::hash::{Hash, Hasher};
use time::{Duration, OffsetDateTime};

/// BaseBar 结构体 - 对应 ta4j 的 BaseBar 类
#[derive(Debug, Clone)]
pub struct BaseBar<T: TrNum> {
    /// 时间周期（例如 1 天、15 分钟等）
    pub(crate) time_period: Duration,
    /// Bar 周期的开始时间（UTC）
    pub begin_time: OffsetDateTime,
    /// Bar 周期的结束时间（UTC）
    pub end_time: OffsetDateTime,
    /// Bar 周期的开盘价
    pub open_price: Option<T>,
    /// Bar 周期的最高价
    pub high_price: Option<T>,
    /// Bar 周期的最低价
    pub low_price: Option<T>,
    /// Bar 周期的收盘价
    pub close_price: Option<T>,
    /// Bar 周期的总交易量
    pub volume: T,
    /// Bar 周期的总交易金额
    pub amount: Option<T>,
    /// Bar 周期的交易次数
    pub trades: u64,
}

impl<T: TrNum> BaseBar<T> {
    /// 构造函数，实现与 Java 版本相同的时间计算逻辑
    pub fn new(
        time_period: Duration,
        end_time: OffsetDateTime,
        open_price: Option<T>,
        high_price: Option<T>,
        low_price: Option<T>,
        close_price: Option<T>,
        volume: T,
        amount: Option<T>,
        trades: u64,
    ) -> Result<Self, String> {
        // 计算 begin_time = end_time - time_period
        let begin_time = end_time.checked_sub(time_period).ok_or_else(|| {
            format!(
                "Begin time = end_time - period overflow: {:?} - {:?}",
                end_time, time_period
            )
        })?;

        Ok(BaseBar {
            time_period,
            begin_time,
            end_time,
            open_price,
            high_price,
            low_price,
            close_price,
            volume,
            amount,
            trades,
        })
    }

    /// 带有完整时间参数的构造函数
    pub fn new_with_times(
        time_period: Option<Duration>,
        begin_time: Option<OffsetDateTime>,
        end_time: Option<OffsetDateTime>,
        open_price: Option<T>,
        high_price: Option<T>,
        low_price: Option<T>,
        close_price: Option<T>,
        volume: T,
        amount: Option<T>,
        trades: u64,
    ) -> Result<Self, String> {
        // 实现与 Java 版本相同的复杂时间计算逻辑
        let calculated_time_period = match time_period {
            Some(period) => {
                if let (Some(begin), Some(end)) = (begin_time, end_time) {
                    let calculated = end - begin;
                    if calculated.is_negative() {
                        return Err("End time must be after begin time".to_string());
                    }
                    if period != calculated {
                        return Err("The calculated timePeriod between beginTime and endTime does not match the given timePeriod".to_string());
                    }
                }
                period
            }
            None => {
                if let (Some(begin), Some(end)) = (begin_time, end_time) {
                    let calculated = end - begin;
                    if calculated.is_negative() {
                        return Err("End time must be after begin time".to_string());
                    }
                    calculated
                } else {
                    return Err("Time period cannot be null".to_string());
                }
            }
        };

        let calculated_begin_time = match begin_time {
            Some(begin) => begin,
            None => {
                if let Some(end) = end_time {
                    end.checked_sub(calculated_time_period)
                        .ok_or("Begin time calculation overflow")?
                } else {
                    return Err("Begin time cannot be null".to_string());
                }
            }
        };

        let calculated_end_time = match end_time {
            Some(end) => end,
            None => calculated_begin_time
                .checked_add(calculated_time_period)
                .ok_or("End time calculation overflow")?,
        };

        Ok(BaseBar {
            time_period: calculated_time_period,
            begin_time: calculated_begin_time,
            end_time: calculated_end_time,
            open_price,
            high_price,
            low_price,
            close_price,
            volume,
            amount,
            trades,
        })
    }
}

impl<T: TrNum + 'static> Bar<T> for BaseBar<T> {
    fn get_time_period(&self) -> Duration {
        self.time_period
    }

    fn get_begin_time(&self) -> OffsetDateTime {
        self.begin_time
    }

    fn get_end_time(&self) -> OffsetDateTime {
        self.end_time
    }

    fn get_open_price(&self) -> Option<T> {
        self.open_price.clone()
    }

    fn get_high_price(&self) -> Option<T> {
        self.high_price.clone()
    }

    fn get_low_price(&self) -> Option<T> {
        self.low_price.clone()
    }

    fn get_close_price(&self) -> Option<T> {
        self.close_price.clone()
    }

    fn get_volume(&self) -> T {
        self.volume.clone()
    }

    fn get_amount(&self) -> T {
        self.amount.clone().unwrap_or_else(T::zero)
    }

    fn get_trades(&self) -> u64 {
        self.trades
    }

    /// 添加交易，对应 Java 版本的 addTrade 方法
    fn add_trade(&mut self, trade_volume: T, trade_price: T) {
        self.add_price(trade_price.clone());

        let trade_amount = trade_volume.multiplied_by(&trade_price);
        self.volume = self.volume.plus(&trade_volume);
        self.amount = add_to_option(&self.amount, trade_amount);

        self.trades += 1;
    }

    /// 添加价格，对应 Java 版本的 addPrice 方法
    fn add_price(&mut self, price: T) {
        // 设置开盘价（只设置一次）
        if self.open_price.is_none() {
            self.open_price = Some(price.clone());
        }

        // 更新收盘价（每次都更新）
        self.close_price = Some(price.clone());

        // 更新最高价
        match &self.high_price {
            Some(high) if high.lt(&price) => {
                self.high_price = Some(price.clone());
            }
            None => {
                self.high_price = Some(price.clone());
            }
            _ => {} // 当前 high >= price，不更新
        }

        // 更新最低价
        match &self.low_price {
            Some(low) if low.gt(&price) => {
                self.low_price = Some(price);
            }
            None => {
                self.low_price = Some(price);
            }
            _ => {} // 当前 low <= price，不更新
        }
    }
}
// 实现 Display trait，对应 Java 的 toString 方法
impl<T: TrNum> fmt::Display for BaseBar<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{end time: {:?}, close price: {:?}, open price: {:?}, low price: {:?}, high price: {:?}, volume: {}}}",
            self.end_time,
            self.close_price,
            self.open_price,
            self.low_price,
            self.high_price,
            self.volume
        )
    }
}

// 实现 PartialEq，对应 Java 的 equals 方法
impl<T: TrNum> PartialEq for BaseBar<T> {
    fn eq(&self, other: &Self) -> bool {
        self.begin_time == other.begin_time
            && self.end_time == other.end_time
            && self.time_period == other.time_period
            && self.open_price == other.open_price
            && self.high_price == other.high_price
            && self.low_price == other.low_price
            && self.close_price == other.close_price
            && self.volume == other.volume
            && self.amount == other.amount
            && self.trades == other.trades
    }
}

impl<T: TrNum> Eq for BaseBar<T> {}

// 实现 Hash，对应 Java 的 hashCode 方法
impl<T: TrNum + Hash> Hash for BaseBar<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.begin_time.hash(state);
        self.end_time.hash(state);
        self.time_period.hash(state);
        self.open_price.hash(state);
        self.high_price.hash(state);
        self.low_price.hash(state);
        self.close_price.hash(state);
        self.volume.hash(state);
        self.amount.hash(state);
        self.trades.hash(state);
    }
}
