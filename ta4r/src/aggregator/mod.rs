pub mod types;

use crate::bar::types::Bar;
use crate::num::TrNum;

pub trait BarAggregator<T: TrNum + 'static> {
    type Bar: Bar<T>;

    /// 将输入的一批 Bar 聚合为新的 Bar 序列
    /// 传入是对输入 Bar 的借用切片
    fn aggregate(&self, bars: &[Self::Bar]) -> Vec<Self::Bar>;
}
