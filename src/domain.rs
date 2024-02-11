use std::fmt::{Debug, Display};

pub trait DomainItem: Clone + Debug + Sized + PartialOrd + Ord + PartialEq + Eq + Display {}
impl<T: Clone + Debug + Sized + PartialOrd + Ord + PartialEq + Eq + Display> DomainItem for T {}

pub trait Domain<DI: DomainItem>: Iterator<Item = DI> + Sized {}
impl<DI: DomainItem, I: Iterator<Item = DI>> Domain<DI> for I {}