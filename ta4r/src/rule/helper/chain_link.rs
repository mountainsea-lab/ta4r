use crate::rule::Rule;
use std::fmt::Debug;

/// ChainLink: 用于 ChainRule 的单个链接
pub struct ChainLink<R>
where
    R: Rule,
{
    /// 规则对象
    rule: R,

    /// threshold：规则必须在多少个 bar 内满足（包含当前 bar）
    threshold: usize,
}

impl<R> ChainLink<R>
where
    R: Rule,
{
    /// 构造函数
    pub fn new(rule: R, threshold: usize) -> Self {
        Self { rule, threshold }
    }

    /// 获取 rule
    pub fn rule(&self) -> &R {
        &self.rule
    }

    /// 设置 rule
    pub fn set_rule(&mut self, rule: R) {
        self.rule = rule;
    }

    /// 获取 threshold
    pub fn threshold(&self) -> usize {
        self.threshold
    }

    /// 设置 threshold
    pub fn set_threshold(&mut self, threshold: usize) {
        self.threshold = threshold;
    }
}

// 实现 Debug，用于打印
impl<R> Debug for ChainLink<R>
where
    R: Rule + Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ChainLink")
            .field("rule", &self.rule)
            .field("threshold", &self.threshold)
            .finish()
    }
}

// 实现 PartialEq，用于比较
impl<R> PartialEq for ChainLink<R>
where
    R: Rule + PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.threshold == other.threshold && self.rule == other.rule
    }
}

// 实现 Eq
impl<R> Eq for ChainLink<R> where R: Rule + Eq {}

// 实现 Hash
impl<R> std::hash::Hash for ChainLink<R>
where
    R: Rule + std::hash::Hash,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.rule.hash(state);
        self.threshold.hash(state);
    }
}
