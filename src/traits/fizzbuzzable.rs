use super::FizzBuzzed;

pub trait FizzBuzzable<O: FizzBuzzed<Self>>: Clone + PartialEq + Eq + PartialOrd + Ord {
    fn min() -> Option<Self>;
    fn max() -> Option<Self>;
    fn succ(&self) -> Self;
    fn pred(&self) -> Self;
}
