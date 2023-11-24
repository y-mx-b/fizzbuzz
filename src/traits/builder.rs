use super::{FizzBuzzable, FizzBuzzed};
use crate::FizzBuzz;
use std::collections::BTreeMap;

pub trait DefaultBuilder<I: FizzBuzzable<O>, O: FizzBuzzed<I>> {
    fn default_map() -> BTreeMap<I, O>;
    fn default_rule() -> Box<dyn Fn(I, I) -> bool>;
    fn build(self) -> FizzBuzz<I, O>;
}
