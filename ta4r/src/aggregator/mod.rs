pub mod base_bar_series_aggregator;
pub mod types;

use crate::bar::types::{Bar, BarSeries};
use crate::num::TrNum;

pub trait BarAggregator<T: TrNum + 'static> {
    type Bar: Bar<T>;

    /// 将输入的一批 Bar 聚合为新的 Bar 序列
    /// 传入是对输入 Bar 的借用切片
    fn aggregate(&self, bars: &[Self::Bar]) -> Vec<Self::Bar>;
}

pub trait BarSeriesAggregator<T: TrNum + 'static> {
    type Bar: Bar<T>;
    type Series: for<'a> BarSeries<'a, T, Bar = Self::Bar>;

    /// 使用默认名称聚合整个 BarSeries，返回新的 BarSeries
    fn aggregate(&self, series: &Self::Series) -> Result<Self::Series, String> {
        let default_name = series.get_name();
        self.aggregate_with_name(series, default_name)
    }

    /// 聚合整个 BarSeries，返回指定名称的新 BarSeries
    fn aggregate_with_name(
        &self,
        series: &Self::Series,
        aggregated_series_name: &str,
    ) -> Result<Self::Series, String>;
}
