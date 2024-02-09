use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum FizzBuzzBuilderError {
    InvalidUnboundedStart,
}

// TODO: write better error messages with more context
impl fmt::Display for FizzBuzzBuilderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::InvalidUnboundedStart => "Unbounded start is invalid.",
            }
        )
    }
}

impl Error for FizzBuzzBuilderError {}
