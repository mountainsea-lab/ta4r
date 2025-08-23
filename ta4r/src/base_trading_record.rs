use std::fmt;
use std::marker::PhantomData;
use crate::analysis::CostModel;
use crate::bar::types::BarSeries;
use crate::num::TrNum;
use crate::position::Position;
use crate::trade::{Trade, TradeType};
use crate::TradingRecord;

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

    /// 所有交易
    trades: Vec<Trade<T, CM, S>>,

    /// 买入交易
    buy_trades: Vec<Trade<T, CM, S>>,

    /// 卖出交易
    sell_trades: Vec<Trade<T, CM, S>>,

    /// 入场交易
    entry_trades: Vec<Trade<T, CM, S>>,

    /// 出场交易
    exit_trades: Vec<Trade<T, CM, S>>,

    /// 交易起始方向
    starting_type: TradeType,

    /// 已关闭的持仓
    positions: Vec<Position<T, CM, HM, S>>,

    /// 当前未关闭持仓
    current_position: Position<T, CM, HM, S>,

    /// 交易成本模型
    transaction_cost_model: CM,

    /// 持有成本模型
    holding_cost_model: HM,

    _phantom: PhantomData<S>,
}

impl<T, CM, HM, S> BaseTradingRecord<T, CM, HM, S>
where
    T: TrNum + 'static,
    CM: CostModel<T> + Clone,
    HM: CostModel<T> + Clone,
    S: BarSeries<T> + 'static,
{
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
        let current_position =
            Position::new(starting_type, transaction_cost_model.clone(), holding_cost_model.clone());

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

    /// 设置名字
    pub fn with_name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    /// 记录交易
    fn record_trade(&mut self, trade: Trade<T, CM, S>, is_entry: bool) {
        if is_entry {
            self.entry_trades.push(trade.clone());
        } else {
            self.exit_trades.push(trade.clone());
        }

        self.trades.push(trade.clone());
        match trade.trade_type {
            TradeType::Buy => self.buy_trades.push(trade.clone()),
            TradeType::Sell => self.sell_trades.push(trade.clone()),
        }

        if self.current_position.is_closed() {
            self.positions.push(self.current_position.clone());
            self.current_position = Position::new(
                self.starting_type,
                self.transaction_cost_model.clone(),
                self.holding_cost_model.clone(),
            );
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
    fn get_starting_type(&self) -> TradeType {
        self.starting_type
    }

    fn get_name(&self) -> &str {
        self.name.as_deref().unwrap_or("")
    }

    fn operate(&mut self, index: usize) {
        // 用默认价格和数量，这里先简单用 1
        self.operate_with_price_amount(index, T::one(), T::one());
    }

    fn operate_with_price_amount(&mut self, index: usize, price: T, amount: T) {
        if self.current_position.is_closed() {
            panic!("Current position should not be closed");
        }
        let new_trade_will_be_entry = self.current_position.is_new();
        let new_trade = self.current_position.operate(index, price, amount);
        self.record_trade(new_trade, new_trade_will_be_entry);
    }

    fn enter(&mut self, index: usize) -> bool {
        self.enter_with_price_amount(index, T::one(), T::one())
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
        self.exit_with_price_amount(index, T::one(), T::one())
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

    fn get_transaction_cost_model(&self) -> &CM {
        &self.transaction_cost_model
    }

    fn get_holding_cost_model(&self) -> &HM {
        &self.holding_cost_model
    }

    fn get_positions(&self) -> &[Position<T, CM, HM, S>] {
        &self.positions
    }

    fn get_current_position(&self) -> &Position<T, CM, HM, S> {
        &self.current_position
    }

    fn get_trades(&self) -> &[Trade<T, CM, S>] {
        &self.trades
    }

    fn get_last_trade(&self) -> Option<&Trade<T, CM, S>> {
        self.trades.last()
    }

    fn get_last_trade_of_type(&self, trade_type: TradeType) -> Option<&Trade<T, CM, S>> {
        match trade_type {
            TradeType::Buy => self.buy_trades.last(),
            TradeType::Sell => self.sell_trades.last(),
        }
    }

    fn get_last_entry(&self) -> Option<&Trade<T, CM, S>> {
        self.entry_trades.last()
    }

    fn get_last_exit(&self) -> Option<&Trade<T, CM, S>> {
        self.exit_trades.last()
    }

    fn get_start_index(&self) -> Option<usize> {
        self.start_index
    }

    fn get_end_index(&self) -> Option<usize> {
        self.end_index
    }
}

impl<T, CM, HM, S> fmt::Display for BaseTradingRecord<T, CM, HM, S>
where
    T: TrNum + 'static,
    CM: CostModel<T> + Clone,
    HM: CostModel<T> + Clone,
    S: BarSeries<T> + 'static,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "BaseTradingRecord: {}",
            self.name.as_deref().unwrap_or("")
        )?;
        for trade in &self.trades {
            writeln!(f, "{:?}", trade)?;
        }
        Ok(())
    }
}
