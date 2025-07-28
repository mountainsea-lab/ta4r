use crate::bar::types::{Bar, BarSeries};
use crate::indicators::Indicator;
use crate::indicators::types::{IndicatorError, IndicatorIterator};
use crate::num::TrNum;
use std::marker::PhantomData;

pub struct BaseIndicator<'a, T, S>
where
    T: TrNum + 'static,
    S: BarSeries<'a, T>,
{
    series: &'a S,
    _marker: PhantomData<T>,
}

impl<'a, T, S> Clone for BaseIndicator<'a, T, S>
where
    T: TrNum + 'static,
    S: BarSeries<'a, T>,
{
    fn clone(&self) -> Self {
        Self {
            series: self.series,
            _marker: PhantomData,
        }
    }
}

impl<'a, T, S> BaseIndicator<'a, T, S>
where
    T: TrNum + 'static,
    S: BarSeries<'a, T>,
{
    pub fn new(series: &'a S) -> Self {
        Self {
            series,
            _marker: Default::default(),
        }
    }

    pub fn get_bar_series(&self) -> &'a S {
        self.series
    }

    pub fn is_stable_at(&self, index: usize, unstable_count: usize) -> bool {
        index >= unstable_count
    }

    pub fn is_stable(&self, unstable_count: usize) -> bool {
        self.series.get_bar_count() >= unstable_count
    }

    pub fn iter<I>(&'a self, indicator: &'a I) -> IndicatorIterator<'a, I>
    where
        I: Indicator<Num = T, Series<'a> = S>,
    {
        match (self.series.get_begin_index(), self.series.get_end_index()) {
            (Some(begin), Some(end)) if begin <= end => IndicatorIterator {
                indicator,
                index: begin,
                end,
            },
            _ => IndicatorIterator {
                indicator,
                index: 1, // 让 index > end，表示空迭代器
                end: 0,
            },
        }
    }
}

impl<'a, T, S> Indicator for BaseIndicator<'a, T, S>
where
    T: TrNum + Clone + 'static,
    S: for<'any> BarSeries<'any, T>,
{
    type Num = T;
    type Series<'b>
        = S
    where
        Self: 'b;

    fn get_value(&self, index: usize) -> Result<T, IndicatorError> {
        let bar = self
            .series
            .get_bar(index)
            .ok_or(IndicatorError::OutOfBounds { index })?;
        let price = bar
            .get_close_price()
            .ok_or(IndicatorError::OutOfBounds { index })?;
        Ok(price)
    }

    fn get_bar_series(&self) -> &Self::Series<'_> {
        self.series
    }

    fn get_count_of_unstable_bars(&self) -> usize {
        0
    }
}
