use crate::bar::types::BarSeries;
use crate::indicators::types::{IndicatorError, IndicatorIterator};
use crate::num::TrNum;

mod abstract_indicator;
mod cached_indicator;
mod recursive_cached_indicator;
pub mod types;

mod helpers;
mod numeric;

pub trait Indicator: Clone {
    type Num: TrNum + 'static;

    /// GAT 返回绑定生命周期的系列
    type Series<'a>: BarSeries<'a, Self::Num>
    where
        Self: 'a;

    /// 获取指定 index 处的指标值
    fn get_value(&self, index: usize) -> Result<Self::Num, IndicatorError>;

    /// 返回该指标依赖的 BarSeries 引用
    fn get_bar_series(&self) -> &Self::Series<'_>;

    /// 返回在多少根 bar 之前该指标是不稳定的（计算值不可靠）
    fn get_count_of_unstable_bars(&self) -> usize;

    /// 判断 index 处是否稳定
    fn is_stable_at(&self, index: usize) -> bool {
        index >= self.get_count_of_unstable_bars()
    }

    /// 当前 series 是否已达到稳定计算条件
    fn is_stable(&self) -> bool {
        self.get_bar_series().get_bar_count() >= self.get_count_of_unstable_bars()
    }

    /// 提供一个便捷迭代器（模拟 Java 的 stream()）
    fn iter(&self) -> IndicatorIterator<Self>
    where
        Self: Sized,
    {
        match (
            self.get_bar_series().get_begin_index(),
            self.get_bar_series().get_end_index(),
        ) {
            (Some(begin), Some(end)) => IndicatorIterator {
                indicator: self,
                index: begin,
                end,
            },
            _ => IndicatorIterator {
                indicator: self,
                index: 1, // 空迭代器起始大于结束
                end: 0,
            },
        }
    }
}
