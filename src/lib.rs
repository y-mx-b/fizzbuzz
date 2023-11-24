use std::fmt;
use std::collections::BTreeMap;

pub trait FizzBuzzed: fmt::Display + Sized + Clone {
    fn from(n: i64, map: &BTreeMap<i64, Self>, rule: &impl Fn(i64, i64) -> bool) -> Vec<Self>;
}

pub struct FizzBuzz<O: FizzBuzzed, R: Fn(i64, i64) -> bool> {
    pub start: i64,
    pub end: i64,
    pub map: BTreeMap<i64, O>,
    pub rule: R,
}

impl<O: FizzBuzzed, R: Fn(i64, i64) -> bool> FizzBuzz<O, R> {
    pub fn result(&self, n: i64) -> Vec<O> {
        O::from(n, &self.map, &self.rule)
    }
}