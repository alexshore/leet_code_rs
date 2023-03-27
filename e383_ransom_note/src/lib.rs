#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        for char in ransom_note.chars() {
            if ransom_note.matches(char).count() > magazine.matches(char).count() {
                return false;
            };
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn case_1() {
        assert_eq!(Solution::can_construct("a".to_string(), "b".to_string()), false);
    }
    #[test]
    fn case_2() {
        assert_eq!(Solution::can_construct("aa".to_string(), "ab".to_string()), false);
    }
    #[test]
    fn case_3() {
        assert_eq!(Solution::can_construct("aa".to_string(), "aab".to_string()), true);
    }
}
