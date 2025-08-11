/// 交易规则接口（Rule）
pub trait Rule<T>: Send + Sync {
    /// 检查给定索引是否满足该规则（不依赖 TradingRecord）
    fn is_satisfied(&self, index: usize) -> bool {
        self.is_satisfied_with_record(index, None)
    }

    /// 检查给定索引是否满足该规则（可选依赖 TradingRecord）
    fn is_satisfied_with_record(
        &self,
        index: usize,
        trading_record: Option<&dyn TradingRecord<T>>,
    ) -> bool;

    /// 与另一规则做 AND 组合
    fn and<R: Rule<T> + 'static>(self: Arc<Self>, other: Arc<R>) -> Arc<AndRule<T>> {
        Arc::new(AndRule::new(self, other))
    }

    /// 与另一规则做 OR 组合
    fn or<R: Rule<T> + 'static>(self: Arc<Self>, other: Arc<R>) -> Arc<OrRule<T>> {
        Arc::new(OrRule::new(self, other))
    }

    /// 与另一规则做 XOR 组合
    fn xor<R: Rule<T> + 'static>(self: Arc<Self>, other: Arc<R>) -> Arc<XorRule<T>> {
        Arc::new(XorRule::new(self, other))
    }

    /// 逻辑取反
    fn negation(self: Arc<Self>) -> Arc<NotRule<T>> {
        Arc::new(NotRule::new(self))
    }
}