#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut strs = strs;
        strs.sort_unstable_by_key(|k| k.len());

        let shortest = strs[0].clone();
        let mut res = String::new();
        let mut new_res = String::new();

        for c in shortest.chars() {
            new_res.push(c);
            for str in strs.iter() {
                if !str.starts_with(&new_res) {
                    return res;
                }
            }
            res = new_res.clone();
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn case_1() {
        assert_eq!(
            Solution::longest_common_prefix(vec!["flower".to_string(), "flow".to_string(), "flight".to_string()]),
            "fl".to_string()
        )
    }

    #[test]
    fn case_2() {
        assert_eq!(
            Solution::longest_common_prefix(vec!["dog".to_string(), "racecar".to_string(), "car".to_string()]),
            "".to_string()
        )
    }
}
