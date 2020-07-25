#![allow(dead_code)]

struct Solution;

impl Solution {
    /// Given a string, determine if it is a palindrome, considering only alphanumeric characters and ignoring cases.
    ///
    /// **Note:** For the purpose of this problem, we define empty string as valid palindrome.
    ///
    /// **Constraints:** `s` consists only of printable ASCII characters.
    pub fn is_palindrome(s: String) -> bool {
        let s: String = s
            .to_ascii_lowercase()
            .chars()
            .filter(|a| a.is_alphanumeric())
            .collect();

        let reversed: String = s
            .chars()
            .rev()
            .collect();

        s == reversed
    }

    /// Same as `is_palindrome` but implemented without using `rev()`
    pub fn is_palindrome_no_reverse(s: String) -> bool {
        let chars: Vec<char> = s
            .to_ascii_lowercase()
            .chars()
            .filter(|a| a.is_alphanumeric())
            .collect();

        let len = chars.len();
        (0..len/2).all(|i| chars[i] == chars[len-i-1])
    }

    /// Same as `is_palindrome` but implemented with a stack
    pub fn is_palindrome_with_stack(s: String) -> bool {
        let chars: Vec<char> = s
            .to_ascii_lowercase()
            .chars()
            .filter(|a| a.is_alphanumeric())
            .collect();

        let mut stack: Vec<char> = chars.clone();

        for &c in chars.iter() {
            let pair_match = stack
                .pop()
                .map(|rc| c == rc)
                .unwrap_or(false);

            if !pair_match {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_all_solutions(s: &str, expected: bool) {
        assert_eq!(Solution::is_palindrome(s.to_string()), expected);
        assert_eq!(Solution::is_palindrome_no_reverse(s.to_string()), expected);
        assert_eq!(Solution::is_palindrome_with_stack(s.to_string()), expected);
    }

    // Input: "A man, a plan, a canal: Panama"
    // Output: true
    #[test]
    pub fn example_1() {
        test_all_solutions("A man, a plan, a canal: Panama", true);
    }

    /// Input: "race a car"
    /// Output: false
    #[test]
    pub fn example_2() {
        test_all_solutions("race a car", false);
    }

    #[test]
    pub fn racecar() {
        test_all_solutions("racecar", true);
    }

    #[test]
    pub fn empty() {
        test_all_solutions("", true);
    }

    #[test]
    pub fn one() {
        test_all_solutions("a", true);
    }
}
