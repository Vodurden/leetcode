#![allow(dead_code)]

struct Solution;

impl Solution {
    /// Given a string, find the first non-repeating character in it and return its index. If it doesn't exist, return -1.
    ///
    /// **Note:** You may assume the string contains only lowercase English letters.
    pub fn first_uniq_char(s: String) -> i32 {
        // We can fit lowercase english letters in a size 26 array. I expect this to be faster
        // then using a hashmap since we don't need to hash the digits.
        let mut counts: [u32; 26] = [0; 26];

        for c in s.chars() {
            counts[c as usize - 'a' as usize] += 1;
        }

        for (i, c) in s.chars().enumerate() {
            if counts[c as usize - 'a' as usize] == 1 {
                return i as i32;
            }
        }

        -1
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    fn test_solution(input: &str, expected: i32) {
        assert_eq!(Solution::first_uniq_char(input.to_string()), expected);
    }


    #[test]
    pub fn example_1() {
        test_solution("leetcode", 0);
    }

    #[test]
    pub fn example_2() {
        test_solution("loveleetcode", 2);
    }

    #[test]
    pub fn no_solution() {
        test_solution("aabbcc", -1);
    }

    #[test]
    pub fn empty() {
        test_solution("", -1);
    }
}
