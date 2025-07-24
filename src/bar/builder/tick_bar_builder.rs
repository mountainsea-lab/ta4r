use crate::bar::base_bar::BaseBar;
use crate::bar::types::{BarBuilder, BarSeries};
use crate::num::TrNum;
use std::sync::Arc;
use time::{Duration, OffsetDateTime};

/// TickBarBuilder 结构体 - 使用泛型参数避免动态分发
#[derive(Debug)]
pub struct TickBarBuilder<'a, T: TrNum + 'static, S: BarSeries<'a, T>> {
    /// 数值工厂
    num_factory: Arc<T::Factory>,
    /// 触发新 Bar 的交易次数阈值
    tick_count: u64,
    /// 当前已处理的交易次数
    passed_ticks_count: u64,
    /// 绑定的 BarSeries（可选，使用泛型参数）
    bar_series: Option<&'a mut S>,
    // Bar 构建字段
    time_period: Option<Duration>,
    end_time: Option<OffsetDateTime>,
    open_price: Option<T>,
    high_price: Option<T>,
    low_price: Option<T>,
    close_price: Option<T>,
    volume: Option<T>,
    amount: Option<T>,
    trades: Option<u64>,
}

impl<'a, T: TrNum + 'static, S: BarSeries<'a, T>> TickBarBuilder<'a, T, S> {
    /// 创建新的 TickBarBuilder，使用默认数值工厂
    pub fn new(tick_count: u64) -> Self
    where
        T::Factory: Default,
    {
        Self::new_with_factory(Arc::new(T::Factory::default()), tick_count)
    }

    /// 创建新的 TickBarBuilder，指定数值工厂
    pub fn new_with_factory(num_factory: Arc<T::Factory>, tick_count: u64) -> Self {
        Self {
            num_factory,
            tick_count,
            passed_ticks_count: 0,
            bar_series: None,
            time_period: None,
            end_time: None,
            open_price: None,
            high_price: None,
            low_price: None,
            close_price: None,
            volume: Some(T::zero()),
            amount: Some(T::zero()),
            trades: Some(0u64),
        }
    }

    /// 绑定到 BarSeries，返回新的类型化构建器
    pub fn bind_to(self, bar_series: &'a mut S) -> TickBarBuilder<'a, T, S> {
        TickBarBuilder {
            num_factory: self.num_factory,
            tick_count: self.tick_count,
            passed_ticks_count: self.passed_ticks_count,
            bar_series: Some(bar_series),
            time_period: self.time_period,
            end_time: self.end_time,
            open_price: self.open_price,
            high_price: self.high_price,
            low_price: self.low_price,
            close_price: self.close_price,
            volume: self.volume,
            amount: self.amount,
            trades: self.trades,
        }
    }

    /// 重置构建器状态
    fn reset(&mut self) {
        self.time_period = None;
        self.open_price = None;
        self.high_price = Some(T::zero());
        self.low_price = T::from_i64(i64::MAX);
        self.close_price = None;
        self.volume = Some(T::zero());
        self.amount = Some(T::zero());
        self.trades = Some(0u64);
    }
}

impl<'a, T: TrNum + 'static, S: BarSeries<'a, T>> BarBuilder<T> for TickBarBuilder<'a, T, S>
where
    S: BarSeries<'a, T, Bar = BaseBar<T>>,
{
    type Bar = BaseBar<T>;
    fn time_period(mut self, time_period: Duration) -> Self {
        self.time_period = match self.time_period {
            Some(existing) => Some(existing + time_period),
            None => Some(time_period),
        };
        self
    }

    fn begin_time(self, _time: OffsetDateTime) -> Self {
        panic!("TickBar can only be built from closePrice");
    }

    fn end_time(mut self, end_time: OffsetDateTime) -> Self {
        self.end_time = Some(end_time);
        self
    }

    fn open_price(self, _open_price: T) -> Self {
        panic!("TickBar can only be built from closePrice");
    }

    fn high_price(self, _high_price: T) -> Self {
        panic!("TickBar can only be built from closePrice");
    }

    fn low_price(self, _low_price: T) -> Self {
        panic!("TickBar can only be built from closePrice");
    }

    fn close_price(mut self, tick_price: T) -> Self {
        // move 一次 tick_price，拆成多个变量避免多次 clone
        let price = tick_price;

        self.close_price = Some(price.clone());

        if self.open_price.is_none() {
            self.open_price = Some(price.clone());
        }

        match &mut self.high_price {
            Some(high) => {
                if price > *high {
                    *high = price.clone();
                }
            }
            None => {
                self.high_price = Some(price.clone());
            }
        }

        match &mut self.low_price {
            Some(low) => {
                if price < *low {
                    *low = price;
                }
            }
            None => {
                self.low_price = Some(price);
            }
        }

        self
    }

    fn volume(mut self, volume: T) -> Self {
        self.volume = Some(self.volume.map_or(volume.clone(), |v| v + volume));
        self
    }

    fn amount(mut self, amount: T) -> Self {
        self.amount = Some(amount);
        self
    }

    fn trades(mut self, trades: u64) -> Self {
        self.trades = Some(trades);
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

        let volume = self.volume.clone().unwrap_or_else(|| T::zero());
        let amount = self.amount.clone().unwrap_or_else(|| T::zero());
        let trades = self.trades.unwrap_or(0);

        BaseBar::new(
            time_period,
            end_time,
            open_price,
            high_price,
            low_price,
            close_price,
            volume,
            amount,
            trades,
        )
    }

    fn add(&mut self) -> Result<(), String> {
        self.passed_ticks_count += 1;

        if self.passed_ticks_count % self.tick_count == 0 {
            let bar = self.build()?;
            if let Some(ref mut series) = self.bar_series {
                series.add_bar(bar);
            }
            self.reset();
        }

        Ok(())
    }
}
