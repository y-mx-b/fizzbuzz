use super::FizzBuzzed;
use std::fmt::{Debug, Display};

// TODO: change FizzBuzzable to be an iterator
// use a builder to automatically generate iterators 

pub trait FizzBuzzableItem: Clone + Debug + Sized + PartialOrd + Ord + PartialEq + Eq + Display {}
impl<T: Clone + Debug + Sized + PartialOrd + Ord + PartialEq + Eq + Display> FizzBuzzableItem for T {}

pub trait FizzBuzzable<T: FizzBuzzableItem, O: FizzBuzzed<T>>: Iterator<Item = T> + Sized{}