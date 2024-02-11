use crate::traits::*;
use crate::FizzBuzz;

/// A builder for the [FizzBuzz] struct.
///
/// This builder uses the
/// [Typestate](https://willcrichton.net/rust-api-type-patterns/typestate.html)
/// pattern to guarantee a build will not fail. Thus, you can only call the
/// [build](crate::FizzBuzzBuilder::build) method once the builder is properly
pub struct FizzBuzzBuilder<DI: DomainItem, D: Domain<DI, RI>, RI: RangeItem<DI>, const DOMAIN: bool>
{
    pub(crate) domain: Option<D>,
    pub(crate) rules: Vec<Box<dyn Fn(DI) -> Option<RI>>>,
}

impl<DI: DomainItem, D: Domain<DI, RI>, RI: RangeItem<DI>> FizzBuzzBuilder<DI, D, RI, false> {
    /// Create a new, empty builder.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// # use fizzbuzz::*;
    /// # use fizzbuzz::default_output::Fromu32;
    /// let fb = FizzBuzzBuilder::new()
    ///             .domain(1..100)
    ///             .rule(|n| if n % 3 == 0 { Fromu32::Fizz } else { Fromu32::Num(n) })
    ///             .build();
    /// for i in fb {
    ///     println!("{}", i.join(""));
    /// }
    /// ```
    pub fn new() -> Self {
        Self {
            domain: None,
            rules: Vec::new(),
        }
    }
}

impl<DI: DomainItem, D: Domain<DI, RI>, RI: RangeItem<DI>, const DOMAIN: bool>
    FizzBuzzBuilder<DI, D, RI, DOMAIN>
{
    /// Explicitly mark the builder as either having a domain or not having a domain.
    fn state<const NEW_DOMAIN: bool>(self) -> FizzBuzzBuilder<DI, D, RI, NEW_DOMAIN> {
        FizzBuzzBuilder::<DI, D, RI, NEW_DOMAIN> {
            domain: self.domain,
            rules: self.rules,
        }
    }

    /// Set the domain upon which [FizzBuzz] will act upon.
    pub fn domain(mut self, domain: D) -> FizzBuzzBuilder<DI, D, RI, true> {
        self.domain = Some(domain);
        self.state::<true>()
    }

    /// Add a new rule.
    pub fn rule(mut self, rule: impl Fn(DI) -> Option<RI> + 'static) -> Self {
        self.rules.push(Box::new(rule));
        self
    }

    /// Overwrite the current rules with the given ones.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// # use fizzbuzz::*;
    /// # use fizzbuzz::default_output::Fromu32;
    /// let fb = FizzBuzzBuilder::new()
    ///             .domain(1..100)
    ///             .rules(vec![
    ///                 Box::new(|n| if n % 3 == 0 { Fromu32::Fizz } else { Fromu32::Num(n) })
    ///             ])
    ///             .build();
    /// for i in fb {
    ///     println!("{}", i.join(""));
    /// }
    /// ```
    pub fn rules(mut self, rules: Vec<Box<dyn Fn(DI) -> Option<RI>>>) -> Self {
        self.rules = rules;
        self
    }
}

impl<DI: DomainItem, D: Domain<DI, RI>, RI: RangeItem<DI>> FizzBuzzBuilder<DI, D, RI, true> {
    pub fn build(self) -> FizzBuzz<DI, D, RI> {
        FizzBuzz {
            domain: self.domain.expect("Used typestate to ensure success."),
            rules: self.rules,
        }
    }
}
