use fizzbuzz::{fizzbuzz, FizzBuzzed};
use std::fmt;

#[derive(Debug)]
enum Output {
    Fizz,
    Buzz,
    FizzBuzz,
    Num(i64),
}

impl fmt::Display for Output {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Output::Fizz => String::from("fizz"),
            Output::Buzz => String::from("buzz"),
            Output::FizzBuzz => String::from("fizzbuzz"),
            Output::Num(n) => format!("{}", n),
        })
    }
}

impl FizzBuzzed for Output {
    fn from(n: i64) -> Self {
    if n % 3 == 0 && n % 5 == 0 {
        Self::Fizz
    } else if n % 3 == 0 {
        Self::Buzz
    } else if n % 5 == 0 {
        Self::FizzBuzz
    } else {
        Self::Num(n)
    }
    }
}

fn main() {
    for i in 1..100 {
       println!("{}", fizzbuzz::<Output>(i));
    }
}