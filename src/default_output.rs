use crate::builder::BuilderState;
use crate::traits::*;
use crate::{FizzBuzzBuilder, FizzBuzzed};
use std::collections::BTreeMap;
use std::fmt;
use std::marker::PhantomData;

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

        impl FizzBuzzed<$type> for $name {
            fn from(
                n: $type,
                map: &BTreeMap<$type, Self>,
                rule: &impl Fn($type, $type) -> bool,
            ) -> Vec<Self> {
                let mut output = Vec::new();

                for &divis in map.keys() {
                    if rule(n, divis) {
                        output.push(map.get(&divis).unwrap().clone());
                    }
                }

                if output.is_empty() {
                    output.push($name::Num(n))
                }

                output
            }
        }

        impl DefaultBuilder<$type, $name>
            for FizzBuzzBuilder<$type, $name, BuilderState<false, false, false, false>>
        {
            fn default_map() -> BTreeMap<$type, $name> {
                BTreeMap::from([(3, $name::Fizz), (5, $name::Buzz)])
            }
            fn default_rule() -> Box<dyn Fn($type, $type) -> bool> {
                Box::new(|n, divis| n % divis == 0)
            }
            fn default() -> FizzBuzzBuilder<$type, $name, BuilderState<true, true, true, true>> {
                FizzBuzzBuilder {
                    _state: PhantomData,
                    start: Some(1),
                    end: Some(100),
                    map: Some(Self::default_map()),
                    rule: Some(Self::default_rule()),
                }
            }
        }
    };
}

#[cfg(feature = "default_output_i8")]
impl_default_output!(i8, Fromi8);
#[cfg(feature = "default_output_i16")]
impl_default_output!(i16, Fromi16);
#[cfg(feature = "default_output_i32")]
impl_default_output!(i32, Fromi32);
#[cfg(feature = "default_output_i64")]
impl_default_output!(i64, Fromi64);
#[cfg(feature = "default_output_i128")]
impl_default_output!(i128, Fromi128);
#[cfg(feature = "default_output_u8")]
impl_default_output!(u8, Fromu8);
#[cfg(feature = "default_output_u16")]
impl_default_output!(u16, Fromu16);
#[cfg(feature = "default_output_u32")]
impl_default_output!(u32, Fromu32);
#[cfg(feature = "default_output_u64")]
impl_default_output!(u64, Fromu64);
#[cfg(feature = "default_output_u128")]
impl_default_output!(u128, Fromu128);
