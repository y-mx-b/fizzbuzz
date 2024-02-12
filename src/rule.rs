use crate::*;

/// A rule used by the [FizzBuzz] iterator to determine the output of a given input.
///
/// # But why?
///
/// Why did I create a completely new type to house functions? Because I can, deal with it.
pub struct Rule<DI: DomainItem, RI: RangeItem>(pub(crate) Box<dyn Fn(&DI) -> Option<RI>>);

impl<DI: DomainItem, RI: RangeItem> Rule<DI, RI> {
    /// Call the [Rule] with the given [DomainItem].
    ///
    /// ```rust
    /// # use fizzbuzz::*;
    /// let rule = Rule::from(|n: &_| Some(n + 1));
    /// assert_eq!(rule.call(&2), Some(3));
    /// ```
    pub fn call(&self, di: &DI) -> Option<RI> {
        self.0(di)
    }
}

impl<DI: DomainItem, RI: RangeItem, F: Fn(&DI) -> Option<RI> + 'static> From<F> for Rule<DI, RI> {
    fn from(value: F) -> Self {
        Rule(Box::new(value))
    }
}

impl<'a, DI: DomainItem, RI: RangeItem> AsRef<dyn Fn(&DI) -> Option<RI> + 'a> for Rule<DI, RI> {
    fn as_ref(&self) -> &(dyn Fn(&DI) -> Option<RI> + 'a) {
        self.0.as_ref()
    }
}

/// Create a vector of [Rule] objects.
///
/// # Example
///
/// ```rust
/// # use fizzbuzz::*;
/// # use fizzbuzz::default_output::Fromu32;
/// let fb: FizzBuzz<u32, _, _> = FizzBuzzBuilder::default()
///     .domain(1..=100)
///     .rules(rules![
///         |n: &_| { if n % 3 == 0 { Some(Fromu32::Fizz) } else { None }},
///         |n: &_| { if n % 5 == 0 { Some(Fromu32::Buzz) } else { None }},
///      ])
///     .build();
///
/// for s in fb {
///     println!("{}", s.join(""));
/// }
/// ```
#[macro_export]
macro_rules! rules {
    ($($fn:expr),* $(,)?) => {
        vec![$(Rule::from($fn)),*]
    };
}
