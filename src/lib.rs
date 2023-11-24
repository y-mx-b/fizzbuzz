mod iter;

use std::fmt;
use std::collections::BTreeMap;
use std::iter::Map;
use crate::iter::FizzBuzzIter;

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

    pub fn iter(&self) -> FizzBuzzIter<'_, O, R> {
        FizzBuzzIter {
            start: self.start,
            end: self.end,
            map: &self.map,
            rule: &self.rule,
        }
    }

    pub fn iter_str(&self) -> Map<FizzBuzzIter<'_, O, R>, impl FnMut(Vec<O>) -> String> {
        self.iter().map(|vo| vo.iter().map(|o| o.to_string()).collect::<Vec<String>>().join(""))
    }
}