//! This module contains traits related to ranges.
//!
//! # Why "Range?"
//!
//! The term "range" is used in the mathematical sense throughout this crate.
//! Given that FizzBuzz is really about mapping one set of input values to another
//! set of sets of output values, the terms "domain" and "range" are used in the
//! functional, mathematical sense, though it technically doesn't *really* match.
//!
//! Regardless, a "range" here refers to the output set.

use crate::*;
use std::fmt::{Debug, Display};

/// An item within a [RangeVariant].
pub trait RangeItem: Display + Debug + Sized + Clone {}
impl<T: Display + Debug + Sized + Clone> RangeItem for T {}

/// An enum containing either a [RangeItem] or a [DomainItem] depending
/// on whether at least one rule applies or no rules apply.
#[derive(Debug, Clone)]
pub enum RangeVariant<DI: DomainItem, RI: RangeItem> {
    /// At least one rule applies.
    Some(Vec<RI>),
    /// No rules apply.
    None(DI),
}

impl<DI: DomainItem, RI: RangeItem> RangeVariant<DI, RI> {
    /// Create a [RangeVariant] given a [DomainItem] and a set of rules.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use fizzbuzz::*;
    /// let rv = RangeVariant::from(10, &rules![|n: &u32| if *n == 10 { Some(true) } else { Some(false) }]);
    /// assert_eq!(rv, RangeVariant::Some(vec![true]));
    /// ```
    pub fn from(di: DI, rules: &[Rule<DI, RI>]) -> Self {
        let mut s = Vec::new();
        for f in rules {
            match f.call(&di) {
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

    /// Create a [String] representation of the contained [RangeItem] or [DomainItem].
    pub fn join(&self, sep: &str) -> String {
        match self {
            RangeVariant::Some(v) => v
                .iter()
                .map(|ri| ri.to_string())
                .collect::<Vec<String>>()
                .join(sep),
            RangeVariant::None(di) => di.to_string(),
        }
    }
}

impl<DI: DomainItem, RI: RangeItem + PartialEq> PartialEq for RangeVariant<DI, RI> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (RangeVariant::Some(v1), RangeVariant::Some(v2)) => v1 == v2,
            (RangeVariant::None(di1), RangeVariant::None(di2)) => di1 == di2,
            _ => false,
        }
    }
}

impl<DI: DomainItem, RI: RangeItem + Eq> Eq for RangeVariant<DI, RI> {}
