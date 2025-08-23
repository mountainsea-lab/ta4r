use crate::TradingRecord;
use crate::analysis::CostModel;
use crate::bar::types::BarSeries;
use crate::indicators::Indicator;
use crate::indicators::helpers::date_time_indicator::DateTimeIndicator;
use crate::num::TrNum;
use crate::rule::Rule;
use crate::rule::base_rule::BaseRule;
use std::marker::PhantomData;
use std::sync::Arc;
use time::{OffsetDateTime, Time};

#[derive(Clone)]
pub struct TimeRange {
    pub from: Time,
    pub to: Time,
}

pub struct TimeRangeRule<T, CM, HM, S, R, F>
where
    T: TrNum + Clone + 'static,
    CM: CostModel<T> + Clone,
    HM: CostModel<T> + Clone,
    S: BarSeries<T> + 'static,
    R: TradingRecord<T, CM, HM, S>,
    F: Fn(&S::Bar) -> OffsetDateTime + Copy,
{
    time_ranges: Vec<TimeRange>,
    time_indicator: Arc<DateTimeIndicator<T, S, F>>,
    base_rule: BaseRule<Self>,
    _phantom: PhantomData<(T, CM, HM, S, R)>,
}

impl<T, CM, HM, S, R, F> TimeRangeRule<T, CM, HM, S, R, F>
where
    T: TrNum + Clone + 'static,
    CM: CostModel<T> + Clone,
    HM: CostModel<T> + Clone,
    S: BarSeries<T> + 'static,
    R: TradingRecord<T, CM, HM, S>,
    F: Fn(&S::Bar) -> OffsetDateTime + Copy,
{
    pub fn new(
        time_ranges: Vec<TimeRange>,
        time_indicator: Arc<DateTimeIndicator<T, S, F>>,
    ) -> Self {
        Self {
            time_ranges,
            time_indicator,
            base_rule: BaseRule::new("TimeRangeRule"),
            _phantom: PhantomData,
        }
    }
}

impl<T, CM, HM, S, R, F> Rule for TimeRangeRule<T, CM, HM, S, R, F>
where
    T: TrNum + Clone + 'static,
    CM: CostModel<T> + Clone,
    HM: CostModel<T> + Clone,
    S: BarSeries<T> + 'static,
    R: TradingRecord<T, CM, HM, S>,
    F: Fn(&S::Bar) -> OffsetDateTime + Copy,
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
        let satisfied = match self.time_indicator.get_value(index) {
            Ok(odt) => {
                let local_time: Time = odt.time(); // 获取一天中的时间
                self.time_ranges
                    .iter()
                    .any(|range| local_time >= range.from && local_time <= range.to)
            }
            Err(_) => false,
        };

        self.base_rule.trace_is_satisfied(index, satisfied);
        satisfied
    }
}
