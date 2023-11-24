use std::fmt;
use std::collections::BTreeMap;

pub trait FizzBuzzed: fmt::Display + Sized + Clone {
    fn from(n: i64, map: &BTreeMap<i64, Self>, rule: impl Fn(i64, i64) -> bool) -> Vec<Self>;
}

pub fn fizzbuzz<O: FizzBuzzed>(n: i64, map: &BTreeMap<i64, O>, rule: impl Fn(i64, i64) -> bool) -> Vec<O> {
    O::from(n, map, rule)
}
