use crate::bar::types::BarSeries;
use crate::num::TrNum;

/// todo 暂时设计后续是分析模块中
pub trait CostModel<T: TrNum> {
    fn calculate(&self, price: T, amount: T) -> T;
}

// 零成本模型示例
#[derive(Debug, Clone, Copy)]
pub struct ZeroCostModel;

impl<T: TrNum> CostModel<T> for ZeroCostModel {
    fn calculate(&self, _price: T, _amount: T) -> T {
        T::zero()
    }
}

/// 交易类型：买或卖
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TradeType {
    Buy,
    Sell,
}

impl TradeType {
    /// 返回互补类型：Buy <-> Sell
    pub fn complement_type(&self) -> TradeType {
        match self {
            TradeType::Buy => TradeType::Sell,
            TradeType::Sell => TradeType::Buy,
        }
    }
}


/// Trade 结构体
#[derive(Debug, Clone)]
pub struct Trade<T: TrNum, C: CostModel<T>> {
    trade_type: TradeType,
    index: usize,
    price_per_asset: T,
    net_price: T,
    amount: T,
    cost: T,
    cost_model: C,
}

impl<T: TrNum, C: CostModel<T> + Copy> Trade<T, C> {
    // 私有构造函数
    fn new_with_cost_model(
        index: usize,
        trade_type: TradeType,
        price_per_asset: T,
        amount: T,
        cost_model: C,
    ) -> Self {
        let cost = cost_model.calculate(price_per_asset, amount);
        let cost_per_asset = cost / amount;

        let net_price = match trade_type {
            TradeType::Buy => price_per_asset + cost_per_asset,
            TradeType::Sell => price_per_asset - cost_per_asset,
        };

        Self {
            trade_type,
            index,
            price_per_asset,
            net_price,
            amount,
            cost,
            cost_model,
        }
    }

    // 构造函数，基于 BarSeries
    pub fn new(index: usize, series: &impl BarSeries<T>, trade_type: TradeType) -> Self {
        Self::new_with_cost_model(
            index,
            trade_type,
            series.get_bar(index).close_price(),
            T::one(),
            ZeroCostModel,
        )
    }

    pub fn new_with_amount(
        index: usize,
        series: &impl BarSeries<T>,
        trade_type: TradeType,
        amount: T,
    ) -> Self {
        Self::new_with_cost_model(
            index,
            trade_type,
            series.get_bar(index).close_price(),
            amount,
            ZeroCostModel,
        )
    }

    pub fn new_with_cost_model_and_amount(
        index: usize,
        series: &impl BarSeries<T>,
        trade_type: TradeType,
        amount: T,
        cost_model: C,
    ) -> Self {
        Self::new_with_cost_model(
            index,
            trade_type,
            series.get_bar(index).close_price(),
            amount,
            cost_model,
        )
    }

    // 静态工厂方法对应 buyAt 和 sellAt

    pub fn buy_at(index: usize, series: &impl BarSeries<T>) -> Self {
        Self::new(index, series, TradeType::Buy)
    }

    pub fn buy_at_with_amount(index: usize, series: &impl BarSeries<T>, amount: T) -> Self {
        Self::new_with_amount(index, series, TradeType::Buy, amount)
    }

    pub fn buy_at_with_amount_and_cost_model(
        index: usize,
        series: &impl BarSeries<T>,
        amount: T,
        cost_model: C,
    ) -> Self {
        Self::new_with_cost_model_and_amount(index, series, TradeType::Buy, amount, cost_model)
    }

    pub fn sell_at(index: usize, series: &impl BarSeries<T>) -> Self {
        Self::new(index, series, TradeType::Sell)
    }

    pub fn sell_at_with_amount(index: usize, series: &impl BarSeries<T>, amount: T) -> Self {
        Self::new_with_amount(index, series, TradeType::Sell, amount)
    }

    pub fn sell_at_with_amount_and_cost_model(
        index: usize,
        series: &impl BarSeries<T>,
        amount: T,
        cost_model: C,
    ) -> Self {
        Self::new_with_cost_model_and_amount(index, series, TradeType::Sell, amount, cost_model)
    }

    // 其他 getter 方法

    pub fn is_buy(&self) -> bool {
        self.trade_type == TradeType::Buy
    }

    pub fn is_sell(&self) -> bool {
        self.trade_type == TradeType::Sell
    }

    pub fn get_value(&self) -> T {
        self.price_per_asset * self.amount
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Bar 和 BarSeries
    #[derive(Debug, Clone)]
    pub struct Bar<T: TrNum> {
        close_price: T,
    }

    impl<T: TrNum> Bar<T> {
        pub fn close_price(&self) -> T {
            self.close_price
        }
    }

    // 简单实现 BarSeries，闭包式
    struct SimpleBarSeries<T: TrNum> {
        bars: Vec<Bar<T>>,
    }

    impl<T: TrNum> SimpleBarSeries<T> {
        fn new(bars: Vec<Bar<T>>) -> Self {
            Self { bars }
        }
    }

    impl<T: TrNum> BarSeries<T> for SimpleBarSeries<T> {
        fn get_bar(&self, index: usize) -> &Bar<T> {
            &self.bars[index]
        }
    }

    #[test]
    fn trade_basic() {
        let price = 100.0;
        let amount = 2.0;

        // 构造简单的 BarSeries
        let bars = vec![Bar { close_price: price }; 30];
        let series = SimpleBarSeries::new(bars);

        // 零成本模型默认
        let buy_trade = Trade::buy_at_with_amount(10, &series, amount);
        assert!(buy_trade.is_buy());
        // net_price = price + cost_per_asset = 100 + 0 = 100
        assert_eq!(buy_trade.net_price, price);
        assert_eq!(buy_trade.get_value(), price * amount);

        let sell_trade = Trade::sell_at_with_amount(20, &series, amount);
        assert!(sell_trade.is_sell());
        assert_eq!(sell_trade.net_price, price);
    }

    // 测试带成本模型的 Trade
    #[derive(Clone, Copy)]
    struct FlatCostModel(f64);

    impl CostModel<f64> for FlatCostModel {
        fn calculate(&self, _price: f64, amount: f64) -> f64 {
            self.0 * amount
        }
    }

    #[test]
    fn trade_with_cost_model() {
        let price = 100.0;
        let amount = 2.0;
        let cost_per_amount = 1.0;

        let bars = vec![Bar { close_price: price }; 30];
        let series = SimpleBarSeries::new(bars);

        let cost_model = FlatCostModel(cost_per_amount);

        let buy_trade =
            Trade::buy_at_with_amount_and_cost_model(5, &series, amount, cost_model);
        assert!(buy_trade.is_buy());
        // cost = cost_model.calculate = 1 * 2 = 2
        // cost_per_asset = 2 / 2 = 1
        // net_price = price + cost_per_asset = 101
        assert_eq!(buy_trade.net_price, price + cost_per_amount);

        let sell_trade =
            Trade::sell_at_with_amount_and_cost_model(15, &series, amount, cost_model);
        assert!(sell_trade.is_sell());
        // net_price = price - cost_per_asset = 99
        assert_eq!(sell_trade.net_price, price - cost_per_amount);
    }
}
