use super::FizzBuzzed;
use std::fmt::{Debug, Display};

// TODO: change FizzBuzzable to be an iterator
// use a builder to automatically generate iterators 

pub trait FizzBuzzable<O: FizzBuzzed<Self>>:
    Clone + PartialEq + Eq + PartialOrd + Ord + Display + Debug + Sized
{
    fn min() -> Option<Self>;
    fn max() -> Option<Self>;
    fn succ(&self) -> Self;
    fn pred(&self) -> Self;
}
