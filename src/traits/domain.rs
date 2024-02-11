use super::RangeItem;
use std::fmt::{Debug, Display};

// TODO: change FizzBuzzable to be an iterator
// use a builder to automatically generate iterators

pub trait DomainItem: Clone + Debug + Sized + PartialOrd + Ord + PartialEq + Eq + Display {}
impl<T: Clone + Debug + Sized + PartialOrd + Ord + PartialEq + Eq + Display> DomainItem for T {}

pub trait Domain<DI: DomainItem, RI: RangeItem>: Iterator<Item = DI> + Sized {}
