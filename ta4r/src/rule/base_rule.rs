use crate::rule::Rule;
use log::trace;
use std::marker::PhantomData;

/// 基础规则
/// 存放公共工具方法，比如调试日志
pub struct BaseRule<'a, R>
where
    R: Rule<'a>,
{
    class_name: &'static str,
    _marker: PhantomData<&'a R>,
}

impl<'a, R> BaseRule<'a, R>
where
    R: Rule<'a>,
{
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
