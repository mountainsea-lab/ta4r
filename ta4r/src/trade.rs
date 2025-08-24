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
use crate::analysis::cost::zero_cost_model::ZeroCostModel;
use crate::bar::types::{Bar, BarSeries};
use crate::num::TrNum;
use crate::num::types::NumError;
use std::fmt;
use std::fmt::Debug;
use std::hash::{Hash, Hasher};

/// 交易类型：买或卖
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TradeType {
    Buy,
    Sell,
}

impl TradeType {
    /// 返回互补类型：Buy <-> Sell
    pub const fn complement_type(&self) -> TradeType {
        match self {
            TradeType::Buy => TradeType::Sell,
            TradeType::Sell => TradeType::Buy,
        }
    }
}

/// 完整版 Trade
pub struct Trade<T, CM, S>
where
    T: TrNum + 'static,
    CM: CostModel<T> + Clone,
    S: BarSeries<T>,
{
    pub(crate) trade_type: TradeType,
    pub(crate) index: usize,
    pub(crate) price_per_asset: T,
    net_price: T,
    pub(crate) amount: T,
    cost: T,
    cost_model: CM,
    // 你可以加个 PhantomData 来标记生命周期
    _marker: std::marker::PhantomData<S>,
}

impl<T, CM, S> Clone for Trade<T, CM, S>
where
    T: TrNum + Clone + 'static,
    CM: CostModel<T> + Clone,
    S: BarSeries<T>,
{
    fn clone(&self) -> Self {
        Self {
            trade_type: self.trade_type,
            index: self.index,
            price_per_asset: self.price_per_asset.clone(),
            net_price: self.net_price.clone(),
            amount: self.amount.clone(),
            cost: self.cost.clone(),
            cost_model: self.cost_model.clone(),
            _marker: std::marker::PhantomData,
        }
    }
}

