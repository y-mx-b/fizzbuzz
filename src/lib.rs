pub mod builder;
pub mod traits;
#[cfg(any(feature = "signed_output", feature = "unsigned_output"))]
pub mod default_output;
mod default_input;

pub use builder::{FizzBuzzBuilder, FizzBuzzBuilderError};
pub use traits::*;

use std::{collections::BTreeMap, marker::PhantomData};

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
/// let fb: FizzBuzz<u32, _, _> = FizzBuzzBuilder::default().build();
/// for v in fb {
///     println!("{:?}", v);
/// }
/// ```
pub struct FizzBuzz<T, I, O>
where
    T: FizzBuzzableItem,
    I: FizzBuzzable<T, O>,
    O: FizzBuzzed<T, I>,
{
    domain: I,
    map: BTreeMap<T, O>,
    rule: Box<dyn Fn(T, T) -> bool>,
}

impl<T: FizzBuzzableItem, I: FizzBuzzable<T, O>, O: FizzBuzzed<T, I>> FizzBuzz<T, I, O> {
    /// Evaluate the output of a given input.
    ///
    /// # Example
    /// ```rust
    /// # use fizzbuzz::*;
    /// let fb: FizzBuzz<u32, _,  _> = FizzBuzzBuilder::default().build();
    /// let result = fb.result(10).join("");
    /// assert_eq!(result, "buzz");
    /// ```
    pub fn result(&self, n: T) -> Vec<O> {
        O::from(n, &self.map, &self.rule)
    }
}

impl<T: FizzBuzzableItem, I: FizzBuzzable<T, O>, O: FizzBuzzed<T, I>> Iterator for FizzBuzz<T, I, O> {
    type Item = Vec<O>;

    fn next(&mut self) -> Option<Self::Item> {
        self.domain.next().and_then(|i| Some(O::from(i, &self.map, &self.rule)))
    }
}

// TODO: impl doubleendediterator