use crate::indicators::Indicator;

pub struct ConstantIndicator<Num, Series> {
    value: Num,
    series: Series,
}

impl<Num, Series> ConstantIndicator<Num, Series> {
    pub fn new(series: Series, value: Num) -> Self {
        Self { value, series }
    }
}

impl<Num: Copy, Series> Indicator for ConstantIndicator<Num, Series> {
    type Num = Num;
    type Series<'a> = Series where Series: 'a;

    fn get_value(&self, _index: usize) -> Self::Num {
        self.value
    }

    fn get_bar_series(&self) -> &Self::Series<'_> {
        &self.series
    }

    fn get_count_of_unstable_bars(&self) -> usize {
        0
    }
}
