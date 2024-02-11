use crate::*;
use std::fmt;
use std::ops::RangeBounds;

macro_rules! impl_fizzbuzzed {
    ($inner:ty, $name:ident) => {};
}

macro_rules! impl_default_builder {
    ($inner:ty, $name:ident) => {
        impl<R: RangeBounds<$inner> + Iterator<Item = $inner>> DefaultBuilder<$inner, R, $name>
            for FizzBuzzBuilder<$inner, R, $name, false>
        {
            fn default_rules() -> Vec<Box<dyn Fn(&$inner) -> Option<$name>>> {
                vec![
                    Box::new(|n| if n % 3 == 0 { Some($name::Fizz) } else { None }),
                    Box::new(|n| if n % 5 == 0 { Some($name::Buzz) } else { None }),
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

macro_rules! impl_default_output {
    ($type:ty, $name:ident) => {
        #[derive(Debug, Clone)]
        /// An auto-generated default output type for the matching primitive integer type.
        ///
        /// With the default rules, [Fizz](Self::Fizz) will be selected as an output if
        /// the input is a multiple of 3 while [Buzz](Self::Buzz) will be selected as
        /// an output if the input is a multiple of 5.
        pub enum $name {
            Fizz,
            Buzz,
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(
                    f,
                    "{}",
                    match self {
                        $name::Fizz => String::from("fizz"),
                        $name::Buzz => String::from("buzz"),
                    }
                )
            }
        }

        impl_fizzbuzzed!($type, $name);

        impl_default_builder!($type, $name);
    };
}

#[cfg(feature = "signed_output")]
impl_default_output!(i8, Fromi8);
#[cfg(feature = "signed_output")]
impl_default_output!(i16, Fromi16);
#[cfg(feature = "signed_output")]
impl_default_output!(i32, Fromi32);
#[cfg(feature = "signed_output")]
impl_default_output!(i64, Fromi64);
#[cfg(feature = "signed_output")]
impl_default_output!(i128, Fromi128);
#[cfg(feature = "unsigned_output")]
impl_default_output!(u8, Fromu8);
#[cfg(feature = "unsigned_output")]
impl_default_output!(u16, Fromu16);
#[cfg(feature = "unsigned_output")]
impl_default_output!(u32, Fromu32);
#[cfg(feature = "unsigned_output")]
impl_default_output!(u64, Fromu64);
#[cfg(feature = "unsigned_output")]
impl_default_output!(u128, Fromu128);
