#![allow(dead_code)]

struct Solution;

impl Solution {
    /// Given two arrays, write a function to compute their intersection.
    ///
    /// **Note:**
    /// - Each element in the result should appear as many times as it shows in both arrays.
    /// - The result can be in any order.
    ///
    /// **Follow Up:**
    /// - What if the given array is already sorted? How would you optimize your algorithm?
    /// - What if nums1's size is small compared to nums2's size? Which algorithm is better?
    /// - What if elements of nums2 are stored on disk, and the memory is limited such that you cannot load all elements into the memory at once?
    ///
    pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut sorted1 = nums1;
        sorted1.sort();
        let sorted1 = sorted1;

        let mut sorted2 = nums2;
        sorted2.sort();
        let sorted2 = sorted2;

        let mut index1 = 0;
        let mut index2 = 0;

        let mut result: Vec<i32> = Vec::new();
        while index1 < sorted1.len() && index2 < sorted2.len() {
            match (sorted1.get(index1), sorted2.get(index2)) {
                (Some(a), Some(b)) if a == b => {
                    result.push(*a);
                    index1 += 1;
                    index2 += 1;
                },
                (Some(a), Some(b)) if a > b => index2 += 1,
                (Some(_), Some(_)) /* if a < b */ => index1 += 1,
                (_, None) => index1 += 1,
                (None, _) => index2 += 1,
            }
        }

        result
    }

    pub fn intersect_no_sort(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut counts = std::collections::HashMap::<i32, (usize, usize)>::new();
        for n in nums1 {
            counts
                .entry(n)
                .and_modify(|(num_1_count, _)| *num_1_count += 1)
                .or_insert((1, 0));
        }

        for n in nums2 {
            counts
                .entry(n)
                .and_modify(|(_, num_2_count)| *num_2_count += 1)
                .or_insert((0, 1));
        }

        counts.into_iter().flat_map(|(key, (num_1_count, num_2_count))| {
            let repetitions = std::cmp::min(num_1_count, num_2_count);
            vec![key; repetitions]
        }).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    pub fn test_all_solutions(
        input1: Vec<i32>,
        input2: Vec<i32>,
        expected_output: Vec<i32>
    ) {
        test_solution(&input1, &input2, &expected_output, Solution::intersect);
        test_solution(&input1, &input2, &expected_output, Solution::intersect_no_sort);
    }

    pub fn test_solution(
        input1: &Vec<i32>,
        input2: &Vec<i32>,
        expected_output: &Vec<i32>,
        solution: fn(Vec<i32>, Vec<i32>) -> Vec<i32>
    ) {
        // We sort the results and expected output because we don't care
        // about the order of results for this excercise.
        let mut result = solution(input1.clone(), input2.clone());
        result.sort();

        let mut expected_output = expected_output.clone();
        expected_output.sort();

        assert_eq!(result, expected_output);
    }

    /// Input: nums1 = [1,2,2,1], nums2 = [2,2]
    /// Output: [2,2]
    #[test]
    pub fn example_1() {
        test_all_solutions(
            vec![1,2,2,1],
            vec![2,2],
            vec![2,2]
        );
    }

    /// Input: nums1 = [4,9,5], nums2 = [9,4,9,8,4]
    /// Output: [4,9]
    #[test]
    pub fn example_2() {
        test_all_solutions(
            vec![4,9,5],
            vec![9,4,9,8,4],
            vec![4,9]
        );
    }

    #[test]
    pub fn empty_lhs() {
        test_all_solutions(
            Vec::new(),
            vec![9,4,9,8,4],
            Vec::new()
        );
    }

    #[test]
    pub fn empty_rhs() {
        test_all_solutions(
            vec![4,9,5],
            Vec::new(),
            Vec::new()
        );
    }
}
