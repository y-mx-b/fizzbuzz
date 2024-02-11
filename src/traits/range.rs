use super::DomainItem;
use std::fmt::{Debug, Display};

pub trait RangeItem<DI: DomainItem>: Display + Debug + Sized + Clone {}
impl<T: Display + Debug + Sized + Clone, DI: DomainItem> RangeItem<DI> for T {}

pub enum RangeVariant<DI: DomainItem, RI: RangeItem<DI>> {
    Some(Vec<RI>),
    None(DI),
}

impl<DI: DomainItem, RI: RangeItem<DI>> RangeVariant<DI, RI> {
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
