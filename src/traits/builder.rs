use crate::traits::*;
use crate::FizzBuzzBuilder;

pub trait DefaultBuilder<DI: DomainItem, D: Domain<DI, RI>, RI: RangeItem<DI>> {
    fn default_rules() -> Vec<Box<dyn Fn(DI) -> RI>>;
    fn default() -> FizzBuzzBuilder<DI, D, RI, false>;
}
