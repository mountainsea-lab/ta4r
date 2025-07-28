use crate::bar::types::BarSeries;
use crate::indicators::Indicator;
use crate::indicators::abstract_indicator::BaseIndicator;
use crate::indicators::cached_indicator::CachedIndicator;
use crate::indicators::types::{IndicatorCalculator, IndicatorError};
use crate::num::TrNum;

// use crate::bar::types::BarSeries;
// use crate::indicators::Indicator;
// use crate::indicators::cached_indicator::CachedIndicator;
// use crate::indicators::types::IndicatorError;
// use crate::num::TrNum;
//
// const RECURSION_THRESHOLD: usize = 100;
//
// /// 递归缓存指标
// pub struct RecursiveCachedIndicator<'a, T, S, F>
// where
//     T: TrNum + 'static,
//     S: BarSeries<'a, T>,
//     F: Fn(&CachedIndicator<'a, T, S, F>, usize) -> Result<T, IndicatorError> + Clone,
// {
//     inner: CachedIndicator<'a, T, S, F>,
// }
//
// impl<'a, T, S, F> RecursiveCachedIndicator<'a, T, S, F>
// where
//     T: TrNum + 'static,
//     S: BarSeries<'a, T>,
//     F: Fn(&CachedIndicator<'a, T, S, F>, usize) -> Result<T, IndicatorError> + Clone,
// {
//     pub fn new(series: &'a S, calculate_fn: F) -> Self {
//         Self {
//             inner: CachedIndicator::new_from_series(series, calculate_fn),
//         }
//     }
//
//     pub fn new_from_indicator<I>(indicator: &'a I, calculate_fn: F) -> Self
//     where
//         I: Indicator<Num = T, Series<'a> = S>,
//     {
//         Self {
//             inner: CachedIndicator::new_from_indicator(indicator, calculate_fn),
//         }
//     }
//
//     pub fn get_value(&self, index: usize) -> Result<T, IndicatorError> {
//         let series = self.inner.base.get_bar_series();
//
//         if let Some(end_index) = series.get_end_index() {
//             if index <= end_index {
//                 let removed_bars_count = series.get_removed_bars_count();
//
//                 let highest_result_index = *self.inner.highest_result_index.borrow();
//
//                 let highest_result_index_usize = if highest_result_index < 0 {
//                     0
//                 } else {
//                     highest_result_index as usize
//                 };
//
//                 let start_index = std::cmp::max(removed_bars_count, highest_result_index_usize);
//
//                 if index > start_index + RECURSION_THRESHOLD {
//                     for prev_index in start_index..index {
//                         // 这里调用 inner.get_value(&self, usize) 也是不可变借用，
//                         // 内部用 RefCell 管理可变性，没问题
//                         self.inner.get_cached_value(prev_index)?;
//                     }
//                 }
//             }
//         }
//
//         self.inner.get_cached_value(index)
//     }
//
//     pub fn get_bar_series(&self) -> &'a S {
//         self.inner.base.get_bar_series()
//     }
//
//     pub fn into_inner(self) -> CachedIndicator<'a, T, S, F> {
//         self.inner
//     }
// }

const RECURSION_THRESHOLD: usize = 100;

pub struct RecursiveCalcWrapper<C> {
    pub(crate) inner: C,
    pub(crate) threshold: usize,
}

impl<C> Clone for RecursiveCalcWrapper<C>
where
    C: Clone,
{
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            threshold: self.threshold,
        }
    }
}

impl<'a, T, S, C> IndicatorCalculator<'a, T, S> for RecursiveCalcWrapper<C>
where
    T: TrNum + Clone + 'static,
    S: BarSeries<'a, T>,
    C: IndicatorCalculator<'a, T, S> + Clone,
{
    fn calculate(&self, base: &BaseIndicator<'a, T, S>, index: usize) -> Result<T, IndicatorError> {
        // 不负责递归预计算，直接调用内层计算器
        self.inner.calculate(base, index)
    }
}

pub struct RecursiveCachedIndicator<'a, T, S, C>
where
    T: TrNum + Clone + 'static,
    S: BarSeries<'a, T>,
    C: IndicatorCalculator<'a, T, S> + Clone,
{
    pub(crate) cached: CachedIndicator<'a, T, S, RecursiveCalcWrapper<C>>,
}

impl<'a, T, S, C> Clone for RecursiveCachedIndicator<'a, T, S, C>
where
    T: TrNum + Clone + 'static,
    S: BarSeries<'a, T>,
    C: IndicatorCalculator<'a, T, S> + Clone,
{
    fn clone(&self) -> Self {
        Self {
            cached: self.cached.clone(),
        }
    }
}

impl<'a, T, S, C> RecursiveCachedIndicator<'a, T, S, C>
where
    T: TrNum + Clone + 'static,
    S: BarSeries<'a, T>,
    C: IndicatorCalculator<'a, T, S> + Clone,
{
    pub fn new(series: &'a S, calculator: C) -> Self {
        Self::new_with_threshold(series, calculator, RECURSION_THRESHOLD)
    }

    pub fn new_with_threshold(series: &'a S, calculator: C, threshold: usize) -> Self {
        let wrapper = RecursiveCalcWrapper {
            inner: calculator,
            threshold,
        };
        Self {
            cached: CachedIndicator::new_from_series(series, wrapper),
        }
    }

    /// 从现有 Indicator 构造，使用默认阈值
    pub fn from_indicator<I>(indicator: &'a I, calculator: C) -> Self
    where
        I: Indicator<Num = T, Series<'a> = S>,
    {
        Self::from_indicator_with_threshold(indicator, calculator, RECURSION_THRESHOLD)
    }

    /// 从现有 Indicator 构造，自定义阈值
    pub fn from_indicator_with_threshold<I>(
        indicator: &'a I,
        calculator: C,
        threshold: usize,
    ) -> Self
    where
        I: Indicator<Num = T, Series<'a> = S>,
    {
        let wrapper = RecursiveCalcWrapper {
            inner: calculator,
            threshold,
        };
        let cached = CachedIndicator::new_from_indicator(indicator, wrapper);
        Self { cached }
    }

    pub fn get_value(&self, index: usize) -> Result<T, IndicatorError> {
        let series = self.cached.base.get_bar_series();

        if series.get_bar_count() == 0 || index > series.get_end_index().unwrap_or(usize::MAX) {
            // 超出范围，直接计算
            return self.cached.get_cached_value(index);
        }

        let removed = series.get_removed_bars_count();
        let highest = *self.cached.highest_result_index.borrow();

        let start = std::cmp::max(removed, if highest < 0 { 0 } else { highest as usize });

        if index > start && (index - start) > self.cached.calculator.threshold {
            // 迭代计算避免深递归
            for i in start..index {
                self.cached.get_cached_value(i)?;
            }
        }

        self.cached.get_cached_value(index)
    }
}

impl<'a, T, S, C> Indicator for RecursiveCachedIndicator<'a, T, S, C>
where
    T: TrNum + Clone + 'static,
    S: for<'any> BarSeries<'any, T>,
    C: IndicatorCalculator<'a, T, S> + Clone,
{
    type Num = T;
    type Series<'b>
        = S
    where
        Self: 'b;

    fn get_value(&self, index: usize) -> Result<T, IndicatorError> {
        self.get_value(index)
    }

    fn get_bar_series(&self) -> &Self::Series<'_> {
        self.cached.get_bar_series()
    }

    fn get_count_of_unstable_bars(&self) -> usize {
        0
    }
}
