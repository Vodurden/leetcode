#![allow(dead_code)]

struct Solution;

impl Solution {
    /// Given a 32-bit signed integer, reverse digits of an integer.
    ///
    /// **Note:**
    /// Assume we are dealing with an environment which could only store integers within the 32-bit signed integer
    /// range: [−2^31,  2^(31 − 1)]. For the purpose of this problem, assume that your function returns 0 when the
    /// reversed integer overflows.
    pub fn reverse(x: i32) -> i32 {
        let mut number = x.abs();
        let mut result: i32 = 0;
        while number > 0 {
            let last_digit = number % 10;
            let (reversed, overflow_mul) = result.overflowing_mul(10);
            let (reversed, overflow_add) = reversed.overflowing_add(last_digit);
            result = reversed;

            if overflow_mul || overflow_add {
                return 0;
            }

            number /= 10;
        }

        // If the original number was negative then we need
        // to negate our result since we removed the sign
        // in the previous step.
        if x < 0 {
            result *= -1;
        }

        result
    }

    /// Same as `reverse` but implemented with strings
    pub fn reverse_with_string(x: i32) -> i32 {
        let reversed: String = x.abs().to_string().chars().rev().collect();
        let reversed: i32 = reversed.parse().unwrap_or(0);

        // If the original number was negative then we need
        // to negate our result since we removed the sign
        // in the previous step.
        if x >= 0 {
            reversed
        } else {
            reversed * -1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_all_solutions(input: i32, expected: i32) {
        assert_eq!(Solution::reverse(input), expected);
        assert_eq!(Solution::reverse_with_string(input), expected);
    }

    /// Input: 123
    /// Output: 321
    #[test]
    pub fn example_1() {
        test_all_solutions(123, 321);
    }

    /// Input: -123
    /// Output: -321
    #[test]
    pub fn example_2() {
        test_all_solutions(-123, -321);
    }

    /// Input: 120
    /// Output: 21
    #[test]
    pub fn example_3() {
        test_all_solutions(120, 21);
    }

    #[test]
    pub fn zero() {
        test_all_solutions(0, 0);
    }

    #[test]
    pub fn out_of_bounds() {
        test_all_solutions(1534236469, 0);
    }
}
