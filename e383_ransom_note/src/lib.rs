#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn can_construct(ransom_note: String, mut magazine: String) -> bool {
        for c in ransom_note.chars() {
            if let Some(index) = magazine.find(c) {
                magazine.remove(index);
            } else {
                return false;
            }
        }
        true
    }

    pub fn can_construct_old2(ransom_note: String, magazine: String) -> bool {
        let mut dict = std::collections::HashMap::new();

        for c in magazine.chars() {
            dict.entry(c).and_modify(|count| *count += 1).or_insert(1);
        }

        for c in ransom_note.chars() {
            match dict.get_mut(&c) {
                Some(n) if *n > 0 => *n -= 1,
                _ => return false,
            }
        }
        true
    }

    pub fn can_construct_old(ransom_note: String, magazine: String) -> bool {
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
