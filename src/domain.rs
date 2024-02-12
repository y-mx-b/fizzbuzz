//! This module contains traits related to domains.
//!
//! # Why "Domain?"
//!
//! The term "domain" is used in the mathematical sense throughout this crate.
//! Given that FizzBuzz is really about mapping one set of input values to another
//! set of sets of output values, the terms "domain" and "range" are used in the
//! functional, mathematical sense, though it technically doesn't *really* match.
//!
//! Regardless, a "domain" here refers to the input set.

use std::fmt::Debug;

/// An item within a [Domain].
///
/// A blanket implementation is provided.
pub trait DomainItem: Clone + Debug + Sized + PartialOrd + Ord + PartialEq + Eq {}
impl<T: Clone + Debug + Sized + PartialOrd + Ord + PartialEq + Eq> DomainItem for T {}

/// An iterator over [DomainItem].
///
/// A blanket implementation is provided.
pub trait Domain<DI: DomainItem>: Iterator<Item = DI> + Sized {}
impl<DI: DomainItem, I: Iterator<Item = DI>> Domain<DI> for I {}
