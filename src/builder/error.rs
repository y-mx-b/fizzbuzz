use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum FizzBuzzBuilderError {
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
                Self::InvalidUnboundedStart => "Unbounded start is invalid.",
                Self::InvalidUnboundedEnd => "Unbounded end is invalid.",
                Self::InvalidUnboundedBounds => "Unbounded bounds are invalid.",
            }
        )
    }
}

impl Error for FizzBuzzBuilderError {}
