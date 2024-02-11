use crate::*;

pub trait DefaultBuilder<DI: DomainItem, D: Domain<DI, RI>, RI: RangeItem> {
    fn default_rules() -> Vec<Box<dyn Fn(&DI) -> Option<RI>>>;
    fn default() -> FizzBuzzBuilder<DI, D, RI, false>;
}