use crate::bar::types::BarSeries;
use crate::indicators::Indicator;
use crate::indicators::averages::mma_indicator::MMAIndicator;
use crate::indicators::helpers::tr_indicator::TRIndicator;
use crate::indicators::types::IndicatorError;
use crate::num::TrNum;

/// ATRIndicator
pub struct ATRIndicator<'a, T, S>
where
    T: TrNum + Clone + 'static,
    S: for<'b> BarSeries<'b, T>,
{
    tr_indicator: &'a TRIndicator<'a, T, S>,
    average_true_range: MMAIndicator<'a, T, S, TRIndicator<'a, T, S>>,
}

impl<'a, T, S> Clone for ATRIndicator<'a, T, S>
where
    T: TrNum + Clone + 'static,
    S: for<'b> BarSeries<'b, T>,
{
    fn clone(&self) -> Self {
        Self {
            tr_indicator: self.tr_indicator,
            average_true_range: self.average_true_range.clone(),
        }
    }
}

impl<'a, T, S> ATRIndicator<'a, T, S>
where
    T: TrNum + Clone + 'static,
    S: for<'b> BarSeries<'b, T>,
{
    /// 使用已有 TRIndicator 创建 ATR
    pub fn from_tr(tr_indicator: &'a TRIndicator<'a, T, S>, bar_count: usize) -> Self {
        let atr = MMAIndicator::new(tr_indicator, bar_count).unwrap();
        Self {
            tr_indicator,
            average_true_range: atr,
        }
    }

    /// 获取 TRIndicator 引用
    pub fn get_tr_indicator(&self) -> &TRIndicator<'a, T, S> {
        self.tr_indicator
    }
}

// 实现 Indicator trait
impl<'a, T, S> Indicator for ATRIndicator<'a, T, S>
where
    T: TrNum + Clone + 'static,
    S: for<'b> BarSeries<'b, T>,
{
    type Num = T;
    type Series<'b>
        = S
    where
        Self: 'b;

    fn get_value(&self, index: usize) -> Result<T, IndicatorError> {
        self.average_true_range.get_value(index)
    }

    fn get_bar_series(&self) -> &Self::Series<'_> {
        self.tr_indicator.get_bar_series()
    }

    fn get_count_of_unstable_bars(&self) -> usize {
        self.average_true_range.get_count_of_unstable_bars()
    }
}

// 可选：Debug
impl<'a, T, S> std::fmt::Debug for ATRIndicator<'a, T, S>
where
    T: TrNum + Clone + std::fmt::Debug,
    S: for<'b> BarSeries<'b, T>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ATRIndicator count_of_unstable_bars: {}",
            self.get_count_of_unstable_bars()
        )
    }
}
