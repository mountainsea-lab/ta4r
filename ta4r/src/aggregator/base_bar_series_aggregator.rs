use crate::aggregator::{BarAggregator, BarSeriesAggregator};
use crate::bar::base_bar::BaseBar;
use crate::bar::base_bar_series::BaseBarSeries;
use crate::bar::base_bar_series_builder::BaseBarSeriesBuilder;
use crate::bar::types::{BarSeries, BarSeriesBuilder};
use crate::num::TrNum;
use std::marker::PhantomData;

// pub struct BaseBarSeriesAggregator<T, BA, S>
// where
//     T: TrNum + 'static,
//     BA: BarAggregator<T>,
//     S: for<'a> BarSeries<'a, T, Bar = BA::Bar>,
// {
//     bar_aggregator: BA,
//     _marker: std::marker::PhantomData<(T, S)>,
// }
//
// impl<T, BA, S> BaseBarSeriesAggregator<T, BA, S>
// where
//     T: TrNum + 'static,
//     BA: BarAggregator<T>,
//     S: for<'a> BarSeries<'a, T, Bar = BA::Bar>,
// {
//     pub fn new(bar_aggregator: BA) -> Self {
//         Self {
//             bar_aggregator,
//             _marker: std::marker::PhantomData,
//         }
//     }
// }
//
// impl<T, BA, S> BarSeriesAggregator<T> for BaseBarSeriesAggregator<T, BA, S>
// where
//     T: TrNum + 'static,
//     BA: BarAggregator<T>,
//     S: for<'a> BarSeries<'a, T, Bar = BA::Bar>,
// {
//     type Bar = BA::Bar;
//     type Series = BaseBarSeries<T>;
//
//     fn aggregate_with_name(&self, series: &S, aggregated_series_name: &str) -> Self::Series {
//         let bars = series.bars();
//         let aggregated_bars = self.bar_aggregator.aggregate(bars);
//
//         BaseBarSeriesBuilder::<T>::default()
//             .with_name(aggregated_series_name)
//             .with_bars(aggregated_bars)
//             .build()
//     }
// }
pub struct BaseBarSeriesAggregator<T, BA>
where
    T: TrNum + 'static,
    BA: BarAggregator<T, Bar = BaseBar<T>>,
{
    bar_aggregator: BA,
    _marker: PhantomData<T>, // 添加这个字段来“使用”T
}

impl<T, BA> BaseBarSeriesAggregator<T, BA>
where
    T: TrNum + 'static,
    BA: BarAggregator<T, Bar = BaseBar<T>>,
{
    pub fn new(bar_aggregator: BA) -> Self {
        Self {
            bar_aggregator,
            _marker: PhantomData,
        }
    }
}

impl<T, BA> BarSeriesAggregator<T> for BaseBarSeriesAggregator<T, BA>
where
    T: TrNum + 'static,
    BA: BarAggregator<T, Bar = BaseBar<T>>,
{
    type Bar = BaseBar<T>;
    type Series = BaseBarSeries<T>;

    fn aggregate_with_name(
        &self,
        series: &BaseBarSeries<T>,
        aggregated_series_name: &str,
    ) -> Result<Self::Series, String> {
        let bars = series.get_bar_data();
        let aggregated_bars = self.bar_aggregator.aggregate(bars);

        BaseBarSeriesBuilder::<T>::default()
            .with_name(aggregated_series_name.to_string())
            .with_bars(aggregated_bars)
            .build()
    }
}
