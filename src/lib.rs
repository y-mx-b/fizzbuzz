use std::fmt;

pub trait FizzBuzzed: fmt::Display + Sized + Clone {
    fn from(n: i64) -> Vec<Self>;
}

pub fn fizzbuzz<O: FizzBuzzed>(n: i64) -> Vec<O> {
    O::from(n)
}
