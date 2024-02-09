use super::FizzBuzzable;
use std::collections::BTreeMap;
use std::fmt::{Debug, Display};

pub trait FizzBuzzed<I: FizzBuzzable<Self>>: Display + Debug + Sized + Clone {
    fn from(n: I, map: &BTreeMap<I, Self>, rule: &impl Fn(I, I) -> bool) -> Vec<Self>;
}

pub trait JoinFizzBuzzed<I: FizzBuzzable<O>, O: FizzBuzzed<I>> {
    fn join(&self, sep: &str) -> String;
}

impl<I: FizzBuzzable<O>, O: FizzBuzzed<I>> JoinFizzBuzzed<I, O> for Vec<O> {
    fn join(&self, sep: &str) -> String {
        self.iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join(sep)
    }
}
