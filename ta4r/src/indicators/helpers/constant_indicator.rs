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
use crate::bar::types::BarSeries;
use crate::indicators::Indicator;
use crate::indicators::abstract_indicator::BaseIndicator;
use crate::indicators::types::IndicatorError;
use crate::num::TrNum;

/// 常数指标：所有索引返回同一个值
pub struct ConstantIndicator<'a, T, S>
where
    T: TrNum + 'static,
    S: BarSeries<'a, T>,
{
    base: BaseIndicator<'a, T, S>,
    value: T,
}

impl<'a, T, S> Clone for ConstantIndicator<'a, T, S>
where
    T: TrNum + Clone + 'static,
    S: BarSeries<'a, T>,
{
    fn clone(&self) -> Self {
        Self {
            base: self.base.clone(),
            value: self.value.clone(),
        }
    }
}

impl<'a, T, S> ConstantIndicator<'a, T, S>
where
    T: TrNum + 'static,
    S: BarSeries<'a, T>,
{
    pub fn new(series: &'a S, value: T) -> Self {
        Self {
            base: BaseIndicator::new(series),
            value,
        }
    }
}

impl<'a, T, S> Indicator for ConstantIndicator<'a, T, S>
where
    T: TrNum + 'static,
    S: for<'any> BarSeries<'any, T>,
{
    type Num = T;
    type Output = T;
    type Series<'b>
        = S
    where
        Self: 'b;
    fn get_value(&self, _index: usize) -> Result<T, IndicatorError> {
        Ok(self.value.clone())
    }

    fn get_bar_series(&self) -> &Self::Series<'_> {
        self.base.get_bar_series()
    }

    fn get_count_of_unstable_bars(&self) -> usize {
        0
    }
}
