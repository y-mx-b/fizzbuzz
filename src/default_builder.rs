use crate::*;

pub trait DefaultBuilder<DI: DomainItem, D: Domain<DI>, RI: RangeItem> {
    fn default_rules() -> Vec<Rule<DI, RI>>;
    fn default() -> FizzBuzzBuilder<DI, D, RI, false>;
}
