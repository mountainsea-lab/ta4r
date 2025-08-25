use crate::TradingRecord;
use crate::analysis::CostModel;
use crate::bar::types::BarSeries;
use crate::indicators::Indicator;
use crate::indicators::helpers::constant_indicator::ConstantIndicator;
use crate::indicators::helpers::cross_indicator::CrossIndicator;
use crate::num::{NumFactory, TrNum};
use crate::rule::Rule;
use crate::rule::base_rule::BaseRule;
use std::marker::PhantomData;
use std::sync::Arc;

/// CrossedUpIndicatorRule
/// 满足条件：当 up crosses-up low  指标时
pub struct CrossedUpIndicatorRule<T, CM, HM, S, IU, IL, R>
where
    T: TrNum + Clone + From<bool> + 'static,
    CM: CostModel<T> + Clone,
    HM: CostModel<T> + Clone,
    S: BarSeries<T> + 'static,
    IU: Indicator<Num = T, Output = T, Series = S>,
    IL: Indicator<Num = T, Output = T, Series = S>,
    R: TradingRecord<T, CM, HM, S>,
{
    cross: CrossIndicator<T, S, IL, IU>,
    base_rule: BaseRule,
    _phantom: PhantomData<(CM, HM, R)>,
}

impl<T, CM, HM, S, IU, IL, R> CrossedUpIndicatorRule<T, CM, HM, S, IU, IL, R>
where
    T: TrNum + Clone + From<bool> + 'static,
    CM: CostModel<T> + Clone,
    HM: CostModel<T> + Clone,
    S: BarSeries<T> + 'static,
    IU: Indicator<Num = T, Output = T, Series = S>,
    IL: Indicator<Num = T, Output = T, Series = S>,
    R: TradingRecord<T, CM, HM, S>,
{
    pub fn new(up: Arc<IU>, low: Arc<IL>) -> Self {
        Self {
            cross: CrossIndicator::new(low, up),
            base_rule: BaseRule::new("CrossedUpIndicatorRule"),
            _phantom: PhantomData,
        }
    }

    /// 常量 threshold 构造
    pub fn from_threshold(
        indicator: Arc<IU>,
        threshold: T,
    ) -> CrossedUpIndicatorRule<T, CM, HM, S, IU, ConstantIndicator<T, S>, R> {
        let low = Arc::new(ConstantIndicator::new(indicator.bar_series(), threshold));
        CrossedUpIndicatorRule {
            cross: CrossIndicator::new(low, indicator),
            base_rule: BaseRule::new("CrossedUpIndicatorRule"),
            _phantom: PhantomData,
        }
    }

    /// 常量 threshold 构造函数（接收 f64）
    pub fn from_threshold_f64(
        indicator: Arc<IU>,
        threshold: f64,
    ) -> CrossedUpIndicatorRule<T, CM, HM, S, IU, ConstantIndicator<T, S>, R> {
        let num_factory = indicator
            .bar_series()
            .with_ref(|s| s.num_factory())
            .expect("num_factory get failed");
        let num = num_factory.num_of_f64(threshold);
        Self::from_threshold(indicator, num)
    }

    pub fn get_up(&self) -> Arc<IU> {
        self.cross.get_low()
    }

    pub fn get_low(&self) -> Arc<IL> {
        self.cross.get_up()
    }
}

impl<T, CM, HM, S, IU, IL, R> Clone for CrossedUpIndicatorRule<T, CM, HM, S, IU, IL, R>
where
    CM: Clone + CostModel<T>,
    HM: Clone + CostModel<T>,
    IL: Indicator<Num = T, Output = T, Series = S>,
    IU: Indicator<Num = T, Output = T, Series = S>,
    R: TradingRecord<T, CM, HM, S>,
    S: 'static + BarSeries<T>,
    T: 'static + Clone + From<bool> + TrNum,
{
    fn clone(&self) -> Self {
        Self {
            cross: self.cross.clone(),
            base_rule: self.base_rule.clone(),
            _phantom: PhantomData,
        }
    }
}

impl<T, CM, HM, S, IU, IL, R> Rule for CrossedUpIndicatorRule<T, CM, HM, S, IU, IL, R>
where
    T: TrNum + Clone + From<bool> + 'static,
    CM: CostModel<T> + Clone,
    HM: CostModel<T> + Clone,
    S: BarSeries<T> + 'static,
    IU: Indicator<Num = T, Output = T, Series = S>,
    IL: Indicator<Num = T, Output = T, Series = S>,
    R: TradingRecord<T, CM, HM, S>,
{
    type Num = T;
    type CostBuy = CM;
    type CostSell = HM;
    type Series = S;
    type TradingRec = R;

    fn is_satisfied_with_record(
        &self,
        index: usize,
        _trading_record: Option<&Self::TradingRec>,
    ) -> bool {
        let crossed = self.cross.get_value(index).map_or(false, |v| v.0);
        self.base_rule.trace_is_satisfied(index, crossed);
        crossed
    }
}
