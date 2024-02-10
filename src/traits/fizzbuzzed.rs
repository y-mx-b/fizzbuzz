use super::FizzBuzzable;
use std::collections::BTreeMap;
use std::fmt::{Debug, Display};

pub trait FizzBuzzed<T, I: FizzBuzzable<Self, T>>: Display + Debug + Sized + Clone {
    fn from(n: I, map: &BTreeMap<I, Self>, rule: &impl Fn(I, I) -> bool) -> Vec<Self>; // TODO: rename this function
}

pub trait JoinFizzBuzzed<T, I: FizzBuzzable<T, O>, O: FizzBuzzed<T, I>> {
    fn join(&self, sep: &str) -> String;
}

impl<T, I: FizzBuzzable<T, O>, O: FizzBuzzed<T, I>> JoinFizzBuzzed<T, I, O> for Vec<O> {
    fn join(&self, sep: &str) -> String {
        self.iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join(sep)
    }
}
