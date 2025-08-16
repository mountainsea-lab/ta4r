use crate::bar::types::{Bar, BarSeries};
use crate::indicators::abstract_indicator::BaseIndicator;
use crate::indicators::cached_indicator::CachedIndicator;
use crate::indicators::types::{IndicatorCalculator, IndicatorError};
use crate::num::TrNum;

/// True range indicator.
///
/// # Formula
///
/// ```text
/// TrueRange = MAX(high - low, high - previousClose, previousClose - low)
/// ```
#[derive(Clone)]
pub struct TRCalculator;

impl<'a, T, S> IndicatorCalculator<'a, T, S> for TRCalculator
where
    T: TrNum + Clone + 'static,
    S: BarSeries<'a, T>,
{
    fn calculate(&self, base: &BaseIndicator<'a, T, S>, index: usize) -> Result<T, IndicatorError> {
        let series = base.get_bar_series();

        // 当前 bar
        let bar = series
            .get_bar(index)
            .ok_or(IndicatorError::OutOfBounds { index })?;

        let high = bar
            .get_high_price()
            .ok_or_else(|| IndicatorError::CalculationError {
                message: format!("High price missing at index {}", index),
            })?;

        let low = bar
            .get_low_price()
            .ok_or_else(|| IndicatorError::CalculationError {
                message: format!("Low price missing at index {}", index),
            })?;

        let hl = high.minus(&low).abs();

        if index == 0 {
            return Ok(hl);
        }

        // 上一个 bar
        let prev_bar = series
            .get_bar(index - 1)
            .ok_or(IndicatorError::OutOfBounds { index: index - 1 })?;

        let previous_close =
            prev_bar
                .get_close_price()
                .ok_or_else(|| IndicatorError::CalculationError {
                    message: format!("Close price missing at index {}", index - 1),
                })?;

        let hc = high.minus(&previous_close).abs();
        let cl = previous_close.minus(&low).abs();

        Ok(hl.max(&hc).max(&cl))
    }
}

/// True range indicator.
///
/// Calculates the true range for a bar series:
/// ```text
/// TrueRange = MAX(high - low, high - previousClose, previousClose - low)
/// ```
pub type TRIndicator<'a, T, S> = CachedIndicator<'a, T, S, TRCalculator>;

impl<'a, T, S> TRIndicator<'a, T, S>
where
    T: TrNum + Clone + 'static,
    S: BarSeries<'a, T>,
{
    /// Creates a new TRIndicator from a bar series
    pub fn new(series: &'a S) -> Self {
        CachedIndicator::new_from_series(series, TRCalculator {})
    }

    pub fn get_count_of_unstable_bars(&self) -> usize {
        1
    }
}
