use fizzbuzz::*;

fn f(_n: &i8) -> Option<default_output::Fromi8> {
    Some(default_output::Fromi8::Buzz)
}

fn main() {
    let fb: FizzBuzz<i8, _, _> = FizzBuzzBuilder::default()
                                    .domain(1..=100)
                                    .rule(|_n: &i8| { Some(default_output::Fromi8::Fizz) })
                                    .rule(f)
                                    .build();

    for s in fb {
        println!("{}", s.join(""));
    }
}
