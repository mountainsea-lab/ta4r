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
use crate::bar::builder::mocks::mock_bar_builder::MockBarBuilder;
use crate::bar::types::{BarBuilderFactory, BarSeries};
use crate::num::TrNum;
use std::marker::PhantomData;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct MockBarBuilderFactory<T: TrNum> {
    _phantom: PhantomData<T>,
}

impl<T: TrNum> Default for MockBarBuilderFactory<T> {
    fn default() -> Self {
        Self {
            _phantom: PhantomData,
        }
    }
}

impl<T: TrNum> MockBarBuilderFactory<T> {
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData,
        }
    }
}

impl<T: TrNum + 'static> BarBuilderFactory<T> for MockBarBuilderFactory<T> {
    type Series = BaseBarSeries<T>;
    // GAT 的合法实现写法（注意这里声明了一个 GAT）
    type Builder<'a>
        = MockBarBuilder<T, Self::Series>
    where
        Self::Series: 'a;

    fn create_bar_builder<'a>(&self, series: &'a mut Self::Series) -> Self::Builder<'a> {
        let factory = series.num_factory();
        MockBarBuilder::new_with_factory(factory).bind_to(series)
    }

    fn create_bar_builder_shared(
        &self,
        num_factory: Arc<T::Factory>,
        shared_series: Arc<Mutex<Self::Series>>,
    ) -> Self::Builder<'static>
    where
        Self::Series: 'static,
    {
        // 注意使用调用方不能再调用前就持有锁，否则后续调用链shared_series锁操作会死锁
        // let factory = {
        //     // 临时持锁只为获取num_factory(Arc)，立即释放锁
        //     let locked = shared_series.lock().unwrap();
        //     locked.num_factory()
        // };

        MockBarBuilder::new_with_factory(num_factory).bind_shared(shared_series)
    }
}
