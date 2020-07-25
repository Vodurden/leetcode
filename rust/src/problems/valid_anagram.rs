#![allow(dead_code)]

struct Solution;

impl Solution {
    /// Given two strings s and t , write a function to determine if t is an anagram of s.
    ///
    /// **Note:** You may assume the string contains only lowercase alphabets.
    ///
    /// **Follow up:** What if the inputs contain unicode characters? How would you adapt your solution to such case?
    pub fn is_anagram(s: String, t: String) -> bool {
        // Two strings are an anagram if they contain the same counts for all letters.
        let mut s_char_counts = [0; 26];
        for c in s.chars() {
            s_char_counts[c as usize - 'a' as usize] += 1;
        }

        let mut t_char_counts = [0; 26];
        for c in t.chars() {
            t_char_counts[c as usize - 'a' as usize] += 1;
        }

        return s_char_counts == t_char_counts;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_solution(s: &str, t: &str, expected: bool) {
        assert_eq!(
            Solution::is_anagram(s.to_string(), t.to_string()),
            expected
        );
    }

    /// Input: s = "anagram", t = "nagaram"
    /// Output: true
    #[test]
    pub fn example_1() {
        test_solution("anagram", "nagaram", true);
    }

    /// Input: s = "rat", t = "car"
    /// Output: false
    #[test]
    pub fn example_2() {
        test_solution("rat", "car", false);
    }

    #[test]
    pub fn empty() {
        test_solution("", "", true);
    }

    #[test]
    pub fn one() {
        test_solution("a", "a", true);
    }
}
