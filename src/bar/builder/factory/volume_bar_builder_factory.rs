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
use crate::bar::base_bar_series::BaseBarSeries;
use crate::bar::builder::volume_bar_builder::VolumeBarBuilder;
use crate::bar::types::{BarBuilderFactory, BarSeries};
use crate::num::TrNum;

/// VolumeBarBuilderFactory - 创建 VolumeBarBuilder 的工厂（单例复用）
#[derive(Debug, Clone)]
pub struct VolumeBarBuilderFactory {
    volume_threshold: i64,
}

impl VolumeBarBuilderFactory {
    pub fn new(volume_threshold: i64) -> Self {
        Self { volume_threshold }
    }
}

impl<T: TrNum + 'static> BarBuilderFactory<T> for VolumeBarBuilderFactory {
    type Series = BaseBarSeries<T>;
    type Builder<'a>
        = VolumeBarBuilder<'a, T, BaseBarSeries<T>>
    where
        Self::Series: 'a;

    fn create_bar_builder<'a>(&self, series: &'a mut Self::Series) -> Self::Builder<'a> {
        let factory = series.num_factory();
        VolumeBarBuilder::new_with_factory(factory, self.volume_threshold.clone()).bind_to(series)
    }
}
