#![allow(dead_code)]

struct Solution;

impl Solution {
    /// The count-and-say sequence is the sequence of integers with the first five terms as following:
    ///
    /// ```text
    /// 1.     1
    /// 2.     11
    /// 3.     21
    /// 4.     1211
    /// 5.     111221
    /// ```
    ///
    /// 1 is read off as "one 1" or 11.
    /// 11 is read off as "two 1s" or 21.
    /// 21 is read off as "one 2, then one 1" or 1211.
    ///
    /// Given an integer n where 1 ≤ n ≤ 30, generate the nth term of the count-and-say sequence. You can do so recursively, in other words
    /// from the previous member read off the digits, counting the number of digits in groups of the same digit.
    ///
    /// Note: Each term of the sequence of integers will be represented as a string.
    pub fn count_and_say(n: i32) -> String {
        if n <= 1 { return "1".to_string(); }

        let previous_value = Solution::count_and_say(n-1);
        let previous_value: Vec<char> = previous_value.chars().collect();

        let mut count = 0;
        let mut result = String::new();

        let mut iterator = previous_value.into_iter().peekable();
        while let Some(c) = iterator.next() {
            count += 1;

            if Some(&c) != iterator.peek() {
                result.push_str(&count.to_string());
                result.push(c);

                count = 0;
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_solution(n: i32, expected: &str) {
        assert_eq!(Solution::count_and_say(n), expected.to_string());
    }

    #[test]
    pub fn base_case() {
        test_solution(1, "1")
    }

    #[test]
    pub fn count_and_say_2() {
        test_solution(2, "11")
    }

    #[test]
    pub fn count_and_say_3() {
        test_solution(3, "21")
    }

    #[test]
    pub fn count_and_say_4() {
        test_solution(4, "1211");
    }

    #[test]
    pub fn count_and_say_5() {
        test_solution(5, "111221");
    }

    #[test]
    pub fn count_and_say_6() {
        test_solution(6, "312211");
    }

    #[test]
    pub fn count_and_say_17() {
        test_solution(17, "11131221131211132221232112111312212321123113112221121113122113111231133221121321132132211331121321231231121113122113322113111221131221");
    }
}
