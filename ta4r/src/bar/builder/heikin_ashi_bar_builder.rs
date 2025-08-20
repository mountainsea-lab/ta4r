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
use crate::bar::base_bar::BaseBar;
use crate::bar::builder::time_bar_builder::TimeBarBuilder;
use crate::bar::types::{BarBuilder, BarSeries};
use crate::num::{NumFactory, TrNum};
use parking_lot::RwLock;
use std::sync::Arc;
use time::{Duration, OffsetDateTime};

/// Heikin-Ashi Bar 构建器
pub struct HeikinAshiBarBuilder<T: TrNum + 'static, S: BarSeries<T>> {
    time_bar_builder: TimeBarBuilder<T, S>,
    previous_heikin_ashi_open_price: Option<T>,
    previous_heikin_ashi_close_price: Option<T>,
}

impl<T: TrNum + 'static, S: BarSeries<T>> BarBuilder<T> for HeikinAshiBarBuilder<T, S>
where
    S: BarSeries<T, Bar = BaseBar<T>>,
{
    type Bar = BaseBar<T>;

    fn time_period(&mut self, time_period: Duration) -> &mut Self {
        self.time_bar_builder.time_period = Some(time_period);
        self
    }

    fn begin_time(&mut self, begin_time: OffsetDateTime) -> &mut Self {
        self.time_bar_builder.begin_time = Some(begin_time);
        self
    }

    fn end_time(&mut self, end_time: OffsetDateTime) -> &mut Self {
        self.time_bar_builder.end_time = Some(end_time);
        self
    }

    fn open_price(&mut self, open_price: T) -> &mut Self {
        self.time_bar_builder.open_price = Some(open_price);
        self
    }

    fn high_price(&mut self, high_price: T) -> &mut Self {
        self.time_bar_builder.high_price = Some(high_price);
        self
    }

    fn low_price(&mut self, low_price: T) -> &mut Self {
        self.time_bar_builder.low_price = Some(low_price);
        self
    }

    fn close_price(&mut self, close_price: T) -> &mut Self {
        self.time_bar_builder.close_price = Some(close_price);
        self
    }

    fn volume(&mut self, volume: T) -> &mut Self {
        self.time_bar_builder.volume = Some(volume);
        self
    }

    fn amount(&mut self, amount: T) -> &mut Self {
        self.time_bar_builder.amount = Some(amount);
        self
    }

    fn trades(&mut self, trades: u64) -> &mut Self {
        self.time_bar_builder.trades = Some(trades);
        self
    }

    fn build(&self) -> Result<Self::Bar, String> {
        let factory = &self.time_bar_builder.num_factory;

        if let (Some(open), Some(high), Some(low), Some(close)) = (
            &self.time_bar_builder.open_price,
            &self.time_bar_builder.high_price,
            &self.time_bar_builder.low_price,
            &self.time_bar_builder.close_price,
        ) {
            if let (Some(prev_open), Some(prev_close)) = (
                &self.previous_heikin_ashi_open_price,
                &self.previous_heikin_ashi_close_price,
            ) {
                let four = factory.num_of_i64(4);
                let two = factory.num_of_i64(2);

                let ha_close = open
                    .add_ref(high)
                    .add_ref(low)
                    .add_ref(close)
                    .divided_by_ref(&four)
                    .map_err(|e| e.to_string())?;

                let ha_open = prev_open
                    .add_ref(prev_close)
                    .divided_by_ref(&two)
                    .map_err(|e| e.to_string())?;

                let ha_high = high.max(&ha_open).max(&ha_close);
                let ha_low = low.min(&ha_open).min(&ha_close);

                return BaseBar::new(
                    self.time_bar_builder
                        .time_period
                        .ok_or("Missing time_period")?,
                    self.time_bar_builder.end_time.ok_or("Missing end_time")?,
                    Some(ha_open),
                    Some(ha_high),
                    Some(ha_low),
                    Some(ha_close),
                    self.time_bar_builder
                        .volume
                        .clone()
                        .ok_or("Missing volume")?,
                    self.time_bar_builder.amount.clone(),
                    self.time_bar_builder.trades.unwrap_or(0),
                );
            }
        }

        // fallback
        self.time_bar_builder.build()
    }

    fn add(&mut self) -> Result<(), String> {
        self.time_bar_builder.add()
    }
}

