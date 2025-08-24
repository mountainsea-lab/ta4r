use crate::TradingRecord;
use crate::analysis::CostModel;
use crate::bar::types::BarSeries;
use crate::num::TrNum;
use crate::position::Position;
use crate::trade::{Trade, TradeType};
use std::fmt;
use std::marker::PhantomData;
use std::sync::Arc;

/// BaseTradingRecord：交易记录实现
/// 基础交易记录实现（Arc 优化版）
///
/// - 使用 [`Arc`] 包装 `Trade` / `Position`，降低 clone 成本
/// - 内部状态修改使用 [`Arc::make_mut`]，确保写时复制
/// - 与 [`TradingRecord`] trait 完全兼容
pub struct BaseTradingRecord<T, CM, HM, S>
where
    T: TrNum + 'static,
    CM: CostModel<T> + Clone,
    HM: CostModel<T> + Clone,
    S: BarSeries<T> + 'static,
{
    /// 交易记录名称
    name: Option<String>,

    /// 记录开始索引
    start_index: Option<usize>,

    /// 记录结束索引
    end_index: Option<usize>,

    /// 所有交易，使用 Arc 避免多次 clone
    trades: Vec<Arc<Trade<T, CM, S>>>,

    /// 买入交易
    buy_trades: Vec<Arc<Trade<T, CM, S>>>,

    /// 卖出交易
    sell_trades: Vec<Arc<Trade<T, CM, S>>>,

    /// 入场交易
    entry_trades: Vec<Arc<Trade<T, CM, S>>>,

    /// 出场交易
    exit_trades: Vec<Arc<Trade<T, CM, S>>>,

    /// 交易起始方向
    starting_type: TradeType,

    /// 已关闭的持仓
    positions: Vec<Arc<Position<T, CM, HM, S>>>,

    /// 当前未关闭持仓
    current_position: Arc<Position<T, CM, HM, S>>,

    /// 交易成本模型
    transaction_cost_model: CM,

    /// 持有成本模型
    holding_cost_model: HM,

    /// PhantomData 标记 BarSeries 类型
    _phantom: PhantomData<S>,
}

impl<T, CM, HM, S> BaseTradingRecord<T, CM, HM, S>
where
    T: TrNum + 'static,
    CM: CostModel<T> + Clone,
    HM: CostModel<T> + Clone,
    S: BarSeries<T> + 'static,
{
    /// 默认价格为 1
    fn default_price() -> T {
        T::one()
    }

    /// 默认数量为 1
    fn default_amount() -> T {
        T::one()
    }

    /// 构造函数（默认 BUY 起始类型，零成本模型）
    pub fn new_default(transaction_cost_model: CM, holding_cost_model: HM) -> Self {
        Self::new_with_type(
            TradeType::Buy,
            None,
            None,
            transaction_cost_model,
            holding_cost_model,
        )
    }

    /// 带起始类型的构造函数
    pub fn new_with_type(
        starting_type: TradeType,
        start_index: Option<usize>,
        end_index: Option<usize>,
        transaction_cost_model: CM,
        holding_cost_model: HM,
    ) -> Self {
        let current_position = Arc::new(Position::new(
            starting_type,
            transaction_cost_model.clone(),
            holding_cost_model.clone(),
        ));

        Self {
            name: None,
            start_index,
            end_index,
            trades: Vec::new(),
            buy_trades: Vec::new(),
            sell_trades: Vec::new(),
            entry_trades: Vec::new(),
            exit_trades: Vec::new(),
            starting_type,
            positions: Vec::new(),
            current_position,
            transaction_cost_model,
            holding_cost_model,
            _phantom: PhantomData,
        }
    }

    /// 设置交易记录名称
    pub fn with_name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    /// 记录交易，Arc 优化避免多次 clone
    fn record_trade(&mut self, trade: Trade<T, CM, S>, is_entry: bool) {
        let trade_arc = Arc::new(trade);

        // 根据是否入场交易记录
        if is_entry {
            self.entry_trades.push(Arc::clone(&trade_arc));
        } else {
            self.exit_trades.push(Arc::clone(&trade_arc));
        }

        // 所有交易
        self.trades.push(Arc::clone(&trade_arc));

        // 按交易类型分类
        match trade_arc.trade_type {
            TradeType::Buy => self.buy_trades.push(Arc::clone(&trade_arc)),
            TradeType::Sell => self.sell_trades.push(Arc::clone(&trade_arc)),
        }

        // 如果当前持仓已关闭，保存并创建新持仓
        if self.current_position.is_closed() {
            self.positions.push(Arc::clone(&self.current_position)); // ✅ Arc clone
            self.current_position = Arc::new(Position::new(
                self.starting_type,
                self.transaction_cost_model.clone(),
                self.holding_cost_model.clone(),
            ));
        }
    }

    /// 使用日志宏打印交易记录摘要和交易明细
    /// - 超过 20 条只显示前 10 条和后 10 条
    /// - trades 切片遍历，无 Arc clone
    pub fn log_trading_record(&self) {
        let total_trades = self.trades.len();
        let total_positions = self.positions.len();

        log::info!(
            "TradingRecord(name={}, trades={}, positions={})",
            self.name.as_deref().unwrap_or(""),
            total_trades,
            total_positions
        );

        let show_limit = 10;
        let trades: &[Arc<Trade<T, CM, S>>] = &self.trades;

        if total_trades <= 2 * show_limit {
            for (i, trade) in trades.iter().enumerate() {
                log::debug!("  Trade[{}]: {:?}", i, trade);
            }
        } else {
            for (i, trade) in trades.iter().take(show_limit).enumerate() {
                log::debug!("  Trade[{}]: {:?}", i, trade);
            }
            log::debug!("  ... {} trades omitted ...", total_trades - 2 * show_limit);
            for (i, trade) in trades.iter().rev().take(show_limit).rev().enumerate() {
                let idx = total_trades - show_limit + i;
                log::debug!("  Trade[{}]: {:?}", idx, trade);
            }
        }
    }
}

