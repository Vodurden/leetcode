#![allow(dead_code)]

struct Solution;

impl Solution {
    /// Write a function to find the longest common prefix string amongst an array of strings.
    ///
    /// If there is no common prefix, return an empty string "".
    ///
    /// Note: All given inputs are in lowercase letters a-z.
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let strs: Vec<Vec<char>> = strs.iter().map(|s| s.chars().collect()).collect();
        let first_str = match strs.first() {
            Some(s) => s,
            None => return String::new(),
        };
        let strs = &strs[1..];

        let mut common_index = 0;
        'outer: while common_index < first_str.len() {
            for str in strs {
                if first_str.get(common_index) != str.get(common_index) {
                    break 'outer;
                }
            }

            common_index += 1;
        }

        first_str[0..common_index].into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_solution(input: Vec<&str>, expected: &str) {
        let input = input.into_iter().map(|s| s.to_string()).collect();
        assert_eq!(
            Solution::longest_common_prefix(input),
            expected.to_string()
        );
    }

    #[test]
    pub fn example_1() {
        test_solution(
            vec!["flower", "flow", "flight"],
            "fl"
        );
    }

    #[test]
    pub fn example_2() {
        test_solution(
            vec!["dog", "racecar", "car"],
            ""
        );
    }

    #[test]
    pub fn empty() {
        test_solution(Vec::new(), "");
    }

    #[test]
    pub fn empty_inner() {
        test_solution(vec![""], "");
    }
}
