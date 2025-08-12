use crate::aggregator::BarAggregator;
use crate::aggregator::types::unwrap_or_err;
use crate::bar::base_bar::BaseBar;
use crate::bar::base_bar_series::BaseBarSeries;
use crate::bar::builder::heikin_ashi_bar_builder::HeikinAshiBarBuilder;
use crate::bar::types::{BarBuilder, BarSeries};
use crate::num::TrNum;
use std::marker::PhantomData;
use std::sync::Arc;

// pub struct HeikinAshiBarAggregator<T, BA>
// where
//     T: TrNum + 'static,
//     BA: BarAggregator<T, Bar = BaseBar<T>>,
// {
//     _marker: PhantomData<(T, BA)>,
//     num_factory: Arc<T::Factory>,
// }
//
// impl<T, BA> HeikinAshiBarAggregator<T, BA>
// where
//     T: TrNum + 'static,
//     BA: BarAggregator<T, Bar = BaseBar<T>>,
// {
//     pub fn new() -> Self {
//         Self {
//             _marker: PhantomData,
//             num_factory: Arc::new(T::Factory::default()),
//         }
//     }
// }
//
// impl<T, BA> BarAggregator<T> for HeikinAshiBarAggregator<T, BA>
// where
//     T: TrNum + 'static,
//     BA: BarAggregator<T, Bar = BaseBar<T>>,
// {
//     type Bar = BaseBar<T>;
//     fn aggregate(&self, bars: &[BaseBar<T>]) -> Result<Vec<Self::Bar>, String> {
//         if bars.is_empty() {
//             return Ok(Vec::new());
//         }
//
//         let mut result = Vec::with_capacity(bars.len());
//         let mut previous_open: Option<T> = None;
//         let mut previous_close: Option<T> = None;
//
//         // 用值链式调用构建 ha_builder，不用 &mut
//         let mut ha_builder = HeikinAshiBarBuilder::<T, BaseBarSeries<T>>::new_with_factory(self.num_factory.clone());
//
//         for bar in bars {
//             let open = unwrap_or_err(bar.open_price.clone(), "open price")?;
//             let high = unwrap_or_err(bar.high_price.clone(), "high price")?;
//             let low = unwrap_or_err(bar.low_price.clone(), "low price")?;
//             let close = unwrap_or_err(bar.close_price.clone(), "close price")?;
//
//             ha_builder
//                 .time_period(bar.time_period)
//                 .begin_time(bar.begin_time)
//                 .end_time(bar.end_time)
//                 .open_price(open)
//                 .high_price(high)
//                 .low_price(low)
//                 .close_price(close)
//                 .volume(bar.volume.clone())
//                 .amount(bar.amount.clone().unwrap_or_else(|| T::zero()))
//                 .trades(bar.trades);
//
//             // 有前一根的 Heikin-Ashi Open/Close 就继续链式调用更新
//             if let (Some(po), Some(pc)) = (&previous_open, &previous_close) {
//                 ha_builder
//                     .previous_heikin_ashi_open_price(Some(po.clone()))
//                     .previous_heikin_ashi_close_price(Some(pc.clone()));
//             } else {
//                 ha_builder
//                     .previous_heikin_ashi_open_price(None)
//                     .previous_heikin_ashi_close_price(None);
//             }
//
//             let ha_bar = ha_builder.build()?;
//
//             previous_open = ha_bar.open_price.clone();
//             previous_close = ha_bar.close_price.clone();
//
//             result.push(ha_bar);
//
//         }
//
//         Ok(result)
//     }
// }

pub struct HeikinAshiBarAggregator<T>
where
    T: TrNum + 'static,
{
    _marker: PhantomData<T>,
    num_factory: Arc<T::Factory>,
}

impl<T> HeikinAshiBarAggregator<T>
where
    T: TrNum + 'static,
{
    pub fn new() -> Self {
        Self {
            _marker: PhantomData,
            num_factory: Arc::new(T::Factory::default()),
        }
    }
}

impl<T> BarAggregator<T> for HeikinAshiBarAggregator<T>
where
    T: TrNum + 'static,
{
    type Bar = BaseBar<T>;
    fn aggregate(&self, bars: &[BaseBar<T>]) -> Result<Vec<Self::Bar>, String> {
        if bars.is_empty() {
            return Ok(Vec::new());
        }

        let mut result = Vec::with_capacity(bars.len());
        let mut previous_open: Option<T> = None;
        let mut previous_close: Option<T> = None;

        // 用值链式调用构建 ha_builder，不用 &mut
        let mut ha_builder =
            HeikinAshiBarBuilder::<T, BaseBarSeries<T>>::new_with_factory(self.num_factory.clone());

        for bar in bars {
            let open = unwrap_or_err(bar.open_price.clone(), "open price")?;
            let high = unwrap_or_err(bar.high_price.clone(), "high price")?;
            let low = unwrap_or_err(bar.low_price.clone(), "low price")?;
            let close = unwrap_or_err(bar.close_price.clone(), "close price")?;

            ha_builder
                .time_period(bar.time_period)
                .begin_time(bar.begin_time)
                .end_time(bar.end_time)
                .open_price(open)
                .high_price(high)
                .low_price(low)
                .close_price(close)
                .volume(bar.volume.clone())
                .amount(bar.amount.clone().unwrap_or_else(|| T::zero()))
                .trades(bar.trades);

            // 有前一根的 Heikin-Ashi Open/Close 就继续链式调用更新
            if let (Some(po), Some(pc)) = (&previous_open, &previous_close) {
                ha_builder
                    .previous_heikin_ashi_open_price(Some(po.clone()))
                    .previous_heikin_ashi_close_price(Some(pc.clone()));
            } else {
                ha_builder
                    .previous_heikin_ashi_open_price(None)
                    .previous_heikin_ashi_close_price(None);
            }

            let ha_bar = ha_builder.build()?;

            previous_open = ha_bar.open_price.clone();
            previous_close = ha_bar.close_price.clone();

            result.push(ha_bar);
        }

        Ok(result)
    }
}
