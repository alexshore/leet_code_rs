#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn running_sum(mut nums: Vec<i32>) -> Vec<i32> {
        for i in 1..nums.len() {
            nums[i] += nums[i - 1];
        }
        nums
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn case_1() {
        assert_eq!(vec![1, 3, 6, 10], Solution::running_sum(vec![1, 2, 3, 4]))
    }
    #[test]
    fn case_2() {
        assert_eq!(vec![1, 2, 3, 4, 5], Solution::running_sum(vec![1, 1, 1, 1, 1]))
    }
    #[test]
    fn case_3() {
        assert_eq!(vec![3, 4, 6, 16, 17], Solution::running_sum(vec![3, 1, 2, 10, 1]))
    }
}
