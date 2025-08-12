use crate::bar::types::BarSeries;
use crate::num::{NumFactory, TrNum};
use std::fmt;
use crate::analysis::cost::CostContext;
use crate::analysis::CostModel;
use crate::num::types::NumError;
use crate::trade::{Trade, TradeType};

#[derive(Clone)]
pub struct Position<'a, T, CM, HM, S>
where
    T: TrNum + 'static,
    CM: CostModel<T> + Clone,
    HM: CostModel<T> + Clone,
    S: BarSeries<'a, T>,
{
    entry: Option<Trade<'a, T, CM, S>>,
    exit: Option<Trade<'a, T, CM, S>>,
    starting_type: TradeType,
    transaction_cost_model: CM,
    holding_cost_model: HM,
    _marker: std::marker::PhantomData<&'a S>,
}

impl<'a, T, CM, HM, S> Position<'a, T, CM, HM, S>
where
    T: TrNum + 'static,
    CM: CostModel<T> + Clone,
    HM: CostModel<T> + Clone,
    S: BarSeries<'a, T>,
{
    /// 构造通用 CostContext（不带 final_index）
    pub fn build_cost_context(&self) -> CostContext<T> {
        CostContext {
            entry_price: self.price_per_asset.clone(),
            amount: self.amount.clone(),
            entry_index: Some(self.index),
            final_index: None,
            is_closed: false,
            // 未来额外字段填充
        }
    }

    /// 构造带 final_index 的 CostContext
    pub fn build_cost_context_with_final_index(&self, final_index: usize) -> CostContext<T> {
        CostContext {
            entry_price: self.price_per_asset.clone(),
            amount: self.amount.clone(),
            entry_index: Some(self.index),
            final_index: Some(final_index),
            is_closed: true,
        }
    }

    /// Creates a new Position with given starting type
    pub fn new(
        starting_type: TradeType,
        transaction_cost_model: CM,
        holding_cost_model: HM,
    ) -> Self {
        Self {
            entry: None,
            exit: None,
            starting_type,
            transaction_cost_model,
            holding_cost_model,
            _marker: std::marker::PhantomData,
        }
    }

    /// Creates a new Position with starting type = BUY and zero cost models
    pub fn new_buy(transaction_cost_model: CM, holding_cost_model: HM) -> Self {
        Self::new(TradeType::Buy, transaction_cost_model, holding_cost_model)
    }

    /// Creates a closed Position from entry and exit trades
    pub fn from_trades(
        entry: Trade<'a, T, CM, S>,
        exit: Trade<'a, T, CM, S>,
        transaction_cost_model: CM,
        holding_cost_model: HM,
    ) -> Self {
        assert_ne!(
            entry.get_type(),
            exit.get_type(),
            "Both trades must have different types"
        );
        Self {
            starting_type: entry.get_type(),
            entry: Some(entry),
            exit: Some(exit),
            transaction_cost_model,
            holding_cost_model,
            _marker: std::marker::PhantomData,
        }
    }

    pub fn entry(&self) -> Option<&Trade<'a, T, CM, S>> {
        self.entry.as_ref()
    }

    pub fn exit(&self) -> Option<&Trade<'a, T, CM, S>> {
        self.exit.as_ref()
    }

    pub fn starting_type(&self) -> TradeType {
        self.starting_type
    }

    pub fn is_new(&self) -> bool {
        self.entry.is_none() && self.exit.is_none()
    }

    pub fn is_opened(&self) -> bool {
        self.entry.is_some() && self.exit.is_none()
    }

    pub fn is_closed(&self) -> bool {
        self.entry.is_some() && self.exit.is_some()
    }

    /// Operate to open or close position, returns the created Trade if any
    pub fn operate(&mut self, index: usize, price: T, amount: T) -> Option<Trade<'a, T, CM, S>> {
        if self.is_new() {
            let trade = Trade::new(
                index,
                self.starting_type,
                price,
                amount,
                self.transaction_cost_model.clone(),
            );
            self.entry = Some(trade.clone());
            Some(trade)
        } else if self.is_opened() {
            let entry_index = self.entry.as_ref().unwrap().get_index();
            if index < entry_index {
                panic!("The index is less than the entry trade index");
            }
            let trade = Trade::new(
                index,
                self.starting_type.complement_type(),
                price,
                amount,
                self.transaction_cost_model.clone(),
            );
            self.exit = Some(trade.clone());
            Some(trade)
        } else {
            None
        }
    }

    pub fn has_profit(&self) -> bool {
        self.is_closed() && self.get_profit().is_positive()
    }

    pub fn has_loss(&self) -> bool {
        self.is_closed() && self.get_profit().is_negative()
    }

    pub fn get_profit(&self) -> T {
        if self.is_opened() {
            self.zero()
        } else {
            let exit_price = self.exit.as_ref().unwrap().get_price_per_asset();
            self.get_gross_profit(exit_price.clone()) - self.get_position_cost()
        }
    }

    pub fn get_profit_with_final(&self, final_index: usize, final_price: T) -> T {
        let gross_profit = self.get_gross_profit(final_price);
        let trading_cost = self.get_position_cost_with_final(final_index);
        gross_profit - trading_cost
    }

    pub fn get_gross_profit(&self, final_price: T) -> T {
        let mut gross_profit = if self.is_opened() {
            let e = self.entry.as_ref().unwrap();
            e.get_amount().multiplied_by(&final_price) - e.get_value()
        } else {
            let e = self.entry.as_ref().unwrap();
            let x = self.exit.as_ref().unwrap();
            x.get_value() - e.get_value()
        };
        if self.entry.as_ref().unwrap().is_sell() {
            gross_profit = gross_profit.neg();
        }
        gross_profit
    }

    pub fn get_gross_return(&self, entry_price: T, exit_price: T) -> Result<T, NumError> {
        let entry_trade = self.entry.as_ref().ok_or_else(|| {
            NumError::PositionOperateError("Cannot compute gross return: no entry trade".to_string())
        })?;

        let binding = entry_price.get_factory().one();
        let one = binding.as_ref();

        let ratio = exit_price.divided_by(&entry_price)?;
        if entry_trade.is_buy() {
            Ok(ratio)
        } else {
            Ok(one.divided_by(&ratio)?) // 卖出方向取倒数
        }
    }

    pub fn get_position_cost(&self) -> T {
        // todo 构建对应的pub struct CostContext<T: TrNum + 'static>解耦传参
        self.transaction_cost_model.calculate_position(self) + self.get_holding_cost()
    }

    pub fn get_position_cost_with_index(&self, final_index: usize) -> T {
        self.transaction_cost_model
            .calculate_with_index(self, final_index)
            + self.get_holding_cost_with_final(final_index)
    }

    pub fn get_holding_cost(&self) -> T {
        self.holding_cost_model.calculate_position(self)
    }

    pub fn get_holding_cost_with_final(&self, final_index: usize) -> T {
        self.holding_cost_model
            .calculate_with_index(self, final_index)
    }

    fn zero(&self) -> T {
        self.entry
            .as_ref()
            .unwrap()
            .get_net_price()
            .num_factory()
            .zero()
    }
}

impl<'a, T, CM, HM, S> fmt::Debug for Position<'a, T, CM, HM, S>
where
    T: TrNum + 'static,
    CM: fmt::Debug + CostModel<T> + Clone,
    HM: fmt::Debug + CostModel<T> + Clone,
    S: BarSeries<'a, T>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Position")
            .field("entry", &self.entry)
            .field("exit", &self.exit)
            .field("starting_type", &self.starting_type)
            .field("transaction_cost_model", &self.transaction_cost_model)
            .field("holding_cost_model", &self.holding_cost_model)
            .finish()
    }
}
