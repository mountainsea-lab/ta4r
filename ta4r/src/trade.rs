use crate::bar::types::BarSeries;
use crate::num::TrNum;
use std::fmt;
use std::fmt::Debug;
use std::hash::{Hash, Hasher};
use crate::analysis::cost::zero_cost_model::ZeroCostModel;
use crate::analysis::CostModel;

/// 交易类型：买或卖
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TradeType {
    Buy,
    Sell,
}

impl TradeType {
    /// 返回互补类型：Buy <-> Sell
    // pub fn complement_type(&self) -> TradeType {
    //     match self {
    //         TradeType::Buy => TradeType::Sell,
    //         TradeType::Sell => TradeType::Buy,
    //     }
    // }

    pub const fn complement_type(&self) -> TradeType {
        match self {
            TradeType::Buy => TradeType::Sell,
            TradeType::Sell => TradeType::Buy,
        }
    }
}

/// 完整版 Trade
pub struct Trade<'a, T, CM, S>
where
    T: TrNum + 'static,
    CM: CostModel<T> + Clone,
    S: BarSeries<'a, T>,
{
    trade_type: TradeType,
    index: usize,
    price_per_asset: T,
    net_price: T,
    amount: T,
    cost: T,
    cost_model: CM,
    // 你可以加个 PhantomData 来标记生命周期
    _marker: std::marker::PhantomData<&'a S>,
}

impl<'a, T, CM, S> Clone for Trade<'a, T, CM, S>
where
    T: TrNum + Clone + 'static,
    CM: CostModel<T> + Clone,
    S: BarSeries<'a, T>,
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

