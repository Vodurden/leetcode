#![allow(dead_code)]

struct Solution;

impl Solution {
    /// Given a non-empty array of digits representing a non-negative integer, increment one to the integer.
    ///
    /// The digits are stored such that the most significant digit is at the head of the list, and each element in the array contains a single digit.
    ///
    /// You may assume the integer does not contain any leading zero, except the number 0 itself.
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut result = digits;
        for n in result.iter_mut().rev() {
            if *n < 9 {
                *n += 1;
                break;
            } else {
                *n = 0;
            }
        }

        if !result.is_empty() && result.iter().all(|&n| n == 0) {
            result.insert(0, 1);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    pub fn test_all_solutions(input: Vec<i32>, expected_output: Vec<i32>) {
        let result = Solution::plus_one(input);
        assert_eq!(result, expected_output);
    }

    /// Input: [1,2,3]
    /// Output: [1,2,4]
    /// Explanation: The array represents the integer 123.
    #[test]
    pub fn example_1() {
        test_all_solutions(vec![1,2,3], vec![1,2,4]);
    }

    /// Input: [4,3,2,1]
    /// Output: [4,3,2,2]
    /// Explanation: The array represents the integer 4321.
    #[test]
    pub fn example_2() {
        test_all_solutions(vec![4,3,2,1], vec![4,3,2,2]);
    }

    #[test]
    pub fn empty() {
        test_all_solutions(Vec::new(), Vec::new());
    }

    #[test]
    pub fn one_input() {
        test_all_solutions(vec![5], vec![6]);
    }

    #[test]
    pub fn zero_input() {
        test_all_solutions(vec![0], vec![1]);
    }

    #[test]
    pub fn one_input_with_digit_extension() {
        test_all_solutions(vec![9], vec![1,0]);
    }

    #[test]
    pub fn multiple_values_with_digit_extension() {
        test_all_solutions(vec![9,9,9], vec![1,0,0,0]);
    }
}
