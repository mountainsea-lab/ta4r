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

use std::cmp::Ordering;
use std::fmt;

/// Representation of `f64`. High performance, lower precision.
///
/// It uses a precision of up to `EPS` decimal places.
///
/// # Note
/// The internal value should never become NaN. No self NaN checks are provided.
#[derive(Clone, Copy)]
pub struct DoubleNum {
    delegate: f64,
}

impl DoubleNum {
    /// Precision: epsilon for floating-point comparison
    const EPS: f64 = 0.00001;

    /// Predefined constants for convenience
    pub const MINUS_ONE: DoubleNum = DoubleNum { delegate: -1.0 };
    pub const ZERO: DoubleNum = DoubleNum { delegate: 0.0 };
    pub const ONE: DoubleNum = DoubleNum { delegate: 1.0 };
    pub const TWO: DoubleNum = DoubleNum { delegate: 2.0 };
    pub const THREE: DoubleNum = DoubleNum { delegate: 3.0 };
    pub const HUNDRED: DoubleNum = DoubleNum { delegate: 100.0 };
    pub const THOUSAND: DoubleNum = DoubleNum { delegate: 1000.0 };

    /// Constructs a new DoubleNum from a f64
    pub fn new(val: f64) -> Self {
        Self { delegate: val }
    }

    /// Constructs a new DoubleNum from a &str
    ///
    /// # Panics
    /// Panics if the string cannot be parsed as f64.
    pub fn from_str(val: &str) -> Self {
        Self {
            delegate: val.parse::<f64>().expect("Failed to parse string to f64"),
        }
    }

    /// Returns the internal f64 value
    pub fn value(&self) -> f64 {
        self.delegate
    }

    /// Returns true if the value is NaN
    pub fn is_nan(&self) -> bool {
        self.delegate.is_nan()
    }

    /// Returns true if the value is zero
    pub fn is_zero(&self) -> bool {
        self.delegate == 0.0
    }

    /// Returns true if the value is positive
    pub fn is_positive(&self) -> bool {
        self.delegate > 0.0
    }

    /// Returns true if the value is positive or zero
    pub fn is_positive_or_zero(&self) -> bool {
        self.delegate >= 0.0
    }

    /// Returns true if the value is negative
    pub fn is_negative(&self) -> bool {
        self.delegate < 0.0
    }

    /// Returns true if the value is negative or zero
    pub fn is_negative_or_zero(&self) -> bool {
        self.delegate <= 0.0
    }

    /// Adds two DoubleNum values, returns NaN if any operand is NaN
    pub fn plus(&self, other: DoubleNum) -> DoubleNum {
        if self.is_nan() || other.is_nan() {
            DoubleNum::nan()
        } else {
            DoubleNum::new(self.delegate + other.delegate)
        }
    }

    /// Subtracts two DoubleNum values, returns NaN if any operand is NaN
    pub fn minus(&self, other: DoubleNum) -> DoubleNum {
        if self.is_nan() || other.is_nan() {
            DoubleNum::nan()
        } else {
            DoubleNum::new(self.delegate - other.delegate)
        }
    }

    /// Multiplies two DoubleNum values, returns NaN if any operand is NaN
    pub fn multiplied_by(&self, other: DoubleNum) -> DoubleNum {
        if self.is_nan() || other.is_nan() {
            DoubleNum::nan()
        } else {
            DoubleNum::new(self.delegate * other.delegate)
        }
    }

    /// Divides two DoubleNum values, returns NaN if divisor is zero or NaN
    pub fn divided_by(&self, other: DoubleNum) -> DoubleNum {
        if other.is_nan() || other.is_zero() {
            DoubleNum::nan()
        } else {
            DoubleNum::new(self.delegate / other.delegate)
        }
    }

    /// Calculates remainder of division, returns NaN if divisor is NaN
    pub fn remainder(&self, other: DoubleNum) -> DoubleNum {
        if other.is_nan() {
            DoubleNum::nan()
        } else {
            DoubleNum::new(self.delegate % other.delegate)
        }
    }

    /// Returns floor of the value
    pub fn floor(&self) -> DoubleNum {
        DoubleNum::new(self.delegate.floor())
    }

    /// Returns ceil of the value
    pub fn ceil(&self) -> DoubleNum {
        DoubleNum::new(self.delegate.ceil())
    }

    /// Returns the value raised to an integer power
    pub fn pow_i32(&self, n: i32) -> DoubleNum {
        DoubleNum::new(self.delegate.powi(n))
    }

    /// Returns the value raised to a DoubleNum power
    pub fn pow(&self, n: DoubleNum) -> DoubleNum {
        DoubleNum::new(self.delegate.powf(n.delegate))
    }