impl<'a, T, CM, S> Trade<'a, T, CM, S>
where
    T: TrNum + 'static,
    CM: CostModel<T> + Clone,
    S: BarSeries<'a, T>,
{
    /// 通过 BarSeries 创建默认数量和零成本的买卖单
    pub fn new_from_series(index: usize, series: &'a S, trade_type: TradeType) -> Self {
        let amount = T::one();
        let cost_model =  ZeroCostModel::new();
        let price = series.get_bar(index).get_close_price().clone();
        Self::new(index, trade_type, price, amount, cost_model)
    }

    pub fn new_from_series_with_amount(
        index: usize,
        series: &'a S,
        trade_type: TradeType,
        amount: T,
    ) -> Self {
        let cost_model = ZeroCostModel::new();
        let price = series.get_bar(index).get_close_price().clone();
        Self::new(index, trade_type, price, amount, cost_model)
    }

    pub fn new_from_series_with_amount_and_cost_model(
        index: usize,
        series: &'a S,
        trade_type: TradeType,
        amount: T,
        cost_model: CM,
    ) -> Self {
        let price = series.get_bar(index).get_close_price().clone();
        Self::new(index, trade_type, price, amount, cost_model)
    }

    /// 直接用参数构造
    pub fn new(
        index: usize,
        trade_type: TradeType,
        price_per_asset: T,
        amount: T,
        cost_model: CM,
    ) -> Self {
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
        trade.set_prices_and_cost(price_per_asset, amount, cost_model);
        trade
    }

    fn set_prices_and_cost(&mut self, price_per_asset: T, amount: T, cost_model: CM) {
        self.cost_model = cost_model.clone();
        self.price_per_asset = price_per_asset.clone();
        self.cost = self.cost_model.calculate(&price_per_asset, &amount);
        let cost_per_asset = self.cost.divided_by(&amount);
        self.net_price = match self.trade_type {
            TradeType::Buy => self.price_per_asset.plus(&cost_per_asset),
            TradeType::Sell => self.price_per_asset.minus(&cost_per_asset),
        };
    }

    // 访问器示例
    pub fn get_type(&self) -> TradeType {
        self.trade_type
    }

    pub fn get_index(&self) -> usize {
        self.index
    }

    pub fn get_price_per_asset(&self) -> &T {
        &self.price_per_asset
    }

    pub fn get_price_per_asset_with_series(&self, series: &'a S) -> T {
        if self.price_per_asset.is_nan() {
            series.get_bar(self.index).get_close_price().clone()
        } else {
            self.price_per_asset.clone()
        }
    }

    pub fn get_net_price(&self) -> &T {
        &self.net_price
    }

    pub fn get_amount(&self) -> &T {
        &self.amount
    }

    pub fn get_cost(&self) -> &T {
        &self.cost
    }

    pub fn get_cost_model(&self) -> &CM {
        &self.cost_model
    }

    pub fn is_buy(&self) -> bool {
        self.trade_type == TradeType::Buy
    }

    pub fn is_sell(&self) -> bool {
        self.trade_type == TradeType::Sell
    }

    pub fn get_value(&self) -> T {
        self.price_per_asset.multiplied_by(&self.amount)
    }

    // 静态工厂方法，全部带生命周期和泛型
    pub fn buy_at(index: usize, series: &'a S) -> Self {
        Self::new_from_series(index, series, TradeType::Buy)
    }

    pub fn buy_at_with_amount(index: usize, series: &'a S, amount: T) -> Self {
        Self::new_from_series_with_amount(index, series, TradeType::Buy, amount)
    }

    pub fn buy_at_with_amount_and_cost_model(
        index: usize,
        series: &'a S,
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

    pub fn buy_at_price(index: usize, price: T, amount: T) -> Self {
        let cost_model = ZeroCostModel::new();
        Self::new(index, TradeType::Buy, price, amount, cost_model)
    }

    pub fn buy_at_price_with_cost_model(index: usize, price: T, amount: T, cost_model: CM) -> Self {
        Self::new(index, TradeType::Buy, price, amount, cost_model)
    }

    pub fn sell_at(index: usize, series: &'a S) -> Self {
        Self::new_from_series(index, series, TradeType::Sell)
    }

    pub fn sell_at_with_amount(index: usize, series: &'a S, amount: T) -> Self {
        Self::new_from_series_with_amount(index, series, TradeType::Sell, amount)
    }

    pub fn sell_at_with_amount_and_cost_model(
        index: usize,
        series: &'a S,
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

    pub fn sell_at_price(index: usize, price: T, amount: T) -> Self {
        let cost_model = ZeroCostModel::new();
        Self::new(index, TradeType::Sell, price, amount, cost_model)
    }

    pub fn sell_at_price_with_cost_model(
        index: usize,
        price: T,
        amount: T,
        cost_model: CM,
    ) -> Self {
        Self::new(index, TradeType::Sell, price, amount, cost_model)
    }
}

// 实现 Display，方便打印
impl<'a, T, CM, S> fmt::Display for Trade<'a, T, CM, S>
where
    T: TrNum + Debug + 'static,
    CM: CostModel<T> + Clone + Debug,
    S: BarSeries<'a, T>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Trade {{ type: {:?}, index: {}, price: {:?}, amount: {:?} }}",
            self.trade_type, self.index, self.price_per_asset, self.amount
        )
    }
}

// 实现 PartialEq 和 Hash
impl<'a, T, CM, S> PartialEq for Trade<'a, T, CM, S>
where
    T: TrNum + PartialEq + 'static,
    CM: CostModel<T> + Clone,
    S: BarSeries<'a, T>,
{
    fn eq(&self, other: &Self) -> bool {
        self.trade_type == other.trade_type
            && self.index == other.index
            && self.price_per_asset == other.price_per_asset
            && self.amount == other.amount
    }
}

impl<'a, T, CM, S> Eq for Trade<'a, T, CM, S>
where
    T: TrNum + PartialEq + Eq + 'static,
    CM: CostModel<T> + Clone,
    S: BarSeries<'a, T>,
{
}

impl<'a, T, CM, S> Hash for Trade<'a, T, CM, S>
where
    T: TrNum + Hash + 'static,
    CM: CostModel<T> + Clone,
    S: BarSeries<'a, T>,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.trade_type.hash(state);
        self.index.hash(state);
        self.price_per_asset.hash(state);
        self.amount.hash(state);
    }
}
