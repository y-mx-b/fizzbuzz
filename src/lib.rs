pub mod builder;
#[cfg(feature = "default_input")]
pub mod default_input;
#[cfg(feature = "default_output")]
pub mod default_output;
pub mod iter;
pub mod traits;

pub use builder::{FizzBuzzBuilder, FizzBuzzBuilderError};
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
/// # use fizzbuzz::*;
/// let fb: FizzBuzz<u32, _> = FizzBuzzBuilder::default().build();
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
    /// Evaluate the output of a given input.
    ///
    /// # Example
    /// ```rust
    /// # use fizzbuzz::*;
    /// let fb: FizzBuzz<u32, _> = FizzBuzzBuilder::default().build();
    /// let result = fb.result(10).join("");
    /// assert_eq!(result, "buzz");
    /// ```
    pub fn result(&self, n: I) -> Vec<O> {
        O::from(n, &self.map, &self.rule)
    }

    pub fn iter(&self) -> FizzBuzzIter<'_, I, O> {
        FizzBuzzIter {
            current: self.start.clone(),
            end: self.end.clone(),
            map: &self.map,
            rule: &self.rule,
        }
    }

    pub fn iter_str<'a>(
        &self,
        sep: &'a str,
    ) -> Map<FizzBuzzIter<'_, I, O>, impl FnMut(Vec<O>) -> String + 'a> {
        self.iter().map(|vo| {
            vo.iter()
                .map(|o| o.to_string())
                .collect::<Vec<String>>()
                .join(sep)
        })
    }
}
