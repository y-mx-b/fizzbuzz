use fizzbuzz::*;

fn main() -> Result<(), FizzBuzzBuilderError> {
    let fb: FizzBuzz<i8, _> = FizzBuzzBuilder::default().range(..=10)?.build();

    for s in fb.iter_str("") {
        println!("{}", s);
    }

    Ok(())
}
