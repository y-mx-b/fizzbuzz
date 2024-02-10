mod error;

pub use error::FizzBuzzBuilderError;

use crate::traits::*;
use crate::FizzBuzz;
use std::collections::BTreeMap;
use std::marker::PhantomData;
// use std::ops::{Bound, RangeBounds};

/// A builder for the [FizzBuzz] struct.
pub struct FizzBuzzBuilder<T, I: FizzBuzzable<T, O>, O: FizzBuzzed<T, I>, BuilderState> {
    pub(crate) _state: PhantomData<BuilderState>,
    pub(crate) domain: Option<I>,
    pub(crate) map: Option<BTreeMap<I, O>>,
    pub(crate) rule: Option<Box<dyn Fn(I, I) -> bool>>,
}

pub struct BuilderState<const MAP: bool, const RULE: bool, const START: bool, const END: bool>(
    PhantomData<bool>,
);

impl<T, I: FizzBuzzable<T, O>, O: FizzBuzzed<T, I>>
    FizzBuzzBuilder<T, I, O, BuilderState<false, false, false, false>>
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
        T,
        I: FizzBuzzable<T, O>,
        O: FizzBuzzed<T, I>,
        const MAP: bool,
        const RULE: bool,
        const START: bool,
        const END: bool,
    > FizzBuzzBuilder<T, I, O, BuilderState<MAP, RULE, START, END>>
{
    fn set_state<const M: bool, const R: bool, const S: bool, const E: bool>(
        self,
    ) -> FizzBuzzBuilder<T, I, O, BuilderState<M, R, S, E>> {
        FizzBuzzBuilder::<T, I, O, BuilderState<M, R, S, E>> {
            _state: PhantomData,
            domain: self.domain,
            map: self.map,
            rule: self.rule,
        }
    }

    // pub fn range<R: RangeBounds<I>>(
    //     mut self,
    //     range: R,
    // ) -> Result<FizzBuzzBuilder<I, O, BuilderState<MAP, RULE, true, true>>, FizzBuzzBuilderError>
    // {
    //     let validate = |b: Bound<&I>, d, is_start: bool| match b {
    //         Bound::Included(n) => Ok(Some(n.clone())),
    //         Bound::Excluded(n) => Ok(Some(if is_start { n.succ() } else { n.pred() })),
    //         Bound::Unbounded => match d {
    //             Some(n) => Ok(Some(n)),
    //             None => {
    //                 if is_start {
    //                     Err(FizzBuzzBuilderError::InvalidUnboundedStart)
    //                 } else {
    //                     Err(FizzBuzzBuilderError::InvalidUnboundedEnd)
    //                 }
    //             }
    //         },
    //     };

    //     let start = validate(range.start_bound(), <I as FizzBuzzable<O>>::min(), true);
    //     let end = validate(range.end_bound(), <I as FizzBuzzable<O>>::max(), false);

    //     match (start, end) {
    //         (Ok(s), Ok(e)) => {
    //             self.start = s;
    //             self.end = e;

    //             Ok(Self::set_state::<MAP, RULE, true, true>(self))
    //         }
    //         (Ok(s), Err(err)) => {
    //             self.start = s;
    //             Err(err)
    //         }
    //         (Err(err), Ok(e)) => {
    //             self.end = e;
    //             Err(err)
    //         }
    //         (Err(e1), Err(e2)) => Err(FizzBuzzBuilderError::InvalidUnboundedBounds),
    //     }
    // }

    pub fn add_mapping(
        mut self,
        input: I,
        output: O,
    ) -> FizzBuzzBuilder<T, I, O, BuilderState<true, RULE, START, END>> {
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
    ) -> FizzBuzzBuilder<T, I, O, BuilderState<true, RULE, START, END>> {
        self.map = Some(BTreeMap::from(map));
        Self::set_state::<true, RULE, START, END>(self)
    }

    pub fn set_rule(mut self, rule: impl Fn(I, I) -> bool + 'static) -> Self {
        self.rule = Some(Box::new(rule));
        self
    }
}

impl<T, I: FizzBuzzable<T, O>, O: FizzBuzzed<T, I>>
    FizzBuzzBuilder<T, I, O, BuilderState<true, true, true, true>>
{
    pub fn build(self) -> FizzBuzz<T, I, O> {
        FizzBuzz {
            domain: self.domain.expect("Used typestate to ensure success."),
            map: self.map.expect("Used typestate to ensure success."),
            rule: self.rule.expect("Used typestate to ensure success."),
        }
    }
}
