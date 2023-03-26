fn main() {
    println!("{:?}", fizz_buzz(15));
}

fn fizz_buzz(n: i32) -> Vec<String> {
    let mut output: Vec<String> = vec![];
    for i in 1..n + 1 {
        output.push(if i % 3 == 0 && i % 5 == 0 {
            String::from("FizzBuzz")
        } else if i % 3 == 0 {
            String::from("Fizz")
        } else if i % 5 == 0 {
            String::from("Buzz")
        } else {
            i.to_string()
        });
    }
    output
}
