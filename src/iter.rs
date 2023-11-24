use crate::FizzBuzzed;
use std::collections::BTreeMap;
use std::iter;

pub struct FizzBuzzIter<'a, O: FizzBuzzed, R: Fn(i64, i64) -> bool> {
    pub(crate) start: i64,
    pub(crate) end: i64,
    pub(crate) map: &'a BTreeMap<i64, O>,
    pub(crate) rule: &'a R,
}

impl<'a, O: FizzBuzzed, R: Fn(i64, i64) -> bool> Iterator for FizzBuzzIter<'a, O, R> {
    type Item = Vec<O>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start > self.end {
            return None;
        }

        let output = Some(O::from(self.start, &self.map, &self.rule));
        self.start += 1;

        output
    }
}

impl<'a, O: FizzBuzzed, R: Fn(i64, i64) -> bool> iter::DoubleEndedIterator for FizzBuzzIter<'a, O, R> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.start > self.end {
            return None;
        }

        let output = Some(O::from(self.end, &self.map, &self.rule));
        self.end -= 1;

        output
    }
}