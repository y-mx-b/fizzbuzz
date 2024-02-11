use crate::traits::*;
use crate::FizzBuzz;

/// A builder for the [FizzBuzz] struct.
///
/// This builder uses the
/// [Typestate](https://willcrichton.net/rust-api-type-patterns/typestate.html)
/// pattern to guarantee a build will not fail. Thus, you can only call the
/// [build](crate::FizzBuzzBuilder::build) method once the builder is properly
pub struct FizzBuzzBuilder<DI: DomainItem, D: Domain<DI, RI>, RI: RangeItem<DI>, const DOMAIN: bool>
{
    pub(crate) domain: Option<D>,
    pub(crate) rules: Vec<Box<dyn Fn(DI) -> RI>>,
}

impl<DI: DomainItem, D: Domain<DI, RI>, RI: RangeItem<DI>> FizzBuzzBuilder<DI, D, RI, false> {
    pub fn new() -> Self {
        Self {
            domain: None,
            rules: Vec::new(),
        }
    }
}

impl<DI: DomainItem, D: Domain<DI, RI>, RI: RangeItem<DI>, const DOMAIN: bool>
    FizzBuzzBuilder<DI, D, RI, DOMAIN>
{
    fn state<const NEW_DOMAIN: bool>(self) -> FizzBuzzBuilder<DI, D, RI, NEW_DOMAIN> {
        FizzBuzzBuilder::<DI, D, RI, NEW_DOMAIN> {
            domain: self.domain,
            rules: self.rules,
        }
    }

    pub fn domain(mut self, domain: D) -> FizzBuzzBuilder<DI, D, RI, true> {
        self.domain = Some(domain);
        self.state::<true>()
    }

    pub fn add_rule(mut self, rule: impl Fn(DI) -> RI + 'static) -> Self {
        self.rules.push(Box::new(rule));
        self
    }

    pub fn rules(mut self, rules: Vec<Box<dyn Fn(DI) -> RI>>) -> Self {
        self.rules = rules;
        self
    }
}

impl<DI: DomainItem, D: Domain<DI, RI>, RI: RangeItem<DI>> FizzBuzzBuilder<DI, D, RI, true> {
    pub fn build(self) -> FizzBuzz<DI, D, RI> {
        FizzBuzz {
            domain: self.domain.expect("Used typestate to ensure success."),
            rules: self.rules,
        }
    }
}
