use super::FizzBuzzable;
use std::collections::BTreeMap;
use std::fmt::{Display, Debug};

pub trait FizzBuzzed<I: FizzBuzzable<Self>>: Display + Debug + Sized + Clone {
    fn from(n: I, map: &BTreeMap<I, Self>, rule: &impl Fn(I, I) -> bool) -> Vec<Self>;
}
