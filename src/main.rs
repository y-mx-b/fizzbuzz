use fizzbuzz::*;

fn main() -> Result<(), FizzBuzzBuilderError> {
    let fb: FizzBuzz<i8, _, _> = FizzBuzzBuilder::default().domain(1..=100).build();

    for s in fb {
        println!("{}", s.join(""));
    }

    Ok(())
}
