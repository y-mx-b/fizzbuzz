use fizzbuzz::traits::*;
use fizzbuzz::{FizzBuzzBuilder, FizzBuzz};

fn main() {
    let fb: FizzBuzz<i32, _> = FizzBuzzBuilder::new().build();

    for s in fb.iter_str() {
        println!("{}", s);
    }
}
