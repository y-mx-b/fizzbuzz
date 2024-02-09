use fizzbuzz::traits::*;
use fizzbuzz::{builder::FizzBuzzBuilderError, FizzBuzz, FizzBuzzBuilder};

fn main() -> Result<(), FizzBuzzBuilderError> {
    let fb: FizzBuzz<i8, _> = FizzBuzzBuilder::new().range(1..=100)?.build();

    for s in fb.iter_str() {
        println!("{}", s);
    }

    Ok(())
}
