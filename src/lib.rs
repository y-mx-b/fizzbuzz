use std::fmt;

pub trait FizzBuzzed: fmt::Display {
    fn from(n: i64) -> Self;
}

pub fn fizzbuzz<O: FizzBuzzed>(n: i64) -> O {
    O::from(n)
}
