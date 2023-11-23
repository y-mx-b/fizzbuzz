use fizzbuzz::{fizzbuzz, FizzBuzzed};
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
    fn from(n: i64) -> Vec<Self> {
        let divisors = [3, 5];
        let rule = |n : i64, divis: i64| n % divis == 0;
        let map = BTreeMap::from([(3, Output::Fizz), (5, Output::Buzz)]);
        let mut output = Vec::new();

        for &divis in divisors.iter() {
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
    for i in 1..100 {
       println!("{}", fizzbuzz::<Output>(i).iter().map(|o| o.to_string()).collect::<Vec<String>>().join(""));
    }
}