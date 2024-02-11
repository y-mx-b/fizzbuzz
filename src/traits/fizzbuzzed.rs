use super::{FizzBuzzable, FizzBuzzableItem};
use std::collections::BTreeMap;
use std::fmt::{Debug, Display};

pub trait FizzBuzzed<T: FizzBuzzableItem, I: FizzBuzzable<T, Self>>: Display + Debug + Sized + Clone {
    fn from(n: T, map: &BTreeMap<T, Self>, rule: &impl Fn(T, T) -> bool) -> Vec<Self>; // TODO: rename this function
}

pub trait JoinFizzBuzzed<T: FizzBuzzableItem, I: FizzBuzzable<T, O>, O: FizzBuzzed<T, I>> {
    fn join(&self, sep: &str) -> String;
}

impl<T: FizzBuzzableItem, I: FizzBuzzable<T, O>, O: FizzBuzzed<T, I>> JoinFizzBuzzed<T, I, O> for Vec<O> {
    fn join(&self, sep: &str) -> String {
        self.iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join(sep)
    }
}
