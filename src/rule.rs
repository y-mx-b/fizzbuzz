use crate::*;

pub struct Rule<DI: DomainItem, RI: RangeItem>(pub(crate) Box<dyn Fn(&DI) -> Option<RI>>);

impl<DI: DomainItem, RI: RangeItem> Rule<DI, RI> {
    pub fn call(&self, di: &DI) -> Option<RI> {
        self.0.as_ref()(di)
    }
}

impl<DI: DomainItem, RI: RangeItem, F: Fn(&DI) -> Option<RI> + 'static> From<F> for Rule<DI, RI> {
    fn from(value: F) -> Self {
        Rule(Box::new(value))
    }
}

impl<'a, DI: DomainItem, RI: RangeItem> AsRef<dyn Fn(&DI) -> Option<RI> + 'a> for Rule<DI, RI> {
    fn as_ref(&self) -> &(dyn Fn(&DI) -> Option<RI> + 'a) {
        self.0.as_ref()
    }
}

#[macro_export]
macro_rules! rule {
    ($fn:expr) => {
        $crate::rule::Rule::from($fn)
    };
}

#[macro_export]
macro_rules! rules {
    ($($fn:expr),* $(,)?) => {
        vec![$(rule!($fn)),*]
    };
}
