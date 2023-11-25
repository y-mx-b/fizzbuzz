use crate::traits::*;

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
    };
}

#[cfg(feature = "default_input_i8")]
impl_fizzbuzzable!(i8);
#[cfg(feature = "default_input_i16")]
impl_fizzbuzzable!(i16);
#[cfg(feature = "default_input_i32")]
impl_fizzbuzzable!(i32);
#[cfg(feature = "default_input_i64")]
impl_fizzbuzzable!(i64);
#[cfg(feature = "default_input_i128")]
impl_fizzbuzzable!(i128);
#[cfg(feature = "default_input_u8")]
impl_fizzbuzzable!(u8);
#[cfg(feature = "default_input_u16")]
impl_fizzbuzzable!(u16);
#[cfg(feature = "default_input_u32")]
impl_fizzbuzzable!(u32);
#[cfg(feature = "default_input_u64")]
impl_fizzbuzzable!(u64);
#[cfg(feature = "default_input_u128")]
impl_fizzbuzzable!(u128);
