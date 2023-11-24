use super::FizzBuzzable;
use std::collections::BTreeMap;
use std::fmt;

pub trait FizzBuzzed<I: FizzBuzzable<Self>>: fmt::Display + Sized + Clone {
    fn from(n: I, map: &BTreeMap<I, Self>, rule: &impl Fn(I, I) -> bool) -> Vec<Self>;
}
