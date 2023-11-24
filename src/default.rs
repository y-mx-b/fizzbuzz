use std::fmt;
use crate::FizzBuzzed;
use std::collections::BTreeMap;

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
