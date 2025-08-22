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
use crate::bar::builder::types::BarSeriesRef;
use crate::bar::types::BarSeries;
use crate::indicators::Indicator;
use crate::indicators::abstract_indicator::BaseIndicator;
use crate::indicators::cached_indicator::CachedIndicator;
use crate::indicators::types::{IndicatorCalculator, IndicatorError};
use crate::num::TrNum;
use std::marker::PhantomData;
use std::sync::Arc;

/// CrossCalculator 负责计算交叉逻辑 IU Indicator Up, IL Indicator Low
pub struct CrossCalculator<T, S, IU, IL>
where
    T: TrNum + Clone + 'static,
    S: BarSeries<T> + 'static,
    IU: Indicator<Num = T, Output = T, Series = S>,
    IL: Indicator<Num = T, Output = T, Series = S>,
{
    up: Arc<IU>,
    low: Arc<IL>,
    _phantom: PhantomData<(T, S)>,
}

impl<T, S, IU, IL> Clone for CrossCalculator<T, S, IU, IL>
where
    T: TrNum + Clone + 'static,
    S: BarSeries<T> + 'static,
    IU: Indicator<Num = T, Output = T, Series = S>,
    IL: Indicator<Num = T, Output = T, Series = S>,
{
    fn clone(&self) -> Self {
        Self {
            up: Arc::clone(&self.up),
            low: Arc::clone(&self.low),
            _phantom: PhantomData,
        }
    }
}

impl<T, S, IU, IL> CrossCalculator<T, S, IU, IL>
where
    T: TrNum + Clone + 'static,
    S: BarSeries<T> + 'static,
    IU: Indicator<Num = T, Output = T, Series = S>,
    IL: Indicator<Num = T, Output = T, Series = S>,
{
    pub fn new(up: Arc<IU>, low: Arc<IL>) -> Self {
        Self {
            up,
            low,
            _phantom: PhantomData,
        }
    }
}

impl<T, S, IU, IL> IndicatorCalculator<T, S> for CrossCalculator<T, S, IU, IL>
where
    T: TrNum + Clone + From<bool> + 'static, // ✅ 新增 From<bool> 约束
    S: BarSeries<T> + 'static,
    IU: Indicator<Num = T, Output = T, Series = S>,
    IL: Indicator<Num = T, Output = T, Series = S>,
{
    type Output = T;

    fn calculate(
        &self,
        _base: &BaseIndicator<T, S>,
        index: usize,
    ) -> Result<Self::Output, IndicatorError> {
        // index = 0 特殊处理，无法判断是否穿越
        if index == 0 {
            return Ok(false.into());
        }

        let up_value = self.up.get_value(index)?;
        let low_value = self.low.get_value(index)?;

        // 当前 up 没有穿越 low
        if up_value.is_greater_than_or_equal(&low_value) {
            return Ok(false.into());
        }

        // 向前回溯，直到发现 up != low（用于忽略平值区间）
        let mut i = index;
        while i > 0 {
            let prev_up = self.up.get_value(i - 1)?;
            let prev_low = self.low.get_value(i - 1)?;
            if !prev_up.is_equal(&prev_low) {
                break;
            }
            i -= 1;
        }

        let prev_up = self.up.get_value(i - 1)?;
        let prev_low = self.low.get_value(i - 1)?;

        // 判断是否从上方穿越（即前一个 up > low，现在 up < low）
        let crossed = prev_up.is_greater_than(&prev_low);
        Ok(crossed.into()) // ✅ 返回 BoolNum 类型的 T
    }
}

// CrossIndicator 结构体，缓存交叉指标结果
pub struct CrossIndicator<T, S, IU, IL>
where
    T: TrNum + Clone + From<bool> + 'static,
    S: BarSeries<T> + 'static,
    IU: Indicator<Num = T, Output = T, Series = S>,
    IL: Indicator<Num = T, Output = T, Series = S>,
{
    cached: CachedIndicator<T, S, CrossCalculator<T, S, IU, IL>>,
    up: Arc<IU>,
    low: Arc<IL>,
}

impl<T, S, IU, IL> Clone for CrossIndicator<T, S, IU, IL>
where
    T: TrNum + Clone + From<bool> + 'static,
    S: BarSeries<T> + 'static,
    IU: Indicator<Num = T, Output = T, Series = S>,
    IL: Indicator<Num = T, Output = T, Series = S>,
{
    fn clone(&self) -> Self {
        Self {
            cached: self.cached.clone(),
            up: Arc::clone(&self.up),
            low: Arc::clone(&self.low),
        }
    }
}

impl<T, S, IU, IL> CrossIndicator<T, S, IU, IL>
where
    T: TrNum + Clone + From<bool> + 'static,
    S: BarSeries<T> + 'static,
    IU: Indicator<Num = T, Output = T, Series = S>,
    IL: Indicator<Num = T, Output = T, Series = S>,
{
    pub fn new(up: Arc<IU>, low: Arc<IL>) -> Self {
        let calculator = CrossCalculator::new(Arc::clone(&up), Arc::clone(&low));
        let cached = CachedIndicator::new_from_indicator(Arc::clone(&up), calculator);
        Self { cached, up, low }
    }

    pub fn get_up(&self) -> Arc<IU> {
        Arc::clone(&self.up)
    }

    pub fn get_low(&self) -> Arc<IL> {
        Arc::clone(&self.low)
    }
}

impl<T, S, IU, IL> Indicator for CrossIndicator<T, S, IU, IL>
where
    T: TrNum + Clone + From<bool> + 'static,
    S: BarSeries<T> + 'static,
    IU: Indicator<Num = T, Output = T, Series = S>,
    IL: Indicator<Num = T, Output = T, Series = S>,
{
    type Num = T;
    type Output = T;
    type Series = S;

    fn get_value(&self, index: usize) -> Result<T, IndicatorError> {
        self.cached.get_cached_value(index)
    }

    fn bar_series(&self) -> BarSeriesRef<Self::Series> {
        self.cached.bar_series()
    }

    fn count_of_unstable_bars(&self) -> usize {
        0
    }
}

impl<T, S, IU, IL> std::fmt::Debug for CrossIndicator<T, S, IU, IL>
where
    T: TrNum + Clone + From<bool> + 'static,
    S: BarSeries<T> + 'static,
    IU: Indicator<Num = T, Output = T, Series = S> + std::fmt::Debug,
    IL: Indicator<Num = T, Output = T, Series = S> + std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CrossIndicator")
            .field("up", &self.up)
            .field("low", &self.low)
            .finish()
    }
}
