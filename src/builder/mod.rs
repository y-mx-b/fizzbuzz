use crate::traits::*;
use crate::FizzBuzz;
use std::marker::PhantomData;

/// A builder for the [FizzBuzz] struct.
pub struct FizzBuzzBuilder<DI: DomainItem, D: Domain<DI, RI>, RI: RangeItem<DI>, BuilderState> {
    pub(crate) _state: PhantomData<BuilderState>,
    pub(crate) domain: Option<D>,
    pub(crate) rules: Vec<Box<dyn Fn(DI) -> RI>>,
}

pub struct BuilderState<const MAP: bool, const DRIMAIN: bool>(PhantomData<bool>);

impl<DI: DomainItem, D: Domain<DI, RI>, RI: RangeItem<DI>>
    FizzBuzzBuilder<DI, D, RI, BuilderState<false, false>>
{
    pub fn new() -> Self {
        Self {
            _state: PhantomData,
            domain: None,
            rules: Vec::new(),
        }
    }
}

impl<
        DI: DomainItem,
        D: Domain<DI, RI>,
        RI: RangeItem<DI>,
        const MAP: bool,
        const DOMAIN: bool,
    > FizzBuzzBuilder<DI, D, RI, BuilderState<MAP, DOMAIN>>
{
    fn state<const NEW_MAP: bool, const NEW_DOMAIN: bool>(
        self,
    ) -> FizzBuzzBuilder<DI, D, RI, BuilderState<NEW_MAP, NEW_DOMAIN>> {
        FizzBuzzBuilder::<DI, D, RI, BuilderState<NEW_MAP, NEW_DOMAIN>> {
            _state: PhantomData,
            domain: self.domain,
            rules: self.rules,
        }
    }

    pub fn domain(mut self, domain: D) -> FizzBuzzBuilder<DI, D, RI, BuilderState<MAP, true>> {
        self.domain = Some(domain);
        self.state::<MAP, true>()
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

impl<DI: DomainItem, D: Domain<DI, RI>, RI: RangeItem<DI>>
    FizzBuzzBuilder<DI, D, RI, BuilderState<true, true>>
{
    pub fn build(self) -> FizzBuzz<DI, D, RI> {
        FizzBuzz {
            domain: self.domain.expect("Used typestate to ensure success."),
            rules: self.rules,
        }
    }
}
