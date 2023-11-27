mod error;

pub use error::FizzBuzzBuilderError;

use crate::traits::*;
use std::collections::BTreeMap;
use std::ops::{RangeBounds, Bound};

/// A builder for the [FizzBuzz](crate::FizzBuzz) struct.
pub struct FizzBuzzBuilder<I: FizzBuzzable<O>, O: FizzBuzzed<I>> {
    pub(crate) start: Option<I>,
    pub(crate) end: Option<I>,
    pub(crate) map: Option<BTreeMap<I, O>>,
    pub(crate) rule: Option<Box<dyn Fn(I, I) -> bool>>,
}

impl<I: FizzBuzzable<O>, O: FizzBuzzed<I>> Default for FizzBuzzBuilder<I, O> {
    fn default() -> Self {
        Self {
            start: None,
            end: None,
            map: None,
            rule: None,
        }
    }
}

impl<I: FizzBuzzable<O>, O: FizzBuzzed<I>> FizzBuzzBuilder<I, O> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn start_at(mut self, start: I) -> Self {
        self.start = Some(start);
        self
    }

    pub fn end_at(mut self, end: I) -> Self {
        self.end = Some(end);
        self
    }

    pub fn range<R: RangeBounds<I>>(mut self, range: R) -> Result<Self, FizzBuzzBuilderError> {
        let validate = |b: Bound<&I>, d, is_start: bool| {
            match b {
                Bound::Included(n) => Ok(Some(n.clone())),
                Bound::Excluded(n) => Ok(Some(if is_start { n.succ() } else { n.pred() })),
                Bound::Unbounded => { 
                    match d {
                        Some(n) => Ok(Some(n)),
                        None => Err(FizzBuzzBuilderError::InvalidUnboundedStart)
                    }
                }
            }
        };

        self.start = validate(range.start_bound(), <I as FizzBuzzable<O>>::min(), true)?;
        self.end = validate(range.end_bound(), <I as FizzBuzzable<O>>::max(), false)?;

        Ok(self)
    }

    pub fn add_mapping(mut self, input: I, output: O) -> Self {
        match &mut self.map {
            Some(m) => {
                m.insert(input, output);
            }
            None => {
                self.map = Some(BTreeMap::from([(input, output)]));
            }
        }

        self
    }

    pub fn set_map<const N: usize>(mut self, map: [(I, O); N]) -> Self {
        self.map = Some(BTreeMap::from(map));
        self
    }

    pub fn set_rule(mut self, rule: impl Fn(I, I) -> bool + 'static) -> Self {
        self.rule = Some(Box::new(rule));
        self
    }

    pub fn build() {}
}
