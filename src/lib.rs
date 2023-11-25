pub mod builder;
#[cfg(feature = "default_input")]
mod default_input;
#[cfg(feature = "default_output")]
mod default_output;
pub mod iter;
pub mod traits;

pub use builder::FizzBuzzBuilder;
pub use traits::{FizzBuzzable, FizzBuzzed};

use crate::iter::FizzBuzzIter;
use std::collections::BTreeMap;
use std::iter::Map;

pub struct FizzBuzz<I, O>
where
    I: FizzBuzzable<O>,
    O: FizzBuzzed<I>,
{
    pub start: I,
    pub end: I,
    pub map: BTreeMap<I, O>,
    pub rule: Box<dyn Fn(I, I) -> bool>,
}

impl<I: FizzBuzzable<O>, O: FizzBuzzed<I>> FizzBuzz<I, O> {
    pub fn result(&self, n: I) -> Vec<O> {
        O::from(n, &self.map, &self.rule)
    }

    pub fn iter(&self) -> FizzBuzzIter<'_, I, O> {
        FizzBuzzIter {
            start: self.start.clone(),
            end: self.end.clone(),
            map: &self.map,
            rule: &self.rule,
        }
    }

    pub fn iter_str(&self) -> Map<FizzBuzzIter<'_, I, O>, impl FnMut(Vec<O>) -> String> {
        self.iter().map(|vo| {
            vo.iter()
                .map(|o| o.to_string())
                .collect::<Vec<String>>()
                .join("")
        })
    }
}
