use crate::{FizzBuzzable, FizzBuzzed};
use std::collections::BTreeMap;
use std::iter;

pub struct FizzBuzzIter<'a, I: FizzBuzzable<O>, O: FizzBuzzed<I>> {
    pub(crate) current: I,
    pub(crate) end: I,
    pub(crate) map: &'a BTreeMap<I, O>,
    pub(crate) rule: &'a dyn Fn(I, I) -> bool,
}

impl<'a, I: FizzBuzzable<O>, O: FizzBuzzed<I>> Iterator for FizzBuzzIter<'a, I, O> {
    type Item = Vec<O>;

    fn next(&mut self) -> Option<Self::Item> {
        // TODO: handle overflow somehow
        if self.current > self.end {
            return None;
        }

        let output = Some(O::from(self.current.clone(), &self.map, &self.rule));
        self.current = self.current.succ(); 

        output
    }
}

impl<'a, I: FizzBuzzable<O>, O: FizzBuzzed<I>> iter::DoubleEndedIterator
    for FizzBuzzIter<'a, I, O>
{
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.current > self.end {
            return None;
        }

        let output = Some(O::from(self.end.clone(), &self.map, &self.rule));
        self.end = self.end.pred();

        output
    }
}
