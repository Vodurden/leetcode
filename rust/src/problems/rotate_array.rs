#![allow(dead_code)]

struct Solution;

impl Solution {
    /// Given an array, rotate the array to the right by k steps, where k is non-negative.
    ///
    /// Follow up:
    ///
    /// - Try to come up as many solutions as you can, there are at least 3 different ways to solve this problem.
    /// - Could you do it in-place with O(1) extra space?
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let k = k as usize;

        let mut result: Vec<i32> = vec![0; nums.len()];
        for (i, &n) in nums.iter().enumerate() {
            let target_index = (i + k) % nums.len();
            result[target_index] = n
        }

        nums.copy_from_slice(&result[..]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    pub fn test_all_solutions(input: Vec<i32>, k: i32, expected_output: Vec<i32>) {
        let mut solution_input = input.clone();
        Solution::rotate(&mut solution_input, k);
        assert_eq!(solution_input, expected_output);
    }

    /// Input: nums = [1,2,3,4,5,6,7], k = 3
    /// Output: [5,6,7,1,2,3,4]
    /// Explanation:
    /// - rotate 1 steps to the right: [7,1,2,3,4,5,6]
    /// - rotate 2 steps to the right: [6,7,1,2,3,4,5]
    /// - rotate 3 steps to the right: [5,6,7,1,2,3,4]
    #[test]
    pub fn example_1() {
        test_all_solutions(vec![1,2,3,4,5,6,7], 3, vec![5,6,7,1,2,3,4]);
    }

    /// Input: nums = [-1,-100,3,99], k = 2
    /// Output: [3,99,-1,-100]
    /// Explanation:
    /// - rotate 1 steps to the right: [99,-1,-100,3]
    /// - rotate 2 steps to the right: [3,99,-1,-100]
    #[test]
    pub fn example_2() {
        test_all_solutions(vec![-1,-100,3,99], 2, vec![3,99,-1,-100]);
    }

    #[test]
    pub fn empty() {
        test_all_solutions(Vec::new(), 2, Vec::new());
    }

    #[test]
    pub fn one_input() {
        test_all_solutions(vec![5], 2, vec![5]);
    }

    #[test]
    pub fn no_rotation() {
        test_all_solutions(vec![1,2,3,4], 0, vec![1,2,3,4]);
    }
}
