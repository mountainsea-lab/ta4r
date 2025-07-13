use crate::num::nan::NaN;
use crate::num::{Num, NumError, NumFactory};

#[derive(Debug, Clone, Copy)]
pub struct NaNFactory;

impl NumFactory<NaN> for NaNFactory {
    fn minus_one() -> NaN { NaN }
    fn zero() -> NaN { NaN }
    fn one() -> NaN { NaN }
    fn two() -> NaN { NaN }
    fn three() -> NaN { NaN }
    fn hundred() -> NaN { NaN }
    fn thousand() -> NaN { NaN }

    fn from_str(_s: &str) -> Result<NaN, NumError> {
        Ok(NaN)
    }

    fn from_f64(_val: f64) -> Result<NaN, NumError> {
        Ok(NaN)
    }

    fn from_i64(_val: i64) -> NaN {
        NaN
    }

    fn from_number(_number: impl Into<f64>) -> Result<NaN, NumError> {
        Ok(NaN)
    }

    fn produces(&self, num: &NaN) -> bool {
        num.is_nan()
    }
}
