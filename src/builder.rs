use crate::FizzBuzz;
use crate::traits::*;
use std::collections::BTreeMap;
use crate::default;

pub struct FizzBuzzBuilder<I: FizzBuzzable<O>, O: FizzBuzzed<I>> {
    start: Option<I>,
    end: Option<I>,
    map: Option<BTreeMap<I, O>>,
    rule: Option<Box<dyn Fn(I, I) -> bool>>,
}

impl<I: FizzBuzzable<O>, O: FizzBuzzed<I>> Default for FizzBuzzBuilder<I, O> {
    fn default() -> Self {
        Self {
            start: None,
            end: None,
            map: None,
            rule: None
        }
    }
}

impl<I: FizzBuzzable<O>, O: FizzBuzzed<I>> FizzBuzzBuilder<I, O> {
    pub fn new() -> Self {
        Self::default()
    }
}

impl DefaultBuilder<i64, default::Output> for FizzBuzzBuilder<i64, default::Output> {
    fn default_map() -> BTreeMap<i64, default::Output> {
        BTreeMap::from([(3, default::Output::Fizz), (5, default::Output::Buzz)])
    }
    fn default_rule() -> Box<dyn Fn(i64, i64) -> bool> {
        Box::new(|n, divis| n % divis == 0)
    }
    fn build(self) -> FizzBuzz<i64, default::Output> {
        FizzBuzz {
            start: self.start.unwrap_or(1),
            end: self.end.unwrap_or(100),
            map: self.map.unwrap_or(Self::default_map()),
            rule: self.rule.unwrap_or(Self::default_rule()),
        }
    }
}