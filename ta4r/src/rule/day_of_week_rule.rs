use crate::TradingRecord;
use crate::analysis::CostModel;
use crate::bar::types::BarSeries;
use crate::indicators::Indicator;
use crate::indicators::helpers::date_time_indicator::DateTimeIndicator;
use crate::num::TrNum;
use crate::rule::Rule;
use crate::rule::base_rule::BaseRule;
use std::collections::HashSet;
use time::{OffsetDateTime, Weekday};

/// DayOfWeekRule
/// 满足条件：当时间指标的 day_of_week 在指定集合中
pub struct DayOfWeekRule<T, CM, HM, S, F, R>
where
    T: TrNum + Clone + From<bool> + 'static,
    CM: CostModel<T> + Clone,
    HM: CostModel<T> + Clone,
    S: BarSeries<T> + 'static,
    F: Fn(&S::Bar) -> OffsetDateTime + Copy,
    R: TradingRecord<T, CM, HM, S>,
{
    time_indicator: DateTimeIndicator<T, S, F>,
    days_of_week: HashSet<Weekday>,
    base_rule: BaseRule,
    _phantom: std::marker::PhantomData<(CM, HM, R)>,
}

impl<T, CM, HM, S, F, R> DayOfWeekRule<T, CM, HM, S, F, R>
where
    T: TrNum + Clone + From<bool> + 'static,
    CM: CostModel<T> + Clone,
    HM: CostModel<T> + Clone,
    S: BarSeries<T> + 'static,
    F: Fn(&S::Bar) -> OffsetDateTime + Copy,
    R: TradingRecord<T, CM, HM, S>,
{
    /// 构造函数
    pub fn new(time_indicator: DateTimeIndicator<T, S, F>, days_of_week: &[Weekday]) -> Self {
        Self {
            time_indicator,
            days_of_week: days_of_week.iter().copied().collect(),
            base_rule: BaseRule::new("DayOfWeekRule"),
            _phantom: std::marker::PhantomData,
        }
    }

    /// 获取时间指标
    pub fn get_time_indicator(&self) -> &DateTimeIndicator<T, S, F> {
        &self.time_indicator
    }

    /// 获取允许的 Weekday 集合
    pub fn get_days_of_week(&self) -> &HashSet<Weekday> {
        &self.days_of_week
    }
}

impl<T, CM, HM, S, F, R> Clone for DayOfWeekRule<T, CM, HM, S, F, R>
where
    CM: Clone + CostModel<T>,
    F: Copy + Fn(&S::Bar) -> OffsetDateTime,
    HM: Clone + CostModel<T>,
    R: TradingRecord<T, CM, HM, S>,
    S: 'static + BarSeries<T>,
    T: 'static + Clone + From<bool> + TrNum,
{
    fn clone(&self) -> Self {
        Self {
            time_indicator: self.time_indicator.clone(),
            days_of_week: self.days_of_week.clone(),
            base_rule: self.base_rule.clone(),
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<T, CM, HM, S, F, R> Rule for DayOfWeekRule<T, CM, HM, S, F, R>
where
    T: TrNum + Clone + From<bool> + 'static,
    CM: CostModel<T> + Clone,
    HM: CostModel<T> + Clone,
    S: BarSeries<T> + 'static,
    F: Fn(&S::Bar) -> OffsetDateTime + Copy,
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
        let dt = match self.time_indicator.get_value(index) {
            Ok(dt) => dt,
            Err(_) => return false,
        };
        let weekday = dt.weekday();
        let satisfied = self.days_of_week.contains(&weekday);
        self.base_rule.trace_is_satisfied(index, satisfied);
        satisfied
    }
}
