mod error;

pub use error::FizzBuzzBuilderError;

use crate::traits::*;
use crate::FizzBuzz;
use std::marker::PhantomData;

/// A builder for the [FizzBuzz] struct.
pub struct FizzBuzzBuilder<T: DomainItem, I: Domain<T, O>, O: RangeItem<T>, BuilderState> {
    pub(crate) _state: PhantomData<BuilderState>,
    pub(crate) domain: Option<I>,
    pub(crate) rules: Vec<Box<dyn Fn(T) -> O>>,
}

pub struct BuilderState<const MAP: bool, const RULE: bool, const DOMAIN: bool>(PhantomData<bool>);

impl<T: DomainItem, I: Domain<T, O>, O: RangeItem<T>>
    FizzBuzzBuilder<T, I, O, BuilderState<false, false, false>>
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
        T: DomainItem,
        I: Domain<T, O>,
        O: RangeItem<T>,
        const MAP: bool,
        const RULE: bool,
        const DOMAIN: bool,
    > FizzBuzzBuilder<T, I, O, BuilderState<MAP, RULE, DOMAIN>>
{
    fn state<const M: bool, const R: bool, const D: bool>(
        self,
    ) -> FizzBuzzBuilder<T, I, O, BuilderState<M, R, D>> {
        FizzBuzzBuilder::<T, I, O, BuilderState<M, R, D>> {
            _state: PhantomData,
            domain: self.domain,
            rules: self.rules,
        }
    }

    pub fn domain(mut self, domain: I) -> FizzBuzzBuilder<T, I, O, BuilderState<MAP, RULE, true>> {
        self.domain = Some(domain);
        self.state::<MAP, RULE, true>()
    }

    pub fn add_rule(mut self, rule: impl Fn(T) -> O + 'static) -> Self {
        self.rules.push(Box::new(rule));
        self
    }

    pub fn rules(mut self, rules: Vec<Box<dyn Fn(T) -> O>>) -> Self {
        self.rules = rules;
        self
    }
}

impl<T: DomainItem, I: Domain<T, O>, O: RangeItem<T>>
    FizzBuzzBuilder<T, I, O, BuilderState<true, true, true>>
{
    pub fn build(self) -> FizzBuzz<T, I, O> {
        FizzBuzz {
            domain: self.domain.expect("Used typestate to ensure success."),
            rules: self.rules,
        }
    }
}
