#![allow(dead_code)]

struct Solution;

impl Solution {
    /// https://leetcode.com/explore/featured/card/top-interview-questions-easy/92/array/727/
    ///
    /// Given a sorted array nums, remove the duplicates in-place such that each element appear only once and return the new length.
    ///
    /// Do not allocate extra space for another array, you must do this by modifying the input array in-place with O(1) extra memory.
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.len() == 0 { return 0 }

        let mut result_length = 1;
        for index in 0..nums.len() {
            if nums[index] > nums[result_length - 1] {
                nums[result_length] = nums[index];
                result_length += 1;
            }
        }

        result_length as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    pub fn testcase(input: Vec<i32>, expected_result: Vec<i32>) {
        let mut input: Vec<i32> = input;
        let result_length = Solution::remove_duplicates(&mut input);

        let result_length = result_length as usize;

        let result: Vec<i32> = input
            .clone()
            .into_iter()
            .take(result_length)
            .collect();

        let expected_length = expected_result.len();
        assert_eq!(result_length, expected_length);
        assert_eq!(result, expected_result);
    }

    /// Given nums = [1,1,2]
    ///
    /// Your function should return length = 2, with the first two elements of nums being 1 and 2 respectively.
    ///
    /// It doesn't matter what you leave beyond the returned length.
    #[test]
    pub fn example_1() {
        testcase(vec![1,1,2], vec![1,2]);
    }

    /// Given nums = [0,0,1,1,1,2,2,3,3,4],
    ///
    /// Your function should return length = 5, with the first five elements of nums being modified to 0, 1, 2, 3, and 4 respectively.
    ///
    /// It doesn't matter what values are set beyond the returned length.
    #[test]
    pub fn example_2() {
        testcase(vec![0,0,1,1,1,2,2,3,3,4], vec![0,1,2,3,4]);
    }

    #[test]
    pub fn empty_input() {
        testcase(Vec::new(), Vec::new());
    }

    #[test]
    pub fn one_input() {
        testcase(vec![1], vec![1]);
    }

    #[test]
    pub fn no_duplicates() {
        testcase(vec![0,1,2,3], vec![0,1,2,3])
    }
}
