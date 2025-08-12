use crate::analysis::CostModel;
use crate::analysis::cost::CostContext;
use crate::analysis::cost::fixed_transaction_cost_model::FixedTransactionCostModel;
use crate::num::TrNum;

// ZeroCostModel，组合FixedTransactionCostModel
#[derive(Debug, Clone)]
pub struct ZeroCostModel<T: TrNum + 'static> {
    inner: FixedTransactionCostModel<T>,
}

impl<T: TrNum + 'static> ZeroCostModel<T> {
    pub fn new() -> Self {
        Self {
            inner: FixedTransactionCostModel::new(T::zero()),
        }
    }
}

impl<T: TrNum + 'static> CostModel<T> for ZeroCostModel<T> {
    fn calculate_with_index(&self, ctx: &CostContext<T>) -> T {
        self.inner.calculate_with_index(ctx)
    }

    fn calculate_position(&self, ctx: &CostContext<T>) -> T {
        self.inner.calculate_position(ctx)
    }

    fn calculate_trade(&self, price: T, amount: T) -> T {
        self.inner.calculate_trade(price, amount)
    }

    fn equals(&self, other: &Self) -> bool {
        self.inner.equals(&other.inner)
    }
}
