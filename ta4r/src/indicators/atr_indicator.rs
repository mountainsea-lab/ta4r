use crate::bar::builder::types::BarSeriesRef;
use crate::bar::types::BarSeries;
use crate::indicators::Indicator;
use crate::indicators::averages::mma_indicator::MMAIndicator;
use crate::indicators::helpers::tr_indicator::TRIndicator;
use crate::indicators::types::IndicatorError;
use crate::num::TrNum;
use std::sync::Arc;

/// ATRIndicator
pub struct ATRIndicator<T, S>
where
    T: TrNum + Clone + 'static,
    S: BarSeries<T> + 'static,
{
    tr_indicator: Arc<TRIndicator<T, S>>,
    average_true_range: MMAIndicator<T, S, TRIndicator<T, S>>,
}

impl<T, S> Clone for ATRIndicator<T, S>
where
    T: TrNum + Clone + 'static,
    S: BarSeries<T> + 'static,
{
    fn clone(&self) -> Self {
        Self {
            tr_indicator: Arc::clone(&self.tr_indicator),
            average_true_range: self.average_true_range.clone(),
        }
    }
}

impl<T, S> ATRIndicator<T, S>
where
    T: TrNum + Clone + 'static,
    S: BarSeries<T> + 'static,
{
    /// 使用已有 TRIndicator 创建 ATR
    pub fn from_tr(tr_indicator: Arc<TRIndicator<T, S>>, bar_count: usize) -> Self {
        let atr = MMAIndicator::new(Arc::clone(&tr_indicator), bar_count).unwrap();
        Self {
            tr_indicator,
            average_true_range: atr,
        }
    }

    /// 获取 TRIndicator 引用
    pub fn get_tr_indicator(&self) -> Arc<TRIndicator<T, S>> {
        Arc::clone(&self.tr_indicator)
    }
}

// 实现 Indicator trait
impl<T, S> Indicator for ATRIndicator<T, S>
where
    T: TrNum + Clone + 'static,
    S: BarSeries<T> + 'static,
{
    type Num = T;
    type Output = T;
    type Series = S;

    fn get_value(&self, index: usize) -> Result<T, IndicatorError> {
        self.average_true_range.get_value(index)
    }

    fn bar_series(&self) -> BarSeriesRef<Self::Series> {
        self.tr_indicator.bar_series()
    }

    fn count_of_unstable_bars(&self) -> usize {
        self.average_true_range.count_of_unstable_bars()
    }
}

// 可选：Debug
impl<T, S> std::fmt::Debug for ATRIndicator<T, S>
where
    T: TrNum + Clone + std::fmt::Debug,
    S: BarSeries<T> + 'static,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ATRIndicator count_of_unstable_bars: {}",
            self.count_of_unstable_bars()
        )
    }
}
