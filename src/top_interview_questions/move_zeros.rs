#![allow(dead_code)]

struct Solution;

impl Solution {
    /// Given an array nums, write a function to move all 0's to the end of it while
    /// maintaining the relative order of the non-zero elements.
    ///
    /// **Note:**
    /// - You must do this in-place without making a copy of the array.
    /// - Minimize the total number of operations.
    ///
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut zero_index = 0;
        for i in 0..nums.len() {
            if nums[i] != 0 {
                nums.swap(i, zero_index);
                zero_index += 1;
            }
        }
    }

    /// This was my first try that turned out to be _waay_ too complicated. But it
    /// was on the right track, just overthinking the problem
    pub fn move_zeroes_first_try(nums: &mut Vec<i32>) {
        // Strategy:
        //
        // - Loop forward until we find a non-zero
        // - Swap the value with the earliest known zero position
        // - Continue from where we found the non-zero
        let mut first_available_zero_index: Option<usize> = None;
        for i in 0..nums.len() {

            let n = nums[i];

            // We've found a non-zero, if we have an available
            // zero position then we should move back there, otherwise
            // we can just stay where we are
            if n != 0 {
                if let Some(zero_index) = first_available_zero_index {
                    nums.swap(i, zero_index);
                    first_available_zero_index = None;

                    // Now let's see if we have any available zeroes
                    // between us an the current index, this allows us
                    // to fill out zeroes even if we've already moved
                    // past them in the main loop
                    for j in zero_index..=i {
                        if nums[j] == 0 {
                            first_available_zero_index = Some(j);
                            break;
                        }
                    }
                }
            } else if n == 0 && first_available_zero_index == None {
                first_available_zero_index = Some(i);
            }

        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    pub fn test_all_solutions(input: Vec<i32>, expected_output: Vec<i32>) {
        test_solution(&input, &expected_output, Solution::move_zeroes);
        test_solution(&input, &expected_output, Solution::move_zeroes_first_try);
    }

    pub fn test_solution(
        input: &Vec<i32>,
        expected_output: &Vec<i32>,
        solution: fn(&mut Vec<i32>)
    ) {
        let mut input = input.clone();
        solution(&mut input);
        assert_eq!(input, expected_output.clone());
    }

    // Input: [0,1,0,3,12]
    // Output: [1,3,12,0,0]
    #[test]
    pub fn example_1() {
        test_all_solutions(vec![0,1,0,3,12], vec![1,3,12,0,0]);
    }

    #[test]
    pub fn chained_zeroes() {
        test_all_solutions(
            vec![1,0,0,5,0,0,0,12,5,0,1],
            vec![1,5,12,5,1,0,0,0,0,0,0]
        )
    }

    #[test]
    pub fn empty() {
        test_all_solutions(Vec::new(), Vec::new());
    }

    pub fn one_input() {
        test_all_solutions(vec![5], vec![5]);
    }
}