    /// Returns the square root, NaN if value is negative
    pub fn sqrt(&self) -> DoubleNum {
        if self.delegate < 0.0 {
            DoubleNum::nan()
        } else {
            DoubleNum::new(self.delegate.sqrt())
        }
    }

    /// Returns absolute value
    pub fn abs(&self) -> DoubleNum {
        DoubleNum::new(self.delegate.abs())
    }

    /// Returns negated value
    pub fn negate(&self) -> DoubleNum {
        DoubleNum::new(-self.delegate)
    }

    /// Returns true if equal within EPS precision, false otherwise
    pub fn is_equal(&self, other: DoubleNum) -> bool {
        if self.is_nan() || other.is_nan() {
            false
        } else {
            (self.delegate - other.delegate).abs() < DoubleNum::EPS
        }
    }

    /// Returns the natural logarithm, NaN if value <= 0
    pub fn log(&self) -> DoubleNum {
        if self.delegate <= 0.0 {
            DoubleNum::nan()
        } else {
            DoubleNum::new(self.delegate.ln())
        }
    }

    /// Returns true if greater than other
    pub fn is_greater_than(&self, other: DoubleNum) -> bool {
        if self.is_nan() || other.is_nan() {
            false
        } else {
            self.delegate > other.delegate
        }
    }

    /// Returns true if greater than or equal to other
    pub fn is_greater_than_or_equal(&self, other: DoubleNum) -> bool {
        if self.is_nan() || other.is_nan() {
            false
        } else {
            self.delegate >= other.delegate
        }
    }

    /// Returns true if less than other
    pub fn is_less_than(&self, other: DoubleNum) -> bool {
        if self.is_nan() || other.is_nan() {
            false
        } else {
            self.delegate < other.delegate
        }
    }

    /// Returns true if less than or equal to other
    pub fn is_less_than_or_equal(&self, other: DoubleNum) -> bool {
        if self.is_nan() || other.is_nan() {
            false
        } else {
            self.delegate <= other.delegate
        }
    }

    /// Returns the minimum of self and other, NaN if other is NaN
    pub fn min(&self, other: DoubleNum) -> DoubleNum {
        if other.is_nan() {
            DoubleNum::nan()
        } else {
            DoubleNum::new(self.delegate.min(other.delegate))
        }
    }

    /// Returns the maximum of self and other, NaN if other is NaN
    pub fn max(&self, other: DoubleNum) -> DoubleNum {
        if other.is_nan() {
            DoubleNum::nan()
        } else {
            DoubleNum::new(self.delegate.max(other.delegate))
        }
    }

    /// Returns a NaN DoubleNum
    pub fn nan() -> DoubleNum {
        DoubleNum::new(f64::NAN)
    }
}

impl PartialEq for DoubleNum {
    fn eq(&self, other: &Self) -> bool {
        if self.is_nan() && other.is_nan() {
            true
        } else {
            (self.delegate - other.delegate).abs() < DoubleNum::EPS
        }
    }
}

impl Eq for DoubleNum {}

impl PartialOrd for DoubleNum {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.is_nan() || other.is_nan() {
            None
        } else {
            self.delegate.partial_cmp(&other.delegate)
        }
    }
}

impl Ord for DoubleNum {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_or(Ordering::Equal)
    }
}

impl fmt::Display for DoubleNum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.delegate)
    }
}

impl fmt::Debug for DoubleNum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "DoubleNum({})", self.delegate)
    }
}

#[cfg(test)]
mod tests {
    use super::DoubleNum;

    #[test]
    fn test_basic_arithmetic() {
        let a = DoubleNum::new(5.0);
        let b = DoubleNum::new(2.0);
        assert_eq!(a.plus(b), DoubleNum::new(7.0));
        assert_eq!(a.minus(b), DoubleNum::new(3.0));
        assert_eq!(a.multiplied_by(b), DoubleNum::new(10.0));
        assert_eq!(a.divided_by(b), DoubleNum::new(2.5));
        assert!(a.divided_by(DoubleNum::ZERO).is_nan());
    }

    #[test]
    fn test_comparisons() {
        let a = DoubleNum::new(5.0);
        let b = DoubleNum::new(5.0 + DoubleNum::EPS / 2.0);
        assert!(a.is_equal(b));
        assert!(a.is_less_than(DoubleNum::new(6.0)));
        assert!(a.is_greater_than(DoubleNum::new(4.0)));
    }

    #[test]
    fn test_nan() {
        let nan = DoubleNum::nan();
        assert!(nan.is_nan());
        assert_eq!(nan.plus(DoubleNum::ONE), nan);
    }
}
