use super::{FizzBuzzable, FizzBuzzed};
use crate::builder::BuilderState;
use crate::FizzBuzzBuilder;
use std::collections::BTreeMap;

pub trait DefaultBuilder<T, I: FizzBuzzable<T, O>, O: FizzBuzzed<T, I>> {
    fn default_map() -> BTreeMap<I, O>;
    fn default_rule() -> Box<dyn Fn(I, I) -> bool>;
    fn default() -> FizzBuzzBuilder<T, I, O, BuilderState<true, true, true, true>>;
}
