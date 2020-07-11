#![allow(dead_code)]

struct Solution;

impl Solution {
    /// Given an array of integers, find if the array contains any duplicates.
    ///
    /// Your function should return true if any value appears at least twice in the
    /// array, and it should return false if every element is distinct.
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut sorted_nums = nums;
        sorted_nums.sort();

        sorted_nums
            .windows(2)
            .any(|window| {
                match window {
                    [a, b] => a == b,
                    _ => false
                }
            })
    }

    pub fn contains_duplicate_no_sort(nums: Vec<i32>) -> bool {
        let mut set = std::collections::HashSet::new();
        for num in nums {
            // If we can't insert a value then it was already present
            if !set.insert(num) {
                return true
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    pub fn test_all_solutions(input: Vec<i32>, expected_output: bool) {
        assert_eq!(Solution::contains_duplicate(input.clone()), expected_output);
        assert_eq!(Solution::contains_duplicate_no_sort(input.clone()), expected_output);
    }

    /// Input: [1,2,3,1]
    /// Output: true
    #[test]
    pub fn example_1() {
        test_all_solutions(vec![1,2,3,1], true);
    }

    /// Input: [1,2,3,4]
    /// Output: false
    #[test]
    pub fn example_2() {
        test_all_solutions(vec![1,2,3,4], false);
    }

    /// Input: [1,1,1,3,3,4,3,2,4,2]
    /// Output: true
    #[test]
    pub fn example_3() {
        test_all_solutions(vec![1,1,1,3,3,4,3,2,4,2], true);
    }

    #[test]
    pub fn empty() {
        test_all_solutions(Vec::new(), false);
    }

    #[test]
    pub fn one_input() {
        test_all_solutions(vec![5], false);
    }
}
