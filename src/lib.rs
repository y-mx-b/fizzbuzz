pub mod builder;
mod default_input;
#[cfg(any(feature = "signed_output", feature = "unsigned_output"))]
pub mod default_output;
pub mod traits;

pub use builder::FizzBuzzBuilder;
pub use traits::*;

///
///
/// The default implementations for Rust's primitive integer types are available
/// by default. Matching default output types are provided as well.
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
    D: Domain<DI, RI>,
    RI: RangeItem<DI>,
{
    domain: D,
    rules: Vec<Box<dyn Fn(DI) -> RI>>,
}

impl<DI: DomainItem, D: Domain<DI, RI>, RI: RangeItem<DI>> FizzBuzz<DI, D, RI> {
    /// Evaluate the output of a given input.
    ///
    /// # Example
    /// ```rust
    /// # use fizzbuzz::*;
    /// let fb: FizzBuzz<u32, _,  _> = FizzBuzzBuilder::default().domain(1..100).build();
    /// let result = fb.result(10).join("");
    /// assert_eq!(result, "buzz");
    /// ```
    pub fn result(&self, n: DI) -> Vec<RI> {
        RI::from(n, &self.rules)
    }
}

impl<DI: DomainItem, D: Domain<DI, RI>, RI: RangeItem<DI>> Iterator for FizzBuzz<DI, D, RI> {
    type Item = Vec<RI>;

    fn next(&mut self) -> Option<Self::Item> {
        self.domain
            .next()
            .and_then(|i| Some(RI::from(i, &self.rules)))
    }
}

impl<DI: DomainItem, D: Domain<DI, RI> + DoubleEndedIterator, RI: RangeItem<DI>> DoubleEndedIterator
    for FizzBuzz<DI, D, RI>
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.domain
            .next_back()
            .and_then(|i| Some(RI::from(i, &self.rules)))
    }
}
