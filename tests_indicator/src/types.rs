use crate::IndicatorFactory;
use ta4r::bar::types::BarSeries;
use ta4r::indicators::Indicator;
use ta4r::indicators::types::IndicatorError;
use ta4r::num::TrNum;

/// 运行时选择的数值类型
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NumKind {
    Double,
    Decimal,
}

impl NumKind {
    pub fn name(&self) -> &'static str {
        match self {
            NumKind::Double => "DoubleNum",
            NumKind::Decimal => "DecimalNum",
        }
    }
}

pub struct TestContext<T, S, I, F>
where
    T: TrNum + 'static,
    S: BarSeries<T>,
    I: Indicator<Num = T> + Clone + 'static,
    F: IndicatorFactory<T, S, I>,
{
    pub kind: NumKind,
    pub factory: F,
    pub phantom: std::marker::PhantomData<(T, S, I)>,
}

impl<T, S, I, F> TestContext<T, S, I, F>
where
    T: TrNum + 'static,
    S: BarSeries<T>,
    I: Indicator<Num = T> + Clone + 'static,
    F: IndicatorFactory<T, S, I>,
{
    pub fn new(kind: NumKind, factory: F) -> Self {
        Self {
            kind,
            factory,
            phantom: std::marker::PhantomData,
        }
    }
    pub fn build_indicator(&self, series: &S, params: &[usize]) -> Result<I, IndicatorError> {
        self.factory.build(series, params)
    }
}

// 帮助函数：断言数字相等，按需扩展（浮点等）
#[warn(dead_code)]
pub fn assert_num_eq<T: TrNum>(expected: f64, actual: T) {
    // 转换 actual 为 f64
    let actual_f64 = actual.to_f64().unwrap_or(f64::NAN);

    // 判断是否都是 NaN，如果是则视为相等
    if expected.is_nan() && actual_f64.is_nan() {
        return; // 都是 NaN，认为相等
    }

    // 判断是否相等（允许一个很小的误差）
    assert!(
        (expected - actual_f64).abs() < 1e-6,
        "expected: {}, actual: {:?} (as f64: {})",
        expected,
        actual,
        actual_f64
    );
}
