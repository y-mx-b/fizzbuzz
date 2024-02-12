//! # FizzBuzz
//!
//! A very serious implementation of FizzBuzz in Rust.
//!
//! # Why?
//!
//! Given that FizzBuzz is really just a mapping between two sets, I decided
//! to try to create as general a solution to FizzBuzz as possible. Thus, this
//! monstrosity of generics galore was born.
//!
//! # Usage
//!
//! Here's a classic FizzBuzz solution using this library.
//!
//! ```rust
//! use fizzbuzz::*;
//!
//! fn main() {
//!     let fb = FizzBuzzBuilder::default().domain(1..100).build();
//!
//!     for i in fb {
//!         println!("{}", i.join(""));
//!     }
//! }
//! ```
//!
//! Given that the crate, by default, provides implementations for all of
//! Rust's integer primitives, you can use the
//! [default constructor](crate::default_builder) to create a new builder
//! and then provide a range for the [domain]. After that, just call the
//! [build](crate::FizzBuzzBuilder::build) method to create a new [FizzBuzz]
//! object and iterate over it. Use the [join](crate::RangeVariant::join)
//! method to get a string representation and tada! FizzBuzz!

pub mod builder;
pub mod default_builder;
#[cfg(any(feature = "signed_output", feature = "unsigned_output"))]
pub mod default_output;
pub mod domain;
pub mod range;
pub mod rule;
pub mod traits;

pub use builder::FizzBuzzBuilder;
pub use default_builder::DefaultBuilder;
pub use domain::{Domain, DomainItem};
pub use range::{RangeItem, RangeVariant};
pub use rule::Rule;

/// An iterator that maps a given set ([Domain]) to a set of [RangeItem] according
/// to a given set of rules.
///
/// Default implementations for Rust's primitive integer types are available
/// by default, with default output types provided. Refer to the [default_output]
/// module for more information.
///
/// # Example
///
/// ```rust
/// # use fizzbuzz::*;
/// let fb: FizzBuzz<u32, _, _> = FizzBuzzBuilder::default().domain(1..100).build();
/// for v in fb {
///     println!("{:?}", v);
/// }
/// ```
pub struct FizzBuzz<DI, D, RI>
where
    DI: DomainItem,
    D: Domain<DI>,
    RI: RangeItem,
{
    domain: D,
    rules: Vec<Rule<DI, RI>>,
}

impl<DI: DomainItem, D: Domain<DI>, RI: RangeItem> FizzBuzz<DI, D, RI> {
    /// Evaluate the output of a given input.
    ///
    /// # Example
    /// ```rust
    /// # use fizzbuzz::*;
    /// let fb: FizzBuzz<u32, _,  _> = FizzBuzzBuilder::default().domain(1..100).build();
    /// let result = fb.result(10).join("");
    /// assert_eq!(result, "buzz");
    /// ```
    pub fn result(&self, n: DI) -> RangeVariant<DI, RI> {
        RangeVariant::from(n, &self.rules())
    }

    pub fn rules(&self) -> &[Rule<DI, RI>] {
        &self.rules
    }
}

impl<DI: DomainItem, D: Domain<DI>, RI: RangeItem> Iterator for FizzBuzz<DI, D, RI> {
    type Item = RangeVariant<DI, RI>;

    fn next(&mut self) -> Option<Self::Item> {
        self.domain
            .next()
            .and_then(|i| Some(RangeVariant::from(i, &self.rules())))
    }
}

impl<DI: DomainItem, D: Domain<DI> + DoubleEndedIterator, RI: RangeItem> DoubleEndedIterator
    for FizzBuzz<DI, D, RI>
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.domain
            .next_back()
            .and_then(|i| Some(RangeVariant::from(i, &self.rules())))
    }
}