impl<T: TrNum + 'static, S: BarSeries<T>> HeikinAshiBarBuilder<T, S> {
    pub fn new_with_factory(num_factory: Arc<T::Factory>) -> Self {
        Self {
            time_bar_builder: TimeBarBuilder::new_with_factory(num_factory),
            previous_heikin_ashi_open_price: None,
            previous_heikin_ashi_close_price: None,
        }
    }
    pub fn bind_to(mut self, series: &mut S) -> Self {
        self.time_bar_builder = self.time_bar_builder.bind_to(series);
        self
    }

    pub fn bind_shared(mut self, series: Arc<RwLock<S>>) -> Self {
        self.time_bar_builder = self.time_bar_builder.bind_shared(series);
        self
    }

    pub fn previous_heikin_ashi_open_price(&mut self, previous_open_price: Option<T>) -> &mut Self {
        self.previous_heikin_ashi_open_price = previous_open_price;
        self
    }

    pub fn previous_heikin_ashi_close_price(
        &mut self,
        previous_close_price: Option<T>,
    ) -> &mut Self {
        self.previous_heikin_ashi_close_price = previous_close_price;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bar::base_bar_series::BaseBarSeries;
    use crate::bar::types::Bar;
    use crate::num::decimal_num::DecimalNum;
    use crate::num::decimal_num_factory::DecimalNumFactory;
    use crate::num::double_num::DoubleNum;
    use crate::num::double_num_factory::DoubleNumFactory;
    use std::sync::Arc;
    use time::Duration;
    use time_macros::datetime;

    #[derive(Clone)]
    enum NumFactoryKind {
        DoubleFactory,
        DecimalFactory,
    }

    impl NumFactoryKind {
        fn name(&self) -> &'static str {
            match self {
                NumFactoryKind::DoubleFactory => "DoubleNumFactory",
                NumFactoryKind::DecimalFactory => "DecimalNumFactory",
            }
        }

        fn factory_double() -> Arc<DoubleNumFactory> {
            Arc::new(DoubleNumFactory::instance())
        }

        fn factory_decimal() -> Arc<DecimalNumFactory> {
            Arc::new(DecimalNumFactory::instance())
        }
    }

    #[test]
    fn test_heikin_ashi_bar_builder_all() {
        println!("Running test for DoubleNumFactory");
        test_heikin_ashi_for::<DoubleNum>(NumFactoryKind::factory_double());

        println!("Running test for DecimalNumFactory");
        test_heikin_ashi_for::<DecimalNum>(NumFactoryKind::factory_decimal());
    }

    fn test_heikin_ashi_for<T>(num_factory: Arc<T::Factory>)
    where
        T: TrNum + 'static,
    {
        let price_open = num_factory.num_of_i64(100);
        let price_high = num_factory.num_of_i64(110);
        let price_low = num_factory.num_of_i64(95);
        let price_close = num_factory.num_of_i64(105);
        let volume = num_factory.num_of_i64(10);
        let amount = num_factory.num_of_i64(1000);
        let trades = 1;

        println!("Price open: {}", price_open);
        println!("Price high: {}", price_high);
        println!("Price low: {}", price_low);
        println!("Price close: {}", price_close);
        println!("Volume: {}", volume);
        println!("Amount: {}", amount);

        // 这里写你具体的 HeikinAshiBarBuilder 测试逻辑

        let time_period = Duration::hours(1);
        let end_time = datetime!(2014-01-01 01:00:00 UTC);

        let input_bar = BaseBar::new(
            time_period,
            end_time,
            Some(price_open.clone()),
            Some(price_high.clone()),
            Some(price_low.clone()),
            Some(price_close.clone()),
            volume.clone(),
            Some(amount.clone()),
            trades,
        )
        .unwrap();

        // ✅ Case 1: 无前一 HA 数据，应返回与原始 bar 相同的字段
        let bar1 =
            HeikinAshiBarBuilder::<T, BaseBarSeries<T>>::new_with_factory(Arc::clone(&num_factory))
                .time_period(time_period)
                .end_time(end_time)
                .open_price(price_open.clone())
                .high_price(price_high.clone())
                .low_price(price_low.clone())
                .close_price(price_close.clone())
                .volume(volume.clone())
                .amount(amount.clone())
                .trades(trades)
                .build()
                .unwrap();

        assert_eq!(bar1.get_open_price(), Some(price_open.clone()));
        assert_eq!(bar1.get_high_price(), Some(price_high.clone()));
        assert_eq!(bar1.get_low_price(), Some(price_low.clone()));
        assert_eq!(bar1.get_close_price(), Some(price_close.clone()));

        // ✅ Case 2: 有前一 HA 数据，计算 HA open/close/high/low
        let prev_ha_open = num_factory.num_of_f64(100.0);
        let prev_ha_close = num_factory.num_of_f64(105.0);

        let expected_ha_close = price_open
            .add_ref(&price_high)
            .add_ref(&price_low)
            .add_ref(&price_close)
            .divided_by_ref(&num_factory.num_of_i64(4))
            .unwrap();

        let expected_ha_open = prev_ha_open
            .add_ref(&prev_ha_close)
            .divided_by_ref(&num_factory.num_of_i64(2))
            .unwrap();

        let expected_ha_high = price_high.max(&expected_ha_open).max(&expected_ha_close);

        let expected_ha_low = price_low.min(&expected_ha_open).min(&expected_ha_close);

        let ha_bar =
            HeikinAshiBarBuilder::<T, BaseBarSeries<T>>::new_with_factory(Arc::clone(&num_factory))
                .previous_heikin_ashi_open_price(prev_ha_open)
                .previous_heikin_ashi_close_price(prev_ha_close)
                .time_period(time_period)
                .end_time(end_time)
                .open_price(price_open.clone())
                .high_price(price_high.clone())
                .low_price(price_low.clone())
                .close_price(price_close.clone())
                .volume(volume.clone())
                .amount(amount.clone())
                .trades(trades)
                .build()
                .unwrap();

        assert_eq!(ha_bar.get_open_price(), Some(expected_ha_open.clone()));
        assert_eq!(ha_bar.get_close_price(), Some(expected_ha_close.clone()));
        assert_eq!(ha_bar.get_high_price(), Some(expected_ha_high.clone()));
        assert_eq!(ha_bar.get_low_price(), Some(expected_ha_low.clone()));
    }
}
