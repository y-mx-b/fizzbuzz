use crate::traits::*;

macro_rules! impl_fizzbuzzable {
    ($type:ty) => {
        impl<O: FizzBuzzed<Self>> FizzBuzzable<O> for $type {
            fn min() -> Option<Self> {
                Some(<$type>::MIN)
            }

            fn max() -> Option<Self> {
                Some(<$type>::MAX)
            }

            fn succ(&self) -> Self {
                self + 1
            }

            fn pred(&self) -> Self {
                self - 1
            }
        }
    };
}

#[cfg(feature = "signed_input")]
impl_fizzbuzzable!(i8);
#[cfg(feature = "signed_input")]
impl_fizzbuzzable!(i16);
#[cfg(feature = "signed_input")]
impl_fizzbuzzable!(i32);
#[cfg(feature = "signed_input")]
impl_fizzbuzzable!(i64);
#[cfg(feature = "signed_input")]
impl_fizzbuzzable!(i128);
#[cfg(feature = "unsigned_input")]
impl_fizzbuzzable!(u8);
#[cfg(feature = "unsigned_input")]
impl_fizzbuzzable!(u16);
#[cfg(feature = "unsigned_input")]
impl_fizzbuzzable!(u32);
#[cfg(feature = "unsigned_input")]
impl_fizzbuzzable!(u64);
#[cfg(feature = "unsigned_input")]
impl_fizzbuzzable!(u128);