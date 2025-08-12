use crate::analysis::cost::cost_model::CostContext;
use crate::bar::types::BarSeries;
use crate::num::TrNum;

pub mod fixed_transaction_cost_model;
pub mod zero_cost_model;

/// 用于封装计算成本所需的所有参数，后续方便扩展 比如解耦: cost与Position的耦合
pub struct CostContext<T: TrNum + 'static> {
    pub entry_price: T,
    pub amount: T,
    pub entry_index: Option<usize>, // 可选，因为有的方法不需要索引
    pub final_index: Option<usize>, // 指定索引
    pub is_closed: bool,
    // 后续如果有更多状态，直接加字段即可
    // pub extra_fee: T,
    // pub timestamp: u64,
    // ...

}
