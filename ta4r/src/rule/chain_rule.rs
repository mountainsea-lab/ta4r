use crate::rule::Rule;
use crate::rule::base_rule::BaseRule;
use crate::rule::helper::chain_link::ChainLink;
use std::marker::PhantomData;

/// ChainRule: 初始规则 + 链条规则 + threshold 检查
pub struct ChainRule<'a, R>
where
    R: Rule<'a>,
{
    base: BaseRule<'a, R>,
    initial_rule: R,
    rules_in_chain: Vec<ChainLink<'a, R>>,
    _marker: PhantomData<&'a R>,
}

impl<'a, R> ChainRule<'a, R>
where
    R: Rule<'a>,
{
    /// 创建 ChainRule
    pub fn new(initial_rule: R, chain_links: Vec<ChainLink<'a, R>>) -> Self {
        Self {
            base: BaseRule::new("ChainRule"),
            initial_rule,
            rules_in_chain: chain_links,
            _marker: PhantomData,
        }
    }

    /// 内部日志方法
    fn trace_is_satisfied(&self, index: usize, is_satisfied: bool) {
        self.base.trace_is_satisfied(index, is_satisfied);
    }
}

impl<'a, R> Rule<'a> for ChainRule<'a, R>
where
    R: Rule<'a>,
{
    type Num = R::Num;
    type CostBuy = R::CostBuy;
    type CostSell = R::CostSell;
    type Series = R::Series;
    type TradingRec = R::TradingRec;

    fn is_satisfied_with_record(
        &self,
        index: usize,
        trading_record: Option<&Self::TradingRec>,
    ) -> bool {
        let mut last_rule_was_satisfied_after_bars = 0;
        let mut start_index = index;

        // 检查初始规则
        if !self
            .initial_rule
            .is_satisfied_with_record(index, trading_record)
        {
            self.trace_is_satisfied(index, false);
            return false;
        }
        self.trace_is_satisfied(index, true);

        // 遍历链条规则
        for link in &self.rules_in_chain {
            let mut satisfied_within_threshold = false;
            start_index = start_index.saturating_sub(last_rule_was_satisfied_after_bars);
            last_rule_was_satisfied_after_bars = 0;

            for i in 0..=link.threshold() {
                if start_index < i {
                    break;
                }
                let resulting_index = start_index - i;

                satisfied_within_threshold = link
                    .rule()
                    .is_satisfied_with_record(resulting_index, trading_record);

                if satisfied_within_threshold {
                    break;
                }
                last_rule_was_satisfied_after_bars += 1;
            }

            if !satisfied_within_threshold {
                self.trace_is_satisfied(index, false);
                return false;
            }
        }

        self.trace_is_satisfied(index, true);
        true
    }
}
