use crate::bar::types::BarSeries;
use crate::num::TrNum;

pub struct AbstractIndicator<'a, T, S>
where
    T: TrNum + 'static,
    S: BarSeries<'a, T>,
{
    // 持有的 BarSeries 引用
    bar_series: &'a S,

    // 日志记录器通常 Rust 里使用宏，不用存储字段，这里不必定义log字段
    // 如果需要日志，可用 log crate 等宏调用

    // 可能会在后续加入其他公共字段，比如缓存大小、状态等
}

impl<'a, T, S> AbstractIndicator<'a, T, S>
where
    T: TrNum + 'static,
    S: BarSeries<'a, T>,
{
    /// 构造函数
    pub fn new(bar_series: &'a S) -> Self {
        Self { bar_series }
    }

    /// 暴露 bar_series 引用
    pub fn bar_series(&self) -> &'a S {
        self.bar_series
    }

    // 后续可以继续添加基础的辅助方法，比如
    // fn to_string(&self) -> String { ... }
}