impl<T, CM, S> Trade<T, CM, S>
where
    T: TrNum + 'static,
    CM: CostModel<T> + Clone,
    S: BarSeries<T>,
{
    /// 带错误返回的通用构造（用于成本模型计算可能失败的情况）
    pub fn try_new(
        index: usize,
        trade_type: TradeType,
        price_per_asset: T,
        amount: T,
        cost_model: CM,
    ) -> Result<Self, NumError> {
        let mut trade = Trade {
            trade_type,
            index,
            price_per_asset: price_per_asset.clone(),
            net_price: price_per_asset.clone(),
            amount: amount.clone(),
            cost: T::zero(),
            cost_model: cost_model.clone(),
            _marker: std::marker::PhantomData,
        };
        // 使用复用的设置方法
        trade.set_prices_and_cost(price_per_asset, amount, cost_model)?;
        Ok(trade)
    }

    /// 通用构造，panic 版本，内部调用 try_new 并 expect
    pub fn new(
        index: usize,
        trade_type: TradeType,
        price_per_asset: T,
        amount: T,
        cost_model: CM,
    ) -> Self {
        Self::try_new(index, trade_type, price_per_asset, amount, cost_model)
            .expect("Failed to create Trade in new()")
    }

    /// 带错误返回的通过 BarSeries 创建指定数量和成本模型买卖单
    pub fn try_new_from_series_with_amount_and_cost_model(
        index: usize,
        series: &S,
        trade_type: TradeType,
        amount: T,
        cost_model: CM,
    ) -> Result<Self, String> {
        let bar = series
            .get_bar(index)
            .ok_or_else(|| format!("Bar at index {} not found", index))?;

        let price = bar
            .get_close_price()
            .clone()
            .ok_or_else(|| format!("Close price at index {} is None", index))?;

        Self::try_new(index, trade_type, price, amount, cost_model)
            .map_err(|e| format!("Failed to create Trade: {:?}", e))
    }

    /// 通过 BarSeries 创建指定数量和成本模型买卖单，panic 版本
    pub fn new_from_series_with_amount_and_cost_model(
        index: usize,
        series: &S,
        trade_type: TradeType,
        amount: T,
        cost_model: CM,
    ) -> Self {
        Self::try_new_from_series_with_amount_and_cost_model(
            index, series, trade_type, amount, cost_model,
        )
        .expect("Failed to create Trade in new_from_series_with_amount_and_cost_model()")
    }

    /// 通用设置价格和成本，带错误返回
    fn set_prices_and_cost(
        &mut self,
        price_per_asset: T,
        amount: T,
        cost_model: CM,
    ) -> Result<(), NumError> {
        self.cost_model = cost_model;
        self.price_per_asset = price_per_asset;
        self.cost = self
            .cost_model
            .calculate_trade(&self.price_per_asset, &amount);
        let cost_per_asset = self.cost.divided_by(&amount)?;
        self.net_price = match self.trade_type {
            TradeType::Buy => self.price_per_asset.plus(&cost_per_asset),
            TradeType::Sell => self.price_per_asset.minus(&cost_per_asset),
        };
        Ok(())
    }

    /// 访问器等通用方法，保持不变
    pub fn get_type(&self) -> TradeType {
        self.trade_type
    }

    pub fn get_index(&self) -> usize {
        self.index
    }

    pub fn get_price_per_asset(&self) -> &T {
        &self.price_per_asset
    }

    pub fn get_price_per_asset_with_series(&self, series: &S) -> Result<T, String> {
        if !self.price_per_asset.is_nan() {
            return Ok(self.price_per_asset.clone());
        }
        let bar = series
            .get_bar(self.index)
            .ok_or_else(|| format!("Bar at index {} not found", self.index))?;
        bar.get_close_price()
            .clone()
            .ok_or_else(|| format!("Close price at index {} is None", self.index))
    }

    /// 获取交易类型
    #[inline]
    pub fn trade_type(&self) -> TradeType {
        self.trade_type
    }

    /// 获取索引
    #[inline]
    pub fn index(&self) -> usize {
        self.index
    }

    /// 原始价格
    #[inline]
    pub fn price_per_asset(&self) -> &T {
        &self.price_per_asset
    }

    /// 净成交价格
    #[inline]
    pub fn net_price(&self) -> &T {
        &self.net_price
    }

    #[inline]
    pub fn net_price_cloned(&self) -> T {
        self.net_price.clone()
    }

    /// 成交量
    #[inline]
    pub fn amount(&self) -> &T {
        &self.amount
    }

    /// 交易成本
    #[inline]
    pub fn cost(&self) -> &T {
        &self.cost
    }

    /// 成本模型
    #[inline]
    pub fn cost_model(&self) -> &CM {
        &self.cost_model
    }

    /// 是否买单
    #[inline]
    pub fn is_buy(&self) -> bool {
        self.trade_type == TradeType::Buy
    }

    /// 是否卖单
    #[inline]
    pub fn is_sell(&self) -> bool {
        self.trade_type == TradeType::Sell
    }

    /// 总成交价值 = price_per_asset * amount
    #[inline]
    pub fn value(&self) -> T {
        self.price_per_asset.multiplied_by(&self.amount)
    }

    /// 净成交价值 = net_price * amount
    #[inline]
    pub fn net_value(&self) -> T {
        self.net_price.multiplied_by(&self.amount)
    }

    /// 静态工厂方法（买/卖）
    pub fn buy(index: usize, price: T, amount: T, cost_model: CM) -> Self {
        Self::new(index, TradeType::Buy, price, amount, cost_model)
    }

    pub fn sell(index: usize, price: T, amount: T, cost_model: CM) -> Self {
        Self::new(index, TradeType::Sell, price, amount, cost_model)
    }

    /// 静态工厂方法，调用 panic 版本（保持兼容）
    pub fn buy_at_with_amount_and_cost_model(
        index: usize,
        series: &S,
        amount: T,
        cost_model: CM,
    ) -> Self {
        Self::new_from_series_with_amount_and_cost_model(
            index,
            series,
            TradeType::Buy,
            amount,
            cost_model,
        )
    }

    pub fn sell_at_with_amount_and_cost_model(
        index: usize,
        series: &S,
        amount: T,
        cost_model: CM,
    ) -> Self {
        Self::new_from_series_with_amount_and_cost_model(
            index,
            series,
            TradeType::Sell,
            amount,
            cost_model,
        )
    }
}

