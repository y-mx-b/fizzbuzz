use super::FizzBuzzableItem;
use std::collections::BTreeMap;
use std::fmt::{Debug, Display};

pub trait FizzBuzzed<T: FizzBuzzableItem>: Display + Debug + Sized + Clone {
    fn from(n: T, map: &BTreeMap<T, Self>, rule: &impl Fn(T, T) -> bool) -> Vec<Self>; // TODO: rename this function
}

pub trait JoinFizzBuzzed<T: FizzBuzzableItem, O: FizzBuzzed<T>> {
    fn join(&self, sep: &str) -> String;
}

impl<T: FizzBuzzableItem, O: FizzBuzzed<T>> JoinFizzBuzzed<T, O> for Vec<O> {
    fn join(&self, sep: &str) -> String {
        self.iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join(sep)
    }
}
