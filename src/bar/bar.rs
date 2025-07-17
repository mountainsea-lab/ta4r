use std::time::Duration;
use chrono::{DateTime, Local, Utc};
use serde::{Deserialize, Serialize};
use crate::num::TrNum;

// Bar 结构体，使用泛型 T 替代接口
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bar<T: TrNum> {
    time_period: Duration,
    begin_time: DateTime<Utc>,
    end_time: DateTime<Utc>,
    open_price: Option<T>,
    high_price: Option<T>,
    low_price: Option<T>,
    close_price: Option<T>,
    volume: T,
    amount: T,
    trades: u64,
}

impl<T: TrNum> Bar<T> {
    // 构造函数
    pub fn new(
        time_period: Duration,
        end_time: DateTime<Utc>,
        open_price: Option<T>,
        high_price: Option<T>,
        low_price: Option<T>,
        close_price: Option<T>,
        volume: T,
        amount: T,
        trades: u64,
    ) -> Self {
        let begin_time = end_time - chrono::Duration::from_std(time_period).unwrap();

        Self {
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
        }
    }

    // 对应 Java 接口中的 getter 方法
    pub fn get_time_period(&self) -> Duration {
        self.time_period
    }

    pub fn get_begin_time(&self) -> DateTime<Utc> {
        self.begin_time
    }

    pub fn get_end_time(&self) -> DateTime<Utc> {
        self.end_time
    }

    pub fn get_open_price(&self) -> Option<&T> {
        self.open_price.as_ref()
    }

    pub fn get_high_price(&self) -> Option<&T> {
        self.high_price.as_ref()
    }

    pub fn get_low_price(&self) -> Option<&T> {
        self.low_price.as_ref()
    }

    pub fn get_close_price(&self) -> Option<&T> {
        self.close_price.as_ref()
    }

    pub fn get_volume(&self) -> &T {
        &self.volume
    }

    pub fn get_amount(&self) -> &T {
        &self.amount
    }

    pub fn get_trades(&self) -> u64 {
        self.trades
    }

    // 对应 inPeriod 方法
    pub fn in_period(&self, timestamp: DateTime<Utc>) -> bool {
        timestamp >= self.begin_time && timestamp < self.end_time
    }

    // 时区转换方法
    pub fn get_zoned_begin_time(&self) -> DateTime<Utc> {
        self.begin_time
    }

    pub fn get_zoned_end_time(&self) -> DateTime<Utc> {
        self.end_time
    }

    pub fn get_system_zoned_begin_time(&self) -> DateTime<Local> {
        self.begin_time.with_timezone(&Local)
    }

    pub fn get_system_zoned_end_time(&self) -> DateTime<Local> {
        self.end_time.with_timezone(&Local)
    }

    // 格式化方法
    pub fn get_date_name(&self) -> String {
        self.get_system_zoned_end_time().to_rfc3339()
    }

    pub fn get_simple_date_name(&self) -> String {
        self.get_system_zoned_end_time().format("%Y-%m-%dT%H:%M:%S").to_string()
    }

    // 市场趋势判断方法
    pub fn is_bearish(&self) -> bool {
        match (&self.open_price, &self.close_price) {
            (Some(open), Some(close)) => close.is_less_than(open),
            _ => false,
        }
    }

    pub fn is_bullish(&self) -> bool {
        match (&self.open_price, &self.close_price) {
            (Some(open), Some(close)) => open.is_less_than(close),
            _ => false,
        }
    }

    // 添加交易数据的方法
    pub fn add_trade(&mut self, trade_volume: T, trade_price: T) {
        self.add_price(trade_price.clone());
        self.volume = self.volume.clone() + trade_volume.clone();
        self.amount = self.amount.clone() + (trade_volume * trade_price);
        self.trades += 1;
    }

    pub fn add_price(&mut self, price: T) {
        if self.open_price.is_none() {
            self.open_price = Some(price.clone());
        }

        self.close_price = Some(price.clone());

        match &self.high_price {
            Some(high) => {
                if high.is_less_than(&price) {
                    self.high_price = Some(price.clone());
                }
            }
            None => self.high_price = Some(price.clone()),
        }

        match &self.low_price {
            Some(low) => {
                if price.is_less_than(low) {
                    self.low_price = Some(price);
                }
            }
            None => self.low_price = Some(price),
        }
    }
}