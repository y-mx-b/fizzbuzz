use fizzbuzz::{FizzBuzz, FizzBuzzed};
use std::fmt;
use std::collections::BTreeMap;

#[derive(Debug, Clone)]
enum Output {
    Fizz,
    Buzz,
    Num(i64),
}

impl fmt::Display for Output {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Output::Fizz => String::from("fizz"),
            Output::Buzz => String::from("buzz"),
            Output::Num(n) => format!("{}", n),
        })
    }
}

impl FizzBuzzed for Output {
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

fn main() {
    let rule = |n : i64, divis: i64| n % divis == 0;
    let map = BTreeMap::from([(3, Output::Fizz), (5, Output::Buzz)]);

    let fb = FizzBuzz {
        start: 1,
        end: 100,
        map: map,
        rule: rule,
    };

    for v in fb.iter() {
       println!("{}", v.iter().map(|o| o.to_string()).collect::<Vec<String>>().join(""));
    }
}