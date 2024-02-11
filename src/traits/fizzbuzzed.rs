use super::DomainItem;
use std::fmt::{Debug, Display};

pub trait RangeItem<T: DomainItem>: Display + Debug + Sized + Clone {
    fn from(n: T, rules: &[Box<dyn Fn(T) -> Self>]) -> Vec<Self>; // TODO: rename this function
}

pub trait JoinRangeItem<T: DomainItem, O: RangeItem<T>> {
    fn join(&self, sep: &str) -> String;
}

impl<T: DomainItem, O: RangeItem<T>> JoinRangeItem<T, O> for Vec<O> {
    fn join(&self, sep: &str) -> String {
        self.iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join(sep)
    }
}
