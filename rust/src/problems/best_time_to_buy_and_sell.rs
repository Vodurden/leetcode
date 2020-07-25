#![allow(dead_code)]

struct Solution;

impl Solution {
    /// Say you have an array `prices` for which the `ith` element is the price of a given stock on day `i`.
    ///
    /// Design an algorithm to find the maximum profit. You may complete as many transactions as you like (i.e., buy one and sell one share of the stock multiple times).
    ///
    /// **Note:** You may not engage in multiple transactions at the same time (i.e., you must sell the stock before you buy again)
    ///
    /// **Constraints:**
    ///
    /// - `1 <= prices.length <= 3 * 10 ^ 4`
    /// - `0 <= prices[i] <= 10 ^ 4`
    ///
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        // We basically just need to sum any day where the value goes up, it helps
        // to look at the input data as a graph.
        prices
            .windows(2)
            .map(|window| {
                match window {
                    [a, b] if b > a => b - a,
                    _ => 0
                }
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Input: [7,1,5,3,6,4]
    /// Output: 7
    /// Explanation:
    /// - Buy on day 2 (price = 1) and sell on day 3 (price = 5), profit = 5-1 = 4.
    /// - Then buy on day 4 (price = 3) and sell on day 5 (price = 6), profit = 6-3 = 3.
    #[test]
    pub fn example_1() {
        let input = vec![7,1,5,3,6,4];
        let expected_result = 7;
        assert_eq!(Solution::max_profit(input), expected_result);
    }

    /// Input: [1,2,3,4,5]
    /// Output: 4
    /// Explanation: Buy on day 1 (price = 1) and sell on day 5 (price = 5), profit = 5-1 = 4.
    ///
    /// Note that you cannot buy on day 1, buy on day 2 and sell them later, as you are
    /// engaging multiple transactions at the same time. You must sell before buying again.
    #[test]
    pub fn example_2() {
        let input = vec![1,2,3,4,5];
        let expected_result = 4;
        assert_eq!(Solution::max_profit(input), expected_result);
    }

    /// Input: [7,6,4,3,1]
    /// Output: 0
    /// Explanation: In this case, no transaction is done, i.e. max profit = 0.
    #[test]
    pub fn example_3() {
        let input = vec![7,6,4,3,1];
        let expected_result = 0;
        assert_eq!(Solution::max_profit(input), expected_result);
    }

    #[test]
    pub fn empty() {
        let input = Vec::new();
        let expected_result = 0;
        assert_eq!(Solution::max_profit(input), expected_result);
    }

    #[test]
    pub fn one_input() {
        let input = vec![5];
        let expected_result = 0;
        assert_eq!(Solution::max_profit(input), expected_result);
    }
}
