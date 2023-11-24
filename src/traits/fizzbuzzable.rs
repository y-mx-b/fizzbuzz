use super::FizzBuzzed;

pub trait FizzBuzzable<O: FizzBuzzed<Self>>: Clone + PartialEq + PartialOrd {
    fn succ(&self) -> Self;
    fn pred(&self) -> Self;
}

macro_rules! impl_fizzbuzzable {
    ($type:ty) => {
        impl<O: FizzBuzzed<Self>> FizzBuzzable<O> for $type {
            fn succ(&self) -> Self {
                self + 1
            }

            fn pred(&self) -> Self {
                self - 1
            }
        }
    }
}

impl_fizzbuzzable!(i8);
impl_fizzbuzzable!(i16);
impl_fizzbuzzable!(i32);
impl_fizzbuzzable!(i64);
impl_fizzbuzzable!(i128);
impl_fizzbuzzable!(u8);
impl_fizzbuzzable!(u16);
impl_fizzbuzzable!(u32);
impl_fizzbuzzable!(u64);
impl_fizzbuzzable!(u128);