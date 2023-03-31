#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let chars: Vec<char> = s.chars().collect();
        let mut res: i32 = 0;
        let map = |c| match c {
            'M' => 1000,
            'D' => 500,
            'C' => 100,
            'L' => 50,
            'X' => 10,
            'V' => 5,
            'I' => 1,
            _ => unreachable!(),
        };

        for i in 0..chars.len() - 1 {
            let cur = map(chars[i]);
            let nex = map(chars[i + 1]);
            if cur < nex {
                res -= cur;
            } else {
                res += cur;
            }
        }
        res += map(*chars.last().unwrap());
        res
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn case_1() {
        assert_eq!(Solution::roman_to_int(String::from("III")), 3);
    }

    #[test]
    fn case_2() {
        assert_eq!(Solution::roman_to_int(String::from("LVIII")), 58);
    }

    #[test]
    fn case_3() {
        assert_eq!(Solution::roman_to_int(String::from("MCMXCIV")), 1994);
    }
}