impl<T, CM, HM, S> TradingRecord<T, CM, HM, S> for BaseTradingRecord<T, CM, HM, S>
where
    T: TrNum + 'static,
    CM: CostModel<T> + Clone,
    HM: CostModel<T> + Clone,
    S: BarSeries<T> + 'static,
{
    fn starting_type(&self) -> TradeType {
        self.starting_type
    }
    fn name(&self) -> &str {
        self.name.as_deref().unwrap_or("")
    }

    /// 执行操作，使用默认价格和数量
    fn operate(&mut self, index: usize) {
        self.operate_with_price_amount(index, Self::default_price(), Self::default_amount());
    }

    /// 执行操作，指定价格和数量
    fn operate_with_price_amount(&mut self, index: usize, price: T, amount: T) {
        let is_entry = self.current_position.is_new();

        // ✅ Arc::make_mut 修改当前持仓
        if let Some(trade) = Arc::make_mut(&mut self.current_position).operate(index, price, amount)
        {
            self.record_trade(trade, is_entry);
        }
    }

    fn enter(&mut self, index: usize) -> bool {
        self.enter_with_price_amount(index, Self::default_price(), Self::default_amount())
    }

    fn enter_with_price_amount(&mut self, index: usize, price: T, amount: T) -> bool {
        if self.current_position.is_new() {
            self.operate_with_price_amount(index, price, amount);
            true
        } else {
            false
        }
    }

    fn exit(&mut self, index: usize) -> bool {
        self.exit_with_price_amount(index, Self::default_price(), Self::default_amount())
    }

    fn exit_with_price_amount(&mut self, index: usize, price: T, amount: T) -> bool {
        if self.current_position.is_opened() {
            self.operate_with_price_amount(index, price, amount);
            true
        } else {
            false
        }
    }

    fn is_closed(&self) -> bool {
        self.current_position.is_closed()
    }
    fn transaction_cost_model(&self) -> &CM {
        &self.transaction_cost_model
    }
    fn holding_cost_model(&self) -> &HM {
        &self.holding_cost_model
    }
    fn positions(&self) -> &[Arc<Position<T, CM, HM, S>>] {
        // ✅ 改成 Arc
        &self.positions
    }
    fn current_position(&self) -> &Arc<Position<T, CM, HM, S>> {
        // ✅ 改成 Arc
        &self.current_position
    }
    fn trades(&self) -> &[Arc<Trade<T, CM, S>>] {
        &self.trades
    }
    fn last_trade(&self) -> Option<&Arc<Trade<T, CM, S>>> {
        self.trades.last()
    }

    fn last_trade_of_type(&self, trade_type: TradeType) -> Option<&Arc<Trade<T, CM, S>>> {
        match trade_type {
            TradeType::Buy => self.buy_trades.last(),
            TradeType::Sell => self.sell_trades.last(),
        }
    }

    fn last_entry(&self) -> Option<&Arc<Trade<T, CM, S>>> {
        self.entry_trades.last()
    }
    fn last_exit(&self) -> Option<&Arc<Trade<T, CM, S>>> {
        self.exit_trades.last()
    }
    fn start_index(&self) -> Option<usize> {
        self.start_index
    }
    fn end_index(&self) -> Option<usize> {
        self.end_index
    }
}

/// Default 构造函数，使用 Buy 起始和 Default 成本模型
impl<T, CM, HM, S> Default for BaseTradingRecord<T, CM, HM, S>
where
    T: TrNum + 'static,
    CM: CostModel<T> + Clone + Default,
    HM: CostModel<T> + Clone + Default,
    S: BarSeries<T> + 'static,
{
    fn default() -> Self {
        Self::new_with_type(TradeType::Buy, None, None, CM::default(), HM::default())
    }
}
