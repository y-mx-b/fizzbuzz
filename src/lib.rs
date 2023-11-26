pub mod builder;
#[cfg(feature = "default_input")]
pub mod default_input;
#[cfg(feature = "default_output")]
pub mod default_output;
pub mod iter;
pub mod traits;

pub use builder::FizzBuzzBuilder;
pub use traits::*;

use crate::iter::FizzBuzzIter;
use std::collections::BTreeMap;
use std::iter::Map;

/// An iterable type that applies a rule to a range of inputs and returns a
/// vector of the resulting outputs that match which rule applications matched
/// the input.
///
/// The default implementations for Rust's primitive integer types are available
/// by default. Matching default output types are provided as well.
///
/// # Example
///
/// ```rust
/// # use fizzbuzz::traits::*;
/// # use fizzbuzz::{FizzBuzzBuilder, FizzBuzz};
/// let fb: FizzBuzz<i32, _> = FizzBuzzBuilder::new().build();
/// for v in fb.iter() {
///     println!("{:?}", v);
/// }
/// ```
pub struct FizzBuzz<I, O>
where
    I: FizzBuzzable<O>,
    O: FizzBuzzed<I>,
{
    start: I,
    end: I,
    map: BTreeMap<I, O>,
    rule: Box<dyn Fn(I, I) -> bool>,
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
