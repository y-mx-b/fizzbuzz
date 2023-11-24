use fizzbuzz::traits::*;
use fizzbuzz::FizzBuzzBuilder;

fn main() {
    let fb = FizzBuzzBuilder::new().build();

    for s in fb.iter_str() {
        println!("{}", s);
    }
}
