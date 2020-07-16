#![allow(dead_code)]

struct Solution;

impl Solution {
    /// Write a function that reverses a string. The input string is given as an array of characters char[].
    ///
    /// Do not allocate extra space for another array, you must do this by modifying the input array in-place
    /// with O(1) extra memory.
    ///
    /// You may assume all the characters consist of printable ascii characters.
    pub fn reverse_string(s: &mut Vec<char>) {
        if s.is_empty() { return; }

        let end = s.len()-1;
        for i in 0..=(end / 2) {
            s.swap(i, end - i);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    pub fn test_solution(input: Vec<char>, expected: Vec<char>) {
        let mut input = input;
        Solution::reverse_string(&mut input);

        assert_eq!(input, expected);
    }

    /// Input:  ["h","e","l","l","o"]
    /// Output: ["o","l","l","e","h"]
    #[test]
    pub fn example_1() {
        let input = vec!['h','e','l','l','o'];
        let expected = vec!['o','l','l','e','h'];
        test_solution(input, expected);
    }

    /// Input: ["H","a","n","n","a","h"]
    /// Output: ["h","a","n","n","a","H"]
    #[test]
    pub fn example_2() {
        let input = vec!['H','a','n','n','a','h'];
        let expected = vec!['h','a','n','n','a','H'];
        test_solution(input, expected);
    }

    #[test]
    pub fn example_3() {
        let input = vec![
            'A',' ','m','a','n',',',' ','a',' ','p','l','a','n',',',' ',
            'a',' ','c','a','n','a','l',':',' ','P','a','n','a','m','a'
        ];

        let expected = vec![
            'a','m','a','n','a','P',' ',':','l','a','n','a','c',' ','a',
            ' ',',','n','a','l','p',' ','a',' ',',','n','a','m',' ','A'
        ];

        test_solution(input, expected);
    }

    #[test]
    pub fn even_string_non_symmetric_inner_chars() {
        let input = vec!['a','b','c','d'];
        let expected = vec!['d','c','b','a'];
        test_solution(input, expected);
    }

    #[test]
    pub fn empty() {
        test_solution(Vec::new(), Vec::new());
    }

    #[test]
    pub fn one() {
        test_solution(vec!['a'], vec!['a']);
    }
}