impl<T, S> Trade<T, ZeroCostModel<T>, S>
where
    T: TrNum + 'static,
    S: BarSeries<T>,
{
    /// 带错误返回的 ZeroCostModel 构造
    pub fn try_new_zero_cost(
        index: usize,
        trade_type: TradeType,
        price_per_asset: T,
        amount: T,
    ) -> Result<Self, NumError> {
        Self::try_new(
            index,
            trade_type,
            price_per_asset,
            amount,
            ZeroCostModel::new(),
        )
    }

    /// 通过 BarSeries 创建默认数量和零成本的买卖单，带错误返回
    pub fn try_new_from_series(
        index: usize,
        series: &S,
        trade_type: TradeType,
    ) -> Result<Self, String> {
        let amount = T::one();
        Self::try_new_from_series_with_amount(index, series, trade_type, amount)
    }

    /// 通过 BarSeries 创建默认数量和零成本的买卖单，panic 版本
    pub fn new_from_series(index: usize, series: &S, trade_type: TradeType) -> Self {
        Self::try_new_from_series(index, series, trade_type)
            .expect("Failed to create Trade with zero cost model from series")
    }

    /// 通过 BarSeries 创建指定数量和零成本买卖单，带错误返回
    pub fn try_new_from_series_with_amount(
        index: usize,
        series: &S,
        trade_type: TradeType,
        amount: T,
    ) -> Result<Self, String> {
        let bar = series
            .get_bar(index)
            .ok_or_else(|| format!("Bar at index {} not found", index))?;

        let price = bar
            .get_close_price()
            .clone()
            .ok_or_else(|| format!("Close price at index {} is None", index))?;

        Self::try_new_zero_cost(index, trade_type, price, amount)
            .map_err(|e| format!("Failed to create Trade with zero cost: {:?}", e))
    }

    /// 直接用参数构造（ZeroCostModel版本），带错误返回
    pub fn try_new_zero_cost_with_params(
        index: usize,
        trade_type: TradeType,
        price_per_asset: T,
        amount: T,
    ) -> Result<Self, NumError> {
        Self::try_new_zero_cost(index, trade_type, price_per_asset, amount)
    }

    /// 直接用参数构造（ZeroCostModel版本），panic 版本
    pub fn new_zero_cost(
        index: usize,
        trade_type: TradeType,
        price_per_asset: T,
        amount: T,
    ) -> Self {
        Self::try_new_zero_cost(index, trade_type, price_per_asset, amount)
            .expect("Trade initialization failed due to invalid numeric operation")
    }

    /// 静态工厂方法，调用默认零成本模型的构造函数，panic 版本
    pub fn buy_at(index: usize, series: &S) -> Self {
        Self::new_from_series(index, series, TradeType::Buy)
    }

    pub fn buy_at_with_amount(index: usize, series: &S, amount: T) -> Self {
        Self::try_new_from_series_with_amount(index, series, TradeType::Buy, amount)
            .expect("Failed to create Buy Trade with zero cost and amount")
    }

    pub fn buy_at_price(index: usize, price: T, amount: T) -> Self {
        Self::new_zero_cost(index, TradeType::Buy, price, amount)
    }

    pub fn sell_at(index: usize, series: &S) -> Self {
        Self::new_from_series(index, series, TradeType::Sell)
    }

    pub fn sell_at_with_amount(index: usize, series: &S, amount: T) -> Self {
        Self::try_new_from_series_with_amount(index, series, TradeType::Sell, amount)
            .expect("Failed to create Sell Trade with zero cost and amount")
    }

    pub fn sell_at_price(index: usize, price: T, amount: T) -> Self {
        Self::new_zero_cost(index, TradeType::Sell, price, amount)
    }
}

// 实现 Display，方便打印
impl<T, CM, S> fmt::Display for Trade<T, CM, S>
where
    T: TrNum + fmt::Display + 'static,
    CM: CostModel<T> + Clone,
    S: BarSeries<T>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} Trade at index {}: price_per_asset = {}, net_price = {}, amount = {}, cost = {}, net_value = {}",
            match self.trade_type {
                TradeType::Buy => "Buy",
                TradeType::Sell => "Sell",
            },
            self.index,
            self.price_per_asset,
            self.net_price,
            self.amount,
            self.cost,
            self.net_value()
        )
    }
}

impl<T, CM, S> Debug for Trade<T, CM, S>
where
    T: TrNum + 'static + Debug,
    CM: CostModel<T> + Clone,
    S: BarSeries<T>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Trade")
            .field("trade_type", &self.trade_type)
            .field("index", &self.index)
            .field("price_per_asset", &self.price_per_asset)
            .field("net_price", &self.net_price)
            .field("amount", &self.amount)
            .field("cost", &self.cost)
            // _marker 跳过
            .finish()
    }
}

// 实现 PartialEq 和 Hash
impl<T, CM, S> PartialEq for Trade<T, CM, S>
where
    T: TrNum + PartialEq + 'static,
    CM: CostModel<T> + Clone,
    S: BarSeries<T>,
{
    fn eq(&self, other: &Self) -> bool {
        self.trade_type == other.trade_type
            && self.index == other.index
            && self.price_per_asset == other.price_per_asset
            && self.amount == other.amount
    }
}

impl<T, CM, S> Eq for Trade<T, CM, S>
where
    T: TrNum + PartialEq + Eq + 'static,
    CM: CostModel<T> + Clone,
    S: BarSeries<T>,
{
}

impl<T, CM, S> Hash for Trade<T, CM, S>
where
    T: TrNum + Hash + 'static,
    CM: CostModel<T> + Clone,
    S: BarSeries<T>,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.trade_type.hash(state);
        self.index.hash(state);
        self.price_per_asset.hash(state);
        self.amount.hash(state);
    }
}
