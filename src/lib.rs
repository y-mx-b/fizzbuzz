mod iter;

use std::fmt;
use std::collections::BTreeMap;
use std::iter::Map;
use crate::iter::FizzBuzzIter;
use std::ops;

pub trait FizzBuzzable<O: FizzBuzzed<Self>>: Clone + PartialEq + PartialOrd {
    fn succ(&self) -> Self;
    fn pred(&self) -> Self;
}

// TODO: provide impl for all number types

pub trait FizzBuzzed<I: FizzBuzzable<Self>>: fmt::Display + Sized + Clone {
    fn from(n: I, map: &BTreeMap<I, Self>, rule: &impl Fn(I, I) -> bool) -> Vec<Self>;
}

pub struct FizzBuzz<I, O, R: Fn(I, I) -> bool> 
where
    I: FizzBuzzable<O>,
    O: FizzBuzzed<I>,
{
    pub start: I,
    pub end: I,
    pub map: BTreeMap<I, O>,
    pub rule: R,
}

impl<I: FizzBuzzable<O>, O: FizzBuzzed<I>, R: Fn(I, I) -> bool> FizzBuzz<I, O, R> {
    pub fn result(&self, n: I) -> Vec<O> {
        O::from(n, &self.map, &self.rule)
    }

    pub fn iter(&self) -> FizzBuzzIter<'_, I, O, R> {
        FizzBuzzIter {
            start: self.start.clone(),
            end: self.end.clone(),
            map: &self.map,
            rule: &self.rule,
        }
    }

    pub fn iter_str(&self) -> Map<FizzBuzzIter<'_, I, O, R>, impl FnMut(Vec<O>) -> String> {
        self.iter().map(|vo| vo.iter().map(|o| o.to_string()).collect::<Vec<String>>().join(""))
    }
}