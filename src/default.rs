use crate::traits::*;
use crate::{FizzBuzz, FizzBuzzBuilder, FizzBuzzed};
use std::collections::BTreeMap;
use std::fmt;

macro_rules! impl_default_output {
    ($type:ty, $name:ident) => {
        #[derive(Debug, Clone)]
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

        impl DefaultBuilder<$type, $name> for FizzBuzzBuilder<$type, $name> {
            fn default_map() -> BTreeMap<$type, $name> {
                BTreeMap::from([(3, $name::Fizz), (5, $name::Buzz)])
            }
            fn default_rule() -> Box<dyn Fn($type, $type) -> bool> {
                Box::new(|n, divis| n % divis == 0)
            }
            fn build(self) -> FizzBuzz<$type, $name> {
                FizzBuzz {
                    start: self.start.unwrap_or(1),
                    end: self.end.unwrap_or(100),
                    map: self.map.unwrap_or(Self::default_map()),
                    rule: self.rule.unwrap_or(Self::default_rule()),
                }
            }
        }
    };
}

impl_default_output!(i8, Fromi8);
impl_default_output!(i16, Fromi16);
impl_default_output!(i32, Fromi32);
impl_default_output!(i64, Fromi64);
impl_default_output!(i128, Fromi128);
impl_default_output!(u8, Fromu8);
impl_default_output!(u16, Fromu16);
impl_default_output!(u32, Fromu32);
impl_default_output!(u64, Fromu64);
impl_default_output!(u128, Fromu128);
