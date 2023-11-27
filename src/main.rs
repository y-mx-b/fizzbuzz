use fizzbuzz::traits::*;
use fizzbuzz::{FizzBuzz, FizzBuzzBuilder, builder::FizzBuzzBuilderError};

fn main() -> Result<(), FizzBuzzBuilderError> {
    let fb: FizzBuzz<i8, _> = FizzBuzzBuilder::new()
                                .range(1..=100)?
                                .build();

    for s in fb.iter_str() {
        println!("{}", s);
    }

    Ok(())
}
