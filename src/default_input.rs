use crate::traits::*;
use std::ops::{Range, RangeFrom, RangeInclusive};

macro_rules! impl_fizzbuzzable {
    ($type:ty) => {
        impl<O: FizzBuzzed<$type, Self>> FizzBuzzable<$type, O> for Range<$type> {}
        impl<O: FizzBuzzed<$type, Self>> FizzBuzzable<$type, O> for RangeFrom<$type> {}
        impl<O: FizzBuzzed<$type, Self>> FizzBuzzable<$type, O> for RangeInclusive<$type> {}
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