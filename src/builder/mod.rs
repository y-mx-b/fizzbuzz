mod error;

pub use error::FizzBuzzBuilderError;

use crate::traits::*;
use crate::FizzBuzz;
use std::collections::BTreeMap;
use std::marker::PhantomData;
use std::ops::{Bound, RangeBounds};

/// A builder for the [FizzBuzz](crate::FizzBuzz) struct.
pub struct FizzBuzzBuilder<I: FizzBuzzable<O>, O: FizzBuzzed<I>, BuilderState> {
    pub(crate) _state: PhantomData<BuilderState>,
    pub(crate) start: Option<I>,
    pub(crate) end: Option<I>,
    pub(crate) map: Option<BTreeMap<I, O>>,
    pub(crate) rule: Option<Box<dyn Fn(I, I) -> bool>>,
}

pub struct BuilderState<const MAP: bool, const RULE: bool, const START: bool, const END: bool>(
    PhantomData<bool>,
);

impl<I: FizzBuzzable<O>, O: FizzBuzzed<I>>
    FizzBuzzBuilder<I, O, BuilderState<false, false, false, false>>
{
    pub fn new() -> Self {
        Self {
            _state: PhantomData,
            start: None,
            end: None,
            map: None,
            rule: None,
        }
    }
}

impl<
        I: FizzBuzzable<O>,
        O: FizzBuzzed<I>,
        const MAP: bool,
        const RULE: bool,
        const START: bool,
        const END: bool,
    > FizzBuzzBuilder<I, O, BuilderState<MAP, RULE, START, END>>
{
    pub fn set_state<const M: bool, const R: bool, const S: bool, const E: bool>(
        self,
    ) -> FizzBuzzBuilder<I, O, BuilderState<M, R, S, E>> {
        FizzBuzzBuilder::<I, O, BuilderState<M, R, S, E>> {
            _state: PhantomData,
            map: self.map,
            rule: self.rule,
            start: self.start,
            end: self.end,
        }
    }

    pub fn range<R: RangeBounds<I>>(
        mut self,
        range: R,
    ) -> Result<FizzBuzzBuilder<I, O, BuilderState<MAP, RULE, true, true>>, FizzBuzzBuilderError>
    {
        let validate = |b: Bound<&I>, d, is_start: bool| match b {
            Bound::Included(n) => Ok(Some(n.clone())),
            Bound::Excluded(n) => Ok(Some(if is_start { n.succ() } else { n.pred() })),
            Bound::Unbounded => match d {
                Some(n) => Ok(Some(n)),
                None => Err(FizzBuzzBuilderError::InvalidUnboundedStart),
            },
        };

        self.start = validate(range.start_bound(), <I as FizzBuzzable<O>>::min(), true)?;
        self.end = validate(range.end_bound(), <I as FizzBuzzable<O>>::max(), false)?;

        Ok(Self::set_state::<MAP, RULE, true, true>(self))
    }

    pub fn add_mapping(
        mut self,
        input: I,
        output: O,
    ) -> FizzBuzzBuilder<I, O, BuilderState<true, RULE, START, END>> {
        match &mut self.map {
            Some(m) => {
                m.insert(input, output);
            }
            None => {
                self.map = Some(BTreeMap::from([(input, output)]));
            }
        }

        Self::set_state::<true, RULE, START, END>(self)
    }

    pub fn set_map<const N: usize>(
        mut self,
        map: [(I, O); N],
    ) -> FizzBuzzBuilder<I, O, BuilderState<true, RULE, START, END>> {
        self.map = Some(BTreeMap::from(map));
        Self::set_state::<true, RULE, START, END>(self)
    }

    pub fn set_rule(mut self, rule: impl Fn(I, I) -> bool + 'static) -> Self {
        self.rule = Some(Box::new(rule));
        self
    }
}

impl<I: FizzBuzzable<O>, O: FizzBuzzed<I>>
    FizzBuzzBuilder<I, O, BuilderState<true, true, true, true>>
{
    pub fn build(self) -> FizzBuzz<I, O> {
        FizzBuzz {
            start: self.start.expect("Used typestate to ensure success."),
            end: self.end.expect("Used typestate to ensure success."),
            map: self.map.expect("Used typestate to ensure success."),
            rule: self.rule.expect("Used typestate to ensure success."),
        }
    }
}
