#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn fizz_buzz(n: i32) -> Vec<String> {
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
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn case() {
        assert_eq!(
            vec!["1", "2", "Fizz", "4", "Buzz", "Fizz", "7", "8", "Fizz", "Buzz", "11", "Fizz", "13", "14", "FizzBuzz"],
            Solution::fizz_buzz(15)
        )
    }
}
