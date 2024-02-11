use crate::traits::*;
use std::ops::RangeBounds;

macro_rules! impl_fizzbuzzable {
    ($type:ty) => {
        impl<O: RangeItem<$type>, R: RangeBounds<$type> + Iterator<Item = $type>> Domain<$type, O>
            for R
        {
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
