#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack: Vec<char> = vec![];
        for char in s.chars() {
            match char {
                ')' | '}' | ']' if stack.is_empty() => return false,
                ')' if *stack.last().unwrap() == '(' => stack.truncate(stack.len() - 1),
                '}' if *stack.last().unwrap() == '{' => stack.truncate(stack.len() - 1),
                ']' if *stack.last().unwrap() == '[' => stack.truncate(stack.len() - 1),
                ')' | '}' | ']' => return false,
                '(' | '{' | '[' => stack.push(char),
                _ => unreachable!(),
            };
        }

        stack.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn case_1() {
        assert!(Solution::is_valid(String::from("()[]{}")))
    }

    #[test]
    fn case_2() {
        assert!(Solution::is_valid(String::from("()")))
    }

    #[test]
    fn case_3() {
        assert!(!Solution::is_valid(String::from("{[]}")))
    }
}
