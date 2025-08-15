use crate::rule::Rule;
use std::fmt::Debug;
use std::marker::PhantomData;

/// ChainLink: 用于 ChainRule 的单个链接
pub struct ChainLink<'a, R>
where
    R: Rule<'a>,
{
    /// 规则对象
    rule: R,

    /// threshold：规则必须在多少个 bar 内满足（包含当前 bar）
    threshold: usize,
    _marker: PhantomData<&'a ()>,
}

impl<'a, R> ChainLink<'a, R>
where
    R: Rule<'a>,
{
    /// 构造函数
    pub fn new(rule: R, threshold: usize) -> Self {
        Self {
            rule,
            threshold,
            _marker: PhantomData,
        }
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
impl<'a, R> Debug for ChainLink<'a, R>
where
    R: Rule<'a> + Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ChainLink")
            .field("rule", &self.rule)
            .field("threshold", &self.threshold)
            .finish()
    }
}

// 实现 PartialEq，用于比较
impl<'a, R> PartialEq for ChainLink<'a, R>
where
    R: Rule<'a> + PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.threshold == other.threshold && self.rule == other.rule
    }
}

// 实现 Eq
impl<'a, R> Eq for ChainLink<'a, R> where R: Rule<'a> + Eq {}

// 实现 Hash
impl<'a, R> std::hash::Hash for ChainLink<'a, R>
where
    R: Rule<'a> + std::hash::Hash,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.rule.hash(state);
        self.threshold.hash(state);
    }
}
