use crate::IndicatorFactory;
use ta4r::bar::types::BarSeries;
use ta4r::indicators::Indicator;
use ta4r::indicators::types::IndicatorError;
use ta4r::num::decimal_num_factory::DecimalNumFactory;
use ta4r::num::double_num_factory::DoubleNumFactory;
use ta4r::num::{NumFactory, TrNum};

/// 运行时选择的数值类型
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NumKind {
    Double,
    Decimal,
}

/// 包装所有数值工厂的枚举（不能 Copy）
#[derive(Clone, Debug)]
pub enum NumFactoryEnum {
    Double(DoubleNumFactory),
    Decimal(DecimalNumFactory),
}

impl NumKind {
    pub fn name(&self) -> &'static str {
        match self {
            NumKind::Double => "DoubleNum",
            NumKind::Decimal => "DecimalNum",
        }
    }

    /// 根据 NumKind 返回对应工厂（枚举包装）
    pub fn num_factory(&self) -> NumFactoryEnum {
        match self {
            NumKind::Double => NumFactoryEnum::Double(DoubleNumFactory::default()),
            NumKind::Decimal => NumFactoryEnum::Decimal(DecimalNumFactory::default()),
        }
    }
}

impl NumFactoryEnum {
    // pub fn one<T: TrNum>(&self) -> T {
    //     match self {
    //         NumFactoryEnum::Double(f) => f.one().as_ref().clone(),
    //         NumFactoryEnum::Decimal(f) => f.one().as_ref().clone(),
    //     }
    // }

    // pub fn num_of_f64<T: TrNum>(&self, val: f64) -> T {
    //     match self {
    //         NumFactoryEnum::Double(f) => f.num_of_f64(val),
    //         NumFactoryEnum::Decimal(f) => f.num_of_f64(val),
    //     }
    // }

    // todo 其他工厂方法按需添加
}

pub struct TestContext<'a, T, S, I, F>
where
    T: TrNum + 'static,
    S: BarSeries<'a, T>,
    I: Indicator<Num = T> + Clone + 'static,
    F: IndicatorFactory<'a, T, S, I>,
{
    pub kind: NumKind,
    pub factory: F,
    pub phantom: std::marker::PhantomData<(&'a T, S, I)>,
}

impl<'a, T, S, I, F> TestContext<'a, T, S, I, F>
where
    T: TrNum + 'static,
    S: BarSeries<'a, T>,
    I: Indicator<Num = T> + Clone + 'static,
    F: IndicatorFactory<'a, T, S, I>,
{
    pub fn new(kind: NumKind, factory: F) -> Self {
        Self {
            kind,
            factory,
            phantom: std::marker::PhantomData,
        }
    }
    pub fn build_indicator(&self, series: &'a S, params: &[usize]) -> Result<I, IndicatorError> {
        self.factory.build(series, params)
    }
}

// 帮助函数：断言数字相等，按需扩展（浮点等）
fn assert_num_eq<T: TrNum>(expected: f64, actual: T) {
    let actual_f64 = actual.to_f64().unwrap_or(f64::NAN);
    assert!(
        (expected - actual_f64).abs() < 1e-6,
        "expected: {}, actual: {:?}",
        expected,
        actual
    );
}
