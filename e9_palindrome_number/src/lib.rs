#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        };

        let x = x.to_string();

        return x == x.chars().rev().collect::<String>();
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn case_1() {
        assert!(Solution::is_palindrome(121));
    }

    #[test]
    fn case_2() {
        assert!(!Solution::is_palindrome(-121));
    }

    #[test]
    fn case_3() {
        assert!(!Solution::is_palindrome(10));
    }
}
