use crate::{FizzBuzzable, FizzBuzzed};
use std::collections::BTreeMap;
use std::iter;

pub struct FizzBuzzIter<'a, I: FizzBuzzable<O>, O: FizzBuzzed<I>, R: Fn(I, I) -> bool> {
    pub(crate) start: I,
    pub(crate) end: I,
    pub(crate) map: &'a BTreeMap<I, O>,
    pub(crate) rule: &'a R,
}

impl<'a, I: FizzBuzzable<O>, O: FizzBuzzed<I>, R: Fn(I, I) -> bool> Iterator
    for FizzBuzzIter<'a, I, O, R>
{
    type Item = Vec<O>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start > self.end {
            return None;
        }

        let output = Some(O::from(self.start.clone(), &self.map, &self.rule));
        self.start = self.start.succ();

        output
    }
}

impl<'a, I: FizzBuzzable<O>, O: FizzBuzzed<I>, R: Fn(I, I) -> bool> iter::DoubleEndedIterator
    for FizzBuzzIter<'a, I, O, R>
{
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.start > self.end {
            return None;
        }

        let output = Some(O::from(self.end.clone(), &self.map, &self.rule));
        self.end = self.end.pred();

        output
    }
}
