mod error;

pub use error::FizzBuzzBuilderError;

use crate::traits::*;
use crate::FizzBuzz;
use std::collections::BTreeMap;
use std::marker::PhantomData;

/// A builder for the [FizzBuzz] struct.
pub struct FizzBuzzBuilder<T: FizzBuzzableItem, I: FizzBuzzable<T, O>, O: FizzBuzzed<T>, BuilderState> {
    pub(crate) _state: PhantomData<BuilderState>,
    pub(crate) domain: Option<I>,
    pub(crate) map: Option<BTreeMap<T, O>>,
    pub(crate) rule: Option<Box<dyn Fn(T, T) -> bool>>,
}

pub struct BuilderState<const MAP: bool, const RULE: bool, const DOMAIN: bool>(
    PhantomData<bool>,
);

impl<T: FizzBuzzableItem, I: FizzBuzzable<T, O>, O: FizzBuzzed<T>>
    FizzBuzzBuilder<T, I, O, BuilderState<false, false, false>>
{
    pub fn new() -> Self {
        Self {
            _state: PhantomData,
            domain: None,
            map: None,
            rule: None,
        }
    }
}

impl<
        T: FizzBuzzableItem,
        I: FizzBuzzable<T, O>,
        O: FizzBuzzed<T>,
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
            map: self.map,
            rule: self.rule,
        }
    }

    pub fn domain(
        mut self,
        domain: I,
    ) -> FizzBuzzBuilder<T, I, O, BuilderState<MAP, RULE, true>>
    {
        self.domain = Some(domain);
        self.state::<MAP, RULE, true>()
    }

    pub fn add_mapping(
        mut self,
        input: T,
        output: O,
    ) -> FizzBuzzBuilder<T, I, O, BuilderState<true, RULE, DOMAIN>> {
        match &mut self.map {
            Some(m) => {
                m.insert(input, output);
            }
            None => {
                self.map = Some(BTreeMap::from([(input, output)]));
            }
        }

        Self::state::<true, RULE, DOMAIN>(self)
    }

    pub fn map<const N: usize>(
        mut self,
        map: [(T, O); N],
    ) -> FizzBuzzBuilder<T, I, O, BuilderState<true, RULE, DOMAIN>> {
        self.map = Some(BTreeMap::from(map));
        Self::state::<true, RULE, DOMAIN>(self)
    }

    pub fn rule(mut self, rule: impl Fn(T, T) -> bool + 'static) -> Self {
        self.rule = Some(Box::new(rule));
        self
    }
}

impl<T: FizzBuzzableItem, I: FizzBuzzable<T, O>, O: FizzBuzzed<T>>
    FizzBuzzBuilder<T, I, O, BuilderState<true, true, true>>
{
    pub fn build(self) -> FizzBuzz<T, I, O> {
        FizzBuzz {
            domain: self.domain.expect("Used typestate to ensure success."),
            map: self.map.expect("Used typestate to ensure success."),
            rule: self.rule.expect("Used typestate to ensure success."),
        }
    }
}
