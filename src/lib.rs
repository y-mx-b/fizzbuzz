mod iter;
mod traits;

pub use traits::{FizzBuzzable, FizzBuzzed};

use crate::iter::FizzBuzzIter;
use std::collections::BTreeMap;
use std::iter::Map;

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
        self.iter().map(|vo| {
            vo.iter()
                .map(|o| o.to_string())
                .collect::<Vec<String>>()
                .join("")
        })
    }
}
