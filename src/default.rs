use crate::traits::*;
use crate::{FizzBuzz, FizzBuzzBuilder, FizzBuzzed};
use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone)]
pub enum Output {
    Fizz,
    Buzz,
    Num(i64),
}

impl fmt::Display for Output {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Output::Fizz => String::from("fizz"),
                Output::Buzz => String::from("buzz"),
                Output::Num(n) => format!("{}", n),
            }
        )
    }
}

impl FizzBuzzed<i64> for Output {
    fn from(n: i64, map: &BTreeMap<i64, Self>, rule: &impl Fn(i64, i64) -> bool) -> Vec<Self> {
        let mut output = Vec::new();

        for &divis in map.keys() {
            if rule(n, divis) {
                output.push(map.get(&divis).unwrap().clone());
            }
        }

        if output.is_empty() {
            output.push(Output::Num(n))
        }

        output
    }
}

impl DefaultBuilder<i64, Output> for FizzBuzzBuilder<i64, Output> {
    fn default_map() -> BTreeMap<i64, Output> {
        BTreeMap::from([(3, Output::Fizz), (5, Output::Buzz)])
    }
    fn default_rule() -> Box<dyn Fn(i64, i64) -> bool> {
        Box::new(|n, divis| n % divis == 0)
    }
    fn build(self) -> FizzBuzz<i64, Output> {
        FizzBuzz {
            start: self.start.unwrap_or(1),
            end: self.end.unwrap_or(100),
            map: self.map.unwrap_or(Self::default_map()),
            rule: self.rule.unwrap_or(Self::default_rule()),
        }
    }
}
