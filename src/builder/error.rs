use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum FizzBuzzBuilderError {
    MissingMap,
    MissingRule,
    MissingRuleAndMap,
    // InvalidStart,
    // InvalidEnd,
    // InvalidBounds
}

impl fmt::Display for FizzBuzzBuilderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::MissingMap => "Please provide an appropriate map.",
                Self::MissingRule => "Please provide an appropriate rule.",
                Self::MissingRuleAndMap => "Please provide an appropriate rule and map.",
            }
        )
    }
}

impl Error for FizzBuzzBuilderError {}
