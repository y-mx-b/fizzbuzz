use super::FizzBuzzed;

pub trait FizzBuzzable<O: FizzBuzzed<Self>>: Clone + PartialEq + Eq + PartialOrd + Ord {
    fn succ(&self) -> Self;
    fn pred(&self) -> Self;
}
