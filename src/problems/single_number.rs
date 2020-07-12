#![allow(dead_code)]

struct Solution;

impl Solution {
    /// Given a non-empty array of integers, every element appears twice except
    /// for one. Find that single one.
    ///
    /// **Note:** Your algorithm should have a linear runtime complexity. Could you
    /// implement it without using extra memory?
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut sorted = nums;
        sorted.sort();

        // We can break the input into chunks of 2, if we find a chunk
        // with mismatched numbers then we know the leftmost value is the
        // odd one out since the array is sorted and the rightmost value _must_
        // have it's pair in the following chunk because all other values are
        // garunteed to have pairs.
        sorted
            .chunks(2)
            .find(|chunk| {
                match chunk {
                    [a, b] => a != b,
                    [_] => true,
                    _ => false
                }
            })
            .expect("Failed to find single number")
            [0]
    }

    pub fn single_number_no_sort(nums: Vec<i32>) -> i32 {
        let mut candidates = std::collections::HashSet::new();
        for &num in nums.iter() {
            // If we can't insert it we already have it
            if !candidates.insert(num) {
                // Since we already had the number we can remove it: We've seen
                // two of them and it's no longer a candidate.
                candidates.remove(&num);
            }
        }

        // There should only be one candidate left at this point
        // and it's our answer
        candidates
            .into_iter()
            .next()
            .expect("Failed to find single number")
    }

    /// I didn't come up with this solution, I a similar solution on leetcode
    /// after writing the previous two.
    ///
    /// It's such a clever trick I had to copy it down!
    pub fn single_number_no_extra_memory_no_sort(nums: Vec<i32>) -> i32 {
        // XORing the same number twice cancels itself out. This means
        // the only XOR that applies to `0` is the single number we're
        // lookng for
        nums.iter().fold(0, |a, b| a ^ b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    pub fn test_all_solutions(input: Vec<i32>, expected_output: i32) {
        assert_eq!(Solution::single_number(input.clone()), expected_output);
        assert_eq!(Solution::single_number_no_sort(input.clone()), expected_output);
        assert_eq!(
            Solution::single_number_no_extra_memory_no_sort(input.clone()),
            expected_output
        );
    }

    /// Input: [2,2,1]
    /// Output: 1
    #[test]
    pub fn example_1() {
        test_all_solutions(vec![2,2,1], 1);
    }

    // Input: [4,1,2,1,2]
    // Output: 4
    #[test]
    pub fn example_2() {
        test_all_solutions(vec![4,1,2,1,2], 4)
    }

    #[test]
    pub fn one_input() {
        test_all_solutions(vec![5], 5);
    }
}
