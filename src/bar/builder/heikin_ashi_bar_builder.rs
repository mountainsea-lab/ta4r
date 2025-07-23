use crate::bar::base_bar::BaseBar;
use crate::bar::builder::time_bar_builder::TimeBarBuilder;
use crate::bar::types::BarSeries;
use crate::num::{NumFactory, TrNum};
use std::sync::Arc;
use time::{Duration, OffsetDateTime};

/// Heikin-Ashi Bar 构建器
pub struct HeikinAshiBarBuilder<'a, T: TrNum + 'static, S: BarSeries<'a, T>> {
    time_bar_builder: TimeBarBuilder<'a, T, S>,
    previous_heikin_ashi_open_price: Option<T>,
    previous_heikin_ashi_close_price: Option<T>,
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

    /// 构建 Bar
    pub fn build(&self) -> Option<BaseBar<T>> {
        let builder = &self.time_bar_builder;

        // 先安全地解构所需字段
        let (open, high, low, close) = (
            builder.open_price.as_ref()?,
            builder.high_price.as_ref()?,
            builder.low_price.as_ref()?,
            builder.close_price.as_ref()?,
        );

        let (volume, amount) = (builder.volume.as_ref()?, builder.amount.as_ref()?);

        let trades = builder.trades?;
        let end_time = builder.end_time?;
        let time_period = builder.time_period?;
        let begin_time = builder.begin_time.unwrap_or(end_time - time_period);

        let factory = builder.num_factory.clone();

        // 如果有前一根 heikin ashi bar，则计算
        if let (Some(prev_open), Some(prev_close)) = (
            self.previous_heikin_ashi_open_price.as_ref(),
            self.previous_heikin_ashi_close_price.as_ref(),
        ) {
            // TODO 需要实现引用版本基础计算函数
            let ha_close = open
                .add(high)
                .add(low)
                .add(close)
                .divided_by(&factory.num_of_i64(4));

            let ha_open = prev_open.add(prev_close).divided_by(&factory.num_of_i64(2));

            let ha_high = high.max(&ha_open).max(&ha_close);
            let ha_low = low.min(&ha_open).min(&ha_close);

            Some(BaseBar {
                time_period,
                begin_time,
                end_time,
                open_price: Some(ha_open),
                high_price: Some(ha_high),
                low_price: Some(ha_low),
                close_price: Some(ha_close),
                volume: volume.clone(),
                amount: amount.clone(),
                trades,
            })
        } else {
            // 否则按原始字段构建普通 Bar
            Some(BaseBar {
                time_period,
                begin_time,
                end_time,
                open_price: Some(open.clone()),
                high_price: Some(high.clone()),
                low_price: Some(low.clone()),
                close_price: Some(close.clone()),
                volume: volume.clone(),
                amount: amount.clone(),
                trades,
            })
        }
    }

    pub fn time_period(mut self, time_period: Duration) -> Self {
        self.time_bar_builder.time_period = Some(time_period);
        self
    }

    pub fn begin_time(mut self, begin_time: OffsetDateTime) -> Self {
        self.time_bar_builder.begin_time = Some(begin_time);
        self
    }

    pub fn end_time(mut self, end_time: OffsetDateTime) -> Self {
        self.time_bar_builder.end_time = Some(end_time);
        self
    }

    pub fn open_price(mut self, open_price: T) -> Self {
        self.time_bar_builder.open_price = Some(open_price);
        self
    }

    pub fn high_price(mut self, high_price: T) -> Self {
        self.time_bar_builder.high_price = Some(high_price);
        self
    }

    pub fn low_price(mut self, low_price: T) -> Self {
        self.time_bar_builder.low_price = Some(low_price);
        self
    }

    pub fn close_price(mut self, close_price: T) -> Self {
        self.time_bar_builder.close_price = Some(close_price);
        self
    }

    pub fn volume(mut self, volume: T) -> Self {
        self.time_bar_builder.volume = Some(volume);
        self
    }

    pub fn amount(mut self, amount: T) -> Self {
        self.time_bar_builder.amount = Some(amount);
        self
    }

    pub fn trades(mut self, trades: u64) -> Self {
        self.time_bar_builder.trades = Some(trades);
        self
    }
}
