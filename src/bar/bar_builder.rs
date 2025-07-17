use std::time::Duration;
use chrono::{DateTime, Utc};
use crate::bar::bar::Bar;
use crate::num::TrNum;

// Builder 模式实现
pub struct BarBuilder<T: TrNum> {
    time_period: Option<Duration>,
    end_time: Option<DateTime<Utc>>,
    open_price: Option<T>,
    high_price: Option<T>,
    low_price: Option<T>,
    close_price: Option<T>,
    volume: Option<T>,
    amount: Option<T>,
    trades: u64,
}

impl<T: TrNum> BarBuilder<T> {
    pub fn new() -> Self {
        Self {
            time_period: None,
            end_time: None,
            open_price: None,
            high_price: None,
            low_price: None,
            close_price: None,
            volume: None,
            amount: None,
            trades: 0,
        }
    }

    pub fn time_period(mut self, period: Duration) -> Self {
        self.time_period = Some(period);
        self
    }

    pub fn end_time(mut self, time: DateTime<Utc>) -> Self {
        self.end_time = Some(time);
        self
    }

    pub fn open_price(mut self, price: T) -> Self {
        self.open_price = Some(price);
        self
    }

    pub fn high_price(mut self, price: T) -> Self {
        self.high_price = Some(price);
        self
    }

    pub fn low_price(mut self, price: T) -> Self {
        self.low_price = Some(price);
        self
    }

    pub fn close_price(mut self, price: T) -> Self {
        self.close_price = Some(price);
        self
    }

    pub fn volume(mut self, vol: T) -> Self {
        self.volume = Some(vol);
        self
    }

    pub fn amount(mut self, amt: T) -> Self {
        self.amount = Some(amt);
        self
    }

    pub fn trades(mut self, count: u64) -> Self {
        self.trades = count;
        self
    }

    pub fn build(self) -> Result<Bar<T>, &'static str> {
        let time_period = self.time_period.ok_or("time_period is required")?;
        let end_time = self.end_time.ok_or("end_time is required")?;
        let volume = self.volume.unwrap_or_else(T::zero);
        let amount = self.amount.unwrap_or_else(T::zero);

        Ok(Bar::new(
            time_period,
            end_time,
            self.open_price,
            self.high_price,
            self.low_price,
            self.close_price,
            volume,
            amount,
            self.trades,
        ))
    }
}

// 使用示例
#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use std::time::Duration;

    #[test]
    fn test_bar_creation() {
        let bar = BarBuilder::<f64>::new()
            .time_period(Duration::from_secs(3600)) // 1小时
            .end_time(Utc::now())
            .open_price(100.0)
            .high_price(105.0)
            .low_price(98.0)
            .close_price(103.0)
            .volume(1000.0)
            .build()
            .unwrap();

        assert_eq!(bar.get_open_price(), Some(&100.0));
        assert_eq!(bar.get_close_price(), Some(&103.0));
        assert!(bar.is_bullish());
        assert!(!bar.is_bearish());
    }
}