use crate::bar::base_bar::BaseBar;
use crate::bar::types::{BarBuilder, BarSeries};
use crate::num::TrNum;
use crate::num::double_num::DoubleNum;
use crate::num::double_num_factory::DoubleNumFactory;
use std::sync::Arc;
use time::{Duration, OffsetDateTime};

/// TimeBarBuilder 结构体 - 使用泛型参数避免动态分发
#[derive(Debug)]
pub struct TimeBarBuilder<'a, T: TrNum + 'static, S: BarSeries<'a, T>> {
    /// 数值工厂
    pub num_factory: Arc<T::Factory>,
    /// 绑定的 BarSeries（可选，使用泛型参数）
    pub bar_series: Option<&'a mut S>,

    // Bar 构建字段
    pub time_period: Option<Duration>,
    pub begin_time: Option<OffsetDateTime>,
    pub end_time: Option<OffsetDateTime>,
    pub open_price: Option<T>,
    pub high_price: Option<T>,
    pub low_price: Option<T>,
    pub close_price: Option<T>,
    pub volume: Option<T>,
    pub amount: Option<T>,
    pub trades: Option<u64>,
}

// 针对DoubleNum的具体实现，直接调用DoubleNumFactory::instance()
impl<'a, S: BarSeries<'a, DoubleNum>> TimeBarBuilder<'a, DoubleNum, S> {
    pub fn new() -> Self {
        Self::new_with_factory(Arc::new(DoubleNumFactory::instance()))
    }
}

// 额外为泛型实现 Default trait
impl<'a, T: TrNum + 'static, S: BarSeries<'a, T>> Default for TimeBarBuilder<'a, T, S>
where
    T::Factory: Default,
{
    fn default() -> Self {
        Self::new_with_factory(Arc::new(T::Factory::default()))
    }
}

impl<'a, T: TrNum + 'static, S: BarSeries<'a, T>> TimeBarBuilder<'a, T, S> {
    /// 创建新的 TimeBarBuilder，指定数值工厂
    pub fn new_with_factory(num_factory: Arc<T::Factory>) -> Self {
        Self {
            num_factory,
            bar_series: None,
            time_period: None,
            begin_time: None,
            end_time: None,
            open_price: None,
            high_price: None,
            low_price: None,
            close_price: None,
            volume: None,
            amount: None,
            trades: None,
        }
    }

    /// 绑定到 BarSeries，返回新的类型化构建器
    pub fn bind_to(self, bar_series: &'a mut S) -> TimeBarBuilder<'a, T, S> {
        TimeBarBuilder {
            num_factory: self.num_factory,
            bar_series: Some(bar_series),
            time_period: self.time_period,
            begin_time: self.begin_time,
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
}

impl<'a, T: TrNum + 'static, S: BarSeries<'a, T>> BarBuilder<T> for TimeBarBuilder<'a, T, S>
where
    S: BarSeries<'a, T, Bar = BaseBar<T>>,
{
    type Bar = BaseBar<T>;

    fn time_period(mut self, time_period: Duration) -> Self {
        self.time_period = Some(time_period);
        self
    }

    fn begin_time(mut self, begin_time: OffsetDateTime) -> Self {
        self.begin_time = Some(begin_time);
        self
    }

    fn end_time(mut self, end_time: OffsetDateTime) -> Self {
        self.end_time = Some(end_time);
        self
    }

    fn open_price(mut self, open_price: T) -> Self {
        self.open_price = Some(open_price);
        self
    }

    fn high_price(mut self, high_price: T) -> Self {
        self.high_price = Some(high_price);
        self
    }

    fn low_price(mut self, low_price: T) -> Self {
        self.low_price = Some(low_price);
        self
    }

    fn close_price(mut self, close_price: T) -> Self {
        self.close_price = Some(close_price);
        self
    }

    fn volume(mut self, volume: T) -> Self {
        self.volume = Some(volume);
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
        // 构建 BaseBar，对应 Java 版本的 build 方法
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
        // 自动推导 amount = close_price * volume
        if self.amount.is_none() {
            if let (Some(ref close), Some(ref volume)) =
                (self.close_price.as_ref(), self.volume.as_ref())
            {
                self.amount = Some(close.multiplied_by(volume));
            }
        }

        let bar = self.build()?;

        if let Some(series) = self.bar_series.as_mut() {
            series.add_bar(bar);
        }

        Ok(())
    }
}
