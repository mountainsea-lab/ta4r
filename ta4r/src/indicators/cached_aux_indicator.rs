use crate::bar::types::BarSeries;
use crate::indicators::AuxIndicator;
use crate::indicators::types::IndicatorError;
use std::cell::RefCell;

use crate::indicators::abstract_indicator::BaseIndicator;
use crate::num::TrNum;

/// 静态分发、组合 BaseIndicator 的缓存指标器
pub struct CachedAuxIndicator<'a, I, T, S>
where
    I: AuxIndicator<Num = T, Series<'a> = S> + 'a,
    T: TrNum + 'static,
    S: BarSeries<'a, T>,
{
    base: BaseIndicator<'a, T, S>,
    indicator: I,
    cache: RefCell<Vec<Option<I::Output>>>,
}

impl<'a, I, T, S> Clone for CachedAuxIndicator<'a, I, T, S>
where
    I: AuxIndicator<Num = T, Series<'a> = S> + Clone,
    T: TrNum + 'static,
    S: BarSeries<'a, T>,
{
    fn clone(&self) -> Self {
        Self {
            base: self.base.clone(), // BaseIndicator 已经实现 Clone
            indicator: self.indicator.clone(),
            cache: RefCell::new(self.cache.borrow().clone()), // 克隆内部 Vec
        }
    }
}

impl<'a, I, T, S> CachedAuxIndicator<'a, I, T, S>
where
    I: AuxIndicator<Num = T, Series<'a> = S> + 'a,
    T: TrNum + 'static,
    S: BarSeries<'a, T>,
{
    pub fn new(indicator: I, series: &'a S) -> Self {
        Self {
            base: BaseIndicator::new(series),
            indicator,
            cache: RefCell::new(Vec::new()),
        }
    }

    pub fn get_cached_value(&self, index: usize) -> Result<I::Output, IndicatorError> {
        let mut cache = self.cache.borrow_mut();
        if index >= cache.len() {
            cache.resize(index + 1, None);
        }

        if let Some(value) = &cache[index] {
            Ok(value.clone())
        } else {
            let value = self.indicator.get_value(index)?;
            cache[index] = Some(value.clone());
            Ok(value)
        }
    }

    pub fn base(&self) -> &BaseIndicator<'a, T, S> {
        &self.base
    }
}

impl<'a, I, T, S> AuxIndicator for CachedAuxIndicator<'a, I, T, S>
where
    I: AuxIndicator<Num = T, Series<'a> = S> + 'a,
    T: TrNum + 'static,
    S: for<'any> BarSeries<'any, T>,
{
    type Output = I::Output;
    type Num = T;
    type Series<'b>
        = S
    where
        Self: 'b;
    fn get_value(&self, index: usize) -> Result<Self::Output, IndicatorError> {
        self.get_cached_value(index)
    }

    fn get_bar_series(&self) -> &Self::Series<'_> {
        self.base.get_bar_series()
    }

    fn get_count_of_unstable_bars(&self) -> usize {
        self.indicator.get_count_of_unstable_bars()
    }
}
