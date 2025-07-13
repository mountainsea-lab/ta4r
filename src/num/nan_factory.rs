// /*!
//  * MIT License
//  *
//  * Copyright (c) 2025 Mountainsea
//  * Based on ta4j (c) 2017–2025 Ta4j Organization & respective authors (see AUTHORS)
//  *
//  * Permission is hereby granted, free of charge, to any person obtaining a copy
//  * of this software and associated documentation files (the "Software"), to deal
//  * in the Software without restriction, including without limitation the rights
//  * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
//  * copies of the Software, and to permit persons to whom the Software is
//  * furnished to do so, subject to the following conditions:
//  *
//  * The above copyright notice and this permission notice shall be included in all
//  * copies or substantial portions of the Software.
//  *
//  * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
//  * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
//  * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
//  * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
//  * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
//  * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
//  * SOFTWARE.
//  */
//
// use crate::num::nan::NaN;
// use crate::num::{NumError, NumFactory, TrNum};
//
// #[derive(Debug, Clone, Copy)]
// pub struct NaNFactory;
//
// impl NumFactory<NaN> for NaNFactory {
//     fn minus_one(&self) -> NaN {
//         NaN
//     }
//     fn zero(&self) -> NaN {
//         NaN
//     }
//     fn one(&self) -> NaN {
//         NaN
//     }
//     fn two(&self) -> NaN {
//         NaN
//     }
//     fn three(&self) -> NaN {
//         NaN
//     }
//     fn hundred(&self) -> NaN {
//         NaN
//     }
//     fn thousand(&self) -> NaN {
//         NaN
//     }
//
//     fn num_of_str(&self, _s: &str) -> Result<NaN, NumError> {
//         Ok(NaN)
//     }
//
//     fn num_of_i64(&self, _val: i64) -> NaN {
//         NaN
//     }
//
//     fn num_of_f64(&self, _number: impl Into<f64>) -> Result<NaN, NumError> {
//         Ok(NaN)
//     }
//
//     fn produces(&self, num: &NaN) -> bool {
//         num.is_nan()
//     }
//
// }
