use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum FizzBuzzBuilderError {
    MissingMap,
    MissingRule,
    MissingRuleAndMap,
    InvalidUnboundedStart,
    InvalidUnboundedEnd,
    InvalidUnboundedBounds,
}

// TODO: write better error messages with more context
impl fmt::Display for FizzBuzzBuilderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::MissingMap => "Please provide an appropriate map.",
                Self::MissingRule => "Please provide an appropriate rule.",
                Self::MissingRuleAndMap => "Please provide an appropriate rule and map.",
                Self::InvalidUnboundedStart => "Unbounded start is invalid.",
                Self::InvalidUnboundedEnd => "Unbounded end is invalid.",
                Self::InvalidUnboundedBounds => "Unbounded bounds are invalid.",
            }
        )
    }
}

impl Error for FizzBuzzBuilderError {}
