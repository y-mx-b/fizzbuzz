use crate::builder::BuilderState;
use crate::traits::*;
use crate::FizzBuzzBuilder;

pub trait DefaultBuilder<T: DomainItem, I: Domain<T, O>, O: RangeItem<T>> {
    fn default_rules() -> Vec<Box<dyn Fn(T) -> O>>;
    fn default() -> FizzBuzzBuilder<T, I, O, BuilderState<true, true, false>>;
}
