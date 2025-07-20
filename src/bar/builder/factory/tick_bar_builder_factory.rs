// use crate::bar::types::BarBuilderFactory;
// use crate::num::TrNum;
//
// /// TickBarBuilderFactory - 创建 TickBarBuilder 的工厂
// #[derive(Debug, Clone)]
// pub struct TickBarBuilderFactory {
//     tick_count: u64,
// }
//
// impl TickBarBuilderFactory {
//     pub fn new(tick_count: u64) -> Self {
//         Self { tick_count }
//     }
// }
//
// impl<T: TrNum> BarBuilderFactory<T> for TickBarBuilderFactory
// where
//     T::Factory: Default + Clone,
// {
//     type Builder = TickBarBuilder<T, ()>; // 未绑定状态
//
//     fn create_bar_builder(&self) -> Self::Builder {
//         TickBarBuilder::new(self.tick_count)
//     }
// }
