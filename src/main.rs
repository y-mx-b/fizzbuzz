fn fizzbuzz(n: i64) -> String {
    if n % 3 == 0 && n % 5 == 0 {
        String::from("fizzbuzz")
    } else if n % 3 == 0 {
        String::from("fizz")
    } else if n % 5 == 0 {
        String::from("buzz")
    } else {
        format!("{}", n)
    }
}

fn main() {
    for i in 1..100 {
       println!("{}", fizzbuzz(i));
    }
}