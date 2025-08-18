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
use crate::rule::Rule;
use log::trace;
use std::marker::PhantomData;

/// 基础规则
/// 存放公共工具方法，比如调试日志
pub struct BaseRule<'a, R>
where
    R: Rule<'a>,
{
    class_name: &'static str,
    _marker: PhantomData<&'a R>,
}

impl<'a, R> BaseRule<'a, R>
where
    R: Rule<'a>,
{
    /// 创建基础规则
    pub fn new(class_name: &'static str) -> Self {
        Self {
            class_name,
            _marker: PhantomData,
        }
    }

    /// 记录规则是否满足
    pub fn trace_is_satisfied(&self, index: usize, is_satisfied: bool) {
        trace!(
            "{}#is_satisfied({}): {}",
            self.class_name, index, is_satisfied
        );
    }
}
