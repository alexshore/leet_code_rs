#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn number_of_steps(mut num: i32) -> i32 {
        let mut steps: i32 = 0;
        while num > 0 {
            if num % 2 == 0 {
                num /= 2;
            } else {
                num -= 1;
            }
            steps += 1;
        }
        steps
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn case_1() {
        assert_eq!(6, Solution::number_of_steps(14));
    }
    #[test]
    fn case_2() {
        assert_eq!(4, Solution::number_of_steps(8));
    }
    #[test]
    fn case_3() {
        assert_eq!(12, Solution::number_of_steps(123));
    }
}
