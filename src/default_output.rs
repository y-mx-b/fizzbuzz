//! This module defines a default output type for [FizzBuzz].

use crate::*;
use std::fmt;
use std::ops::RangeBounds;

macro_rules! impl_default_builder {
    ($inner:ty, $name:ident) => {
        impl<R: RangeBounds<$inner> + Iterator<Item = $inner>> DefaultBuilder<$inner, R, $name>
            for FizzBuzzBuilder<$inner, R, $name, false>
        {
            fn default_rules() -> Vec<Rule<$inner, $name>> {
                rules![
                    |n: &$inner| if n % 3 == 0 { Some($name::Fizz) } else { None },
                    |n: &$inner| if n % 5 == 0 { Some($name::Buzz) } else { None },
                ]
            }
            fn default() -> FizzBuzzBuilder<$inner, R, $name, false> {
                FizzBuzzBuilder {
                    domain: None,
                    rules: Self::default_rules(),
                }
            }
        }
    };
}

#[derive(Debug, Clone)]
/// A default output type.
///
/// With the default rules, [Fizz](Self::Fizz) will be selected as an output if
/// the input is a multiple of 3 while [Buzz](Self::Buzz) will be selected as
/// an output if the input is a multiple of 5.
pub enum DefaultOutput {
    Fizz,
    Buzz,
}

impl fmt::Display for DefaultOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                DefaultOutput::Fizz => String::from("fizz"),
                DefaultOutput::Buzz => String::from("buzz"),
            }
        )
    }
}

#[cfg(feature = "default_signed")]
impl_default_builder!(i8, DefaultOutput);
#[cfg(feature = "default_signed")]
impl_default_builder!(i16, DefaultOutput);
#[cfg(feature = "default_signed")]
impl_default_builder!(i32, DefaultOutput);
#[cfg(feature = "default_signed")]
impl_default_builder!(i64, DefaultOutput);
#[cfg(feature = "default_signed")]
impl_default_builder!(i128, DefaultOutput);
#[cfg(feature = "default_unsigned")]
impl_default_builder!(u8, DefaultOutput);
#[cfg(feature = "default_unsigned")]
impl_default_builder!(u16, DefaultOutput);
#[cfg(feature = "default_unsigned")]
impl_default_builder!(u32, DefaultOutput);
#[cfg(feature = "default_unsigned")]
impl_default_builder!(u64, DefaultOutput);
#[cfg(feature = "default_unsigned")]
impl_default_builder!(u128, DefaultOutput);
