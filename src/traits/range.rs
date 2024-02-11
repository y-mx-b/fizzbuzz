use super::DomainItem;
use std::fmt::{Debug, Display};

pub trait RangeItem<DI: DomainItem>: Display + Debug + Sized + Clone {
    fn from(n: DI, rules: &[Box<dyn Fn(DI) -> Option<Self>>]) -> Vec<Self>; // TODO: rename this function
}

pub trait JoinRangeItem<DI: DomainItem, RI: RangeItem<DI>> {
    fn join(&self, sep: &str) -> String;
}

impl<DI: DomainItem, RI: RangeItem<DI>> JoinRangeItem<DI, RI> for Vec<RI> {
    fn join(&self, sep: &str) -> String {
        self.iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join(sep)
    }
}
