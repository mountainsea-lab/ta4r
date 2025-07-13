/// 高性能数值类型，对应ta4j的DoubleNum
#[derive(Debug, Clone, PartialEq)]
pub struct DoubleNum {
    delegate: f64,
}

impl DoubleNum {
    pub const MINUS_ONE: DoubleNum = DoubleNum { delegate: -1.0 };
    pub const ZERO: DoubleNum = DoubleNum { delegate: 0.0 };
    pub const ONE: DoubleNum = DoubleNum { delegate: 1.0 };
    pub const TWO: DoubleNum = DoubleNum { delegate: 2.0 };
    pub const THREE: DoubleNum = DoubleNum { delegate: 3.0 };
    pub const HUNDRED: DoubleNum = DoubleNum { delegate: 100.0 };
    pub const THOUSAND: DoubleNum = DoubleNum { delegate: 1000.0 };

    const EPS: f64 = 0.00001; // 精度阈值，与ta4j保持一致

    /// 对应ta4j的valueOf方法
    pub fn value_of_string(val: &str) -> Self {
        Self {
            delegate: val.parse().unwrap(),
        }
    }

    pub fn value_of_number(val: f64) -> Self {
        Self { delegate: val }
    }
}
