use fizzbuzz::*;

fn main() {
    let fb: FizzBuzz<i8, _, _> = FizzBuzzBuilder::default()
        .domain(1..=100)
        // .rule(|_n: &i8| { Some(default_output::Fromi8::Fizz) })
        // .rule(f)
        .rules(rules![
            |n: &_| { if n % 3 == 0 { Some(default_output::Fromi8::Fizz) } else { None }},
            |n: &_| { if n % 5 == 0 { Some(default_output::Fromi8::Buzz) } else { None }},
         ])
        .build();

    for s in fb {
        println!("{}", s.join(""));
    }
}
