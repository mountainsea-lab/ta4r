use std::cell::RefCell;
use std::rc::Rc;

use crate::indicators::AuxIndicator;
use crate::indicators::types::IndicatorError;

/// 通用的非数值型缓存指标器
#[derive(Clone)]
pub struct CachedAuxIndicator<I>
where
    I: AuxIndicator,
{
    indicator: I,
    cache: Rc<RefCell<Vec<Option<I::Output>>>>,
}

impl<I> CachedAuxIndicator<I>
where
    I: AuxIndicator,
{
    pub fn new(indicator: I) -> Self {
        Self {
            indicator,
            cache: Rc::new(RefCell::new(Vec::new())),
        }
    }
}

impl<I> AuxIndicator for CachedAuxIndicator<I>
where
    I: AuxIndicator,
{
    type Output = I::Output;
    type Num = I::Num;
    type Series<'a>
        = I::Series<'a>
    where
        Self: 'a;

    fn get_value(&self, index: usize) -> Result<Self::Output, IndicatorError> {
        let mut cache = self.cache.borrow_mut();

        if index >= cache.len() {
            cache.resize(index + 1, None);
        }

        if let Some(value) = &cache[index] {
            Ok(value.clone())
        } else {
            let value = self.indicator.get_value(index)?;
            cache[index] = Some(value.clone());
            Ok(value)
        }
    }

    fn get_bar_series(&self) -> &Self::Series<'_> {
        self.indicator.get_bar_series()
    }

    fn get_count_of_unstable_bars(&self) -> usize {
        self.indicator.get_count_of_unstable_bars()
    }
}
