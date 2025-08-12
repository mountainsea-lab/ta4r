/*!
 * MIT License
 *
 * Copyright (c) 2025 Mountainsea
 * Based on ta4j (c) 2017–2025 Ta4j Organization & respective authors (see AUTHORS)
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

use crate::analysis::CostModel;
use crate::analysis::cost::CostContext;
use crate::bar::types::BarSeries;
use crate::num::types::NumError;
use crate::num::{NumFactory, TrNum};
use crate::trade::{Trade, TradeType};
use std::fmt;

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
    <T as TrNum>::Factory: NumFactory<T>,
    CM: CostModel<T> + Clone,
    HM: CostModel<T> + Clone,
    S: BarSeries<'a, T>,
{
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
        let trading_cost = self.get_position_cost_with_index(final_index);
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
            NumError::PositionOperateError(
                "Cannot compute gross return: no entry trade".to_string(),
            )
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
        // 未闭合
        let ctx: CostContext<T> = self.into();
        self.transaction_cost_model.calculate_position(&ctx) + self.get_holding_cost()
    }

    pub fn get_position_cost_with_index(&self, final_index: usize) -> T {
        // 闭合
        let closed_ctx = self.to_closed_cost_context(final_index);
        self.transaction_cost_model
            .calculate_with_index(&closed_ctx)
            + self.get_holding_cost_with_final(final_index)
    }

    pub fn get_holding_cost(&self) -> T {
        let ctx: CostContext<T> = self.into();
        self.holding_cost_model.calculate_position(&ctx)
    }

    pub fn get_holding_cost_with_final(&self, final_index: usize) -> T {
        // 闭合
        let closed_ctx = self.to_closed_cost_context(final_index);
        self.holding_cost_model.calculate_with_index(&closed_ctx)
    }

    fn zero(&self) -> T {
        let trade = self.entry.as_ref().unwrap();
        let net_price = trade.get_net_price();
        let factory = net_price.get_factory();

        let zero_wrapped = factory.zero();
        zero_wrapped.as_ref().clone()
    }
}

impl<'a, T, CM, HM, S> fmt::Debug for Position<'a, T, CM, HM, S>
where
    T: TrNum + 'static + fmt::Debug,
    CM: CostModel<T> + Clone + fmt::Debug,
    HM: CostModel<T> + Clone + fmt::Debug,
    S: BarSeries<'a, T>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Position")
            .field("entry", &self.entry)
            .field("exit", &self.exit)
            .field("starting_type", &self.starting_type)
            .field("transaction_cost_model", &self.transaction_cost_model)
            .field("holding_cost_model", &self.holding_cost_model)
            // PhantomData 跳过打印
            .finish()
    }
}

impl<'a, T, CM, HM, S> From<&Position<'a, T, CM, HM, S>> for CostContext<T>
where
    T: TrNum + Clone,
    CM: CostModel<T> + Clone,
    HM: CostModel<T> + Clone,
    S: BarSeries<'a, T>,
{
    fn from(pos: &Position<'a, T, CM, HM, S>) -> Self {
        let trade = pos.entry.as_ref().expect("Position has no entry trade");

        CostContext {
            entry_price: trade.price_per_asset.clone(),
            amount: trade.amount.clone(),
            entry_index: Some(trade.index),
            final_index: None,
            is_closed: false,
        }
    }
}

impl<'a, T, CM, HM, S> Position<'a, T, CM, HM, S>
where
    T: TrNum + Clone,
    CM: CostModel<T> + Clone,
    HM: CostModel<T> + Clone,
    S: BarSeries<'a, T>,
{
    pub fn to_closed_cost_context(&self, final_index: usize) -> CostContext<T> {
        let trade = self.entry.as_ref().expect("Position has no entry trade");

        CostContext {
            entry_price: trade.price_per_asset.clone(),
            amount: trade.amount.clone(),
            entry_index: Some(trade.index),
            final_index: Some(final_index),
            is_closed: true,
        }
    }
}
