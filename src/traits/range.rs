use super::DomainItem;
use std::fmt::{Debug, Display};

pub trait RangeItem: Display + Debug + Sized + Clone {}
impl<T: Display + Debug + Sized + Clone> RangeItem for T {}

pub enum RangeVariant<DI: DomainItem, RI: RangeItem> {
    Some(Vec<RI>),
    None(DI),
}

impl<DI: DomainItem, RI: RangeItem> RangeVariant<DI, RI> {
    pub fn from(di: DI, rules: &[Box<dyn Fn(&DI) -> Option<RI>>]) -> Self {
        let mut s = Vec::new();
        for f in rules {
            match f(&di) {
                Some(ri) => {
                    s.push(ri);
                }
                _ => {}
            }
        }

        if s.is_empty() {
            RangeVariant::None(di)
        } else {
            RangeVariant::Some(s)
        }
    }

    pub fn join(&self, sep: &str) -> String {
        match self {
            RangeVariant::Some(v) => v.iter().map(|ri| ri.to_string()).collect::<Vec<String>>().join(sep),
            RangeVariant::None(di) => di.to_string(),
        }
    }
}