#![allow(dead_code)]

struct Solution;

impl Solution {
    /// Given an array of integers, return indices of the two numbers such that they
    /// add up to a specific target.
    ///
    /// You may assume that each input would have exactly one solution, and you may
    /// not use the same element twice.
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for i in 0..nums.len() {
            for j in i+1..nums.len() {
                if nums[i] + nums[j] == target {
                    return vec![i as i32, j as i32];
                }
            }
        }

        panic!("No solution found");
    }

    pub fn two_sum_hash_map(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut differences = std::collections::HashMap::<i32, usize>::new();
        for (i, &n) in nums.iter().enumerate() {
            // Calculate the number we need to have already seen to have a solution
            let needed_number: i32 = target - n;
            if let Some(first_index) = differences.get(&needed_number) {
                return vec![*first_index as i32, i as i32]
            } else {
                differences.insert(n, i);
            }
        }

        panic!("No solution found");
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    pub fn test_all_solutions(
        input: Vec<i32>,
        target: i32,
        expected_output: Vec<i32>
    ) {
        test_solution(&input, target, &expected_output, Solution::two_sum);
        test_solution(&input, target, &expected_output, Solution::two_sum_hash_map);
    }

    pub fn test_solution(
        input: &Vec<i32>,
        target: i32,
        expected_output: &Vec<i32>,
        solution: fn(Vec<i32>, i32) -> Vec<i32>
    ) {
        let result = solution(input.clone(), target);
        assert_eq!(result, expected_output.clone());
    }

    /// Given nums = [2, 7, 11, 15], target = 9,
    ///
    /// Because nums[0] + nums[1] = 2 + 7 = 9,
    /// return [0, 1].
    #[test]
    pub fn example_1() {
        test_all_solutions(vec![2,7,11,15], 9, vec![0, 1]);
    }


    #[test]
    pub fn sum_zero() {
        test_all_solutions(vec![0,4,3,0], 0, vec![0,3]);
    }

    #[test]
    pub fn sum_negatives() {
        test_all_solutions(vec![-1,-2,-3,-4,-5], -8, vec![2,4]);
    }
}
