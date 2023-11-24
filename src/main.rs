use fizzbuzz::FizzBuzzBuilder;
use fizzbuzz::traits::*;

fn main() {
    let fb = FizzBuzzBuilder::new().build();

    for s in fb.iter_str() {
        println!("{}", s);
    }
}
