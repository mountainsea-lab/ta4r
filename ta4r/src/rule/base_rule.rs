use log::trace;
use std::marker::PhantomData;

/// 基础规则
/// 存放公共工具方法，比如调试日志
pub struct BaseRule<'a, N, CM, HM, S, TR> {
    class_name: &'static str,
    _marker: PhantomData<(&'a N, CM, HM, S, TR)>,
}

impl<'a, N, CM, HM, S, TR> BaseRule<'a, N, CM, HM, S, TR> {
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
