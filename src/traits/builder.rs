use crate::traits::*;
use crate::builder::BuilderState;
use crate::FizzBuzzBuilder;
use std::collections::BTreeMap;

pub trait DefaultBuilder<T: DomainItem, I: Domain<T, O>, O: RangeItem<T>> {
    fn default_map() -> BTreeMap<T, O>;
    fn default_rule() -> Box<dyn Fn(T, T) -> bool>;
    fn default() -> FizzBuzzBuilder<T, I, O, BuilderState<true, true, false>>;
}
