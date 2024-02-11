use crate::traits::*;
use crate::FizzBuzzBuilder;
use std::fmt;
use std::ops::RangeBounds;

macro_rules! impl_fizzbuzzed {
    ($inner:ty, $name:ident) => {
        impl RangeItem<$inner> for $name {
            fn from(n: $inner, rules: &[Box<dyn Fn($inner) -> Self>]) -> Vec<Self> {
                let mut output = Vec::new();
                for f in rules.iter() {
                    match f(n) {
                        $name::Fizz | $name::Buzz => {
                            output.push(f(n));
                        }
                        _ => {}
                    }
                }

                if output.is_empty() {
                    output.push($name::Num(n))
                }

                output
            }
        }
    };
}

macro_rules! impl_default_builder {
    ($inner:ty, $name:ident) => {
        impl<R: RangeBounds<$inner> + Iterator<Item = $inner>> DefaultBuilder<$inner, R, $name>
            for FizzBuzzBuilder<$inner, R, $name, false>
        {
            fn default_rules() -> Vec<Box<dyn Fn($inner) -> $name>> {
                vec![
                    Box::new(|n| {
                        if n % 3 == 0 {
                            $name::Fizz
                        } else {
                            $name::Num(n)
                        }
                    }),
                    Box::new(|n| {
                        if n % 5 == 0 {
                            $name::Buzz
                        } else {
                            $name::Num(n)
                        }
                    }),
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
        /// With the default mappings and rules, [Fizz](Self::Fizz) will be selected as
        /// an output if the input is a multiple of 3 while [Buzz](Self::Buzz) will be
        /// selected as an output if the input is a multiple of 5. Otherwise, [Num](Self::Num)
        /// will be returned.
        pub enum $name {
            Fizz,
            Buzz,
            Num($type),
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(
                    f,
                    "{}",
                    match self {
                        $name::Fizz => String::from("fizz"),
                        $name::Buzz => String::from("buzz"),
                        $name::Num(n) => format!("{}", n),
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
