#![allow(dead_code)]

struct Solution;

impl Solution {
    /// Implement atoi which converts a string to an integer.
    ///
    /// The function first discards as many whitespace characters as necessary until the first non-whitespace character is found. Then, starting from this character, takes
    /// an optional initial plus or minus sign followed by as many numerical digits as possible, and interprets them as a numerical value.
    ///
    /// The string can contain additional characters after those that form the integral number, which are ignored and have no effect on the behavior of this function.
    ///
    /// If the first sequence of non-whitespace characters in str is not a valid integral number, or if no such sequence exists because either str is empty or it
    /// contains only whitespace characters, no conversion is performed.
    ///
    /// If no valid conversion could be performed, a zero value is returned.
    ///
    /// Note:
    ///
    /// - Only the space character `' '` is considered as whitespace character.
    /// - Assume we are dealing with an environment which could only store integers within the 32-bit signed integer range: [−2^31,  2^(31 − 1)]. If the numerical value is
    ///   out of the range of representable values, INT_MAX (2^(31 − 1)) or INT_MIN (−2^31) is returned.
    pub fn my_atoi(s: String) -> i32 {
        let mut iter = s.chars().into_iter().peekable();

        // Skip whitespace
        while let Some(' ') = iter.peek() {
            iter.next();
        }

        // Determine positive/negative
        let mut negative = false;
        match iter.peek() {
            Some('+') => { iter.next(); },
            Some('-') => {
                iter.next();
                negative = true;
            }
            _ => {}
        }

        // Read number
        let mut number: i32 = 0;
        for c in iter {
            if !c.is_numeric() { break; }

            let digit = c as i32 - '0' as i32;
            number = number.saturating_mul(10);
            number = if negative {
                number.saturating_sub(digit)
            } else {
                number.saturating_add(digit)
            };
        }

        number
    }

    /// Like `my_atoi` but trying to use iterators more with a slightly different solution
    pub fn my_atoi_iterator(s: String) -> i32 {
        use std::convert::TryFrom;

        let mut number_chars: Vec<char> = s
            .chars()
            .skip_while(|c| *c == ' ')
            .take_while(|c| *c == '+' || *c == '-' || c.is_numeric())
            .collect();

        let digit_start = number_chars.iter().position(|&c| c.is_numeric()).unwrap_or(0);
        let digits: Vec<char> = number_chars.split_off(digit_start).into_iter().take_while(|&c| c != '-' && c != '+').collect();
        let prefix = number_chars; // The set of '+' and '-' prefixing the number.

        // Handle a case where the prefix is +-+- or some variation.
        let negative = match &prefix[..] {
            [] => false,
            &[c] => c == '-',
            _ => return 0, // If we have >1 prefix character we have a string like +--+ which is invalid
        };

        let number = digits
            .into_iter()
            .filter(|&c| c != '-' && c != '+')
            .fold(Some(0), |acc: Option<i32>, c: char| {
                let digit = i32::try_from(c.to_digit(10)?).ok()?;
                let acc = acc?.checked_mul(10)?;
                let acc = acc.checked_add(digit)?;

                Some(acc)
            });

        let negative_multiplier = if negative { -1 } else { 1 };
        number
            .and_then(|n| n.checked_mul(negative_multiplier))
            .unwrap_or_else(|| {
                if negative {
                    std::i32::MIN
                } else {
                    std::i32::MAX
                }
            })
    }

    /// Same as `my_atoi` but implemented without using as much of the iterator api.
    pub fn my_atoi_loops(s: String) -> i32 {
        let chars: Vec<char> = s.chars().collect();
        let mut index = 0;

        if chars.len() == 0 {
            return 0;
        }

        // Skip all whitespace
        while index < chars.len() {
            if chars[index] != ' ' {
                break;
            }

            index += 1;
        }

        if index >= chars.len() {
            return 0;
        }

        // Read a single '+' or '-'. If we see a digit skip to the next section
        let prefix = match chars[index] {
            '-' => Some('-'),
            '+' => Some('+'),
            _ => None,
        };
        let negative = prefix == Some('-');
        if let Some(_) = prefix {
            index += 1;
        }

        // Now we can read all our digits and construct our number
        let mut number: i32 = 0;
        while index < chars.len() && chars[index].is_numeric() {
            let digit = chars[index] as i32 - '0' as i32;
            number = number.saturating_mul(10);
            number = if negative {
                number.saturating_sub(digit)
            } else {
                number.saturating_add(digit)
            };

            index += 1;
        }

        number
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    pub fn test_all_solutions(s: &str, expected: i32) {
        assert_eq!(Solution::my_atoi(s.to_string()), expected, "my_atoi failed on value \"{}\"", s);
        assert_eq!(Solution::my_atoi_loops(s.to_string()), expected, "my_atoi_loops failed on value \"{}\"", s);
        assert_eq!(Solution::my_atoi_iterator(s.to_string()), expected, "my_atoi_iterator failed on value \"{}\"", s);
    }

    /// Input: "42"
    /// Output: 42
    #[test]
    pub fn example_1() {
        test_all_solutions("42", 42);
    }

    /// Input: "   -42"
    /// Output: -42
    /// Explanation: The first non-whitespace character is '-', which is the minus sign. Then take as many numerical digits as possible, which gets 42.
    #[test]
    pub fn example_2() {
        test_all_solutions("   -42", -42);
    }

    /// Input: "4193 with words"
    /// Output: 4193
    /// Explanation: Conversion stops at digit '3' as the next character is not a numerical digit.
    #[test]
    pub fn example_3() {
        test_all_solutions("4193 with words", 4193);
    }

    /// Input: "words and 987"
    /// Output: 0
    /// Explanation: The first non-whitespace character is 'w', which is not a numerical digit or a +/- sign. Therefore no valid conversion could be performed.
    #[test]
    pub fn example_4() {
        test_all_solutions("words and 987", 0);
    }

    /// Input: "-91283472332"
    /// Output: -2147483648
    /// Explanation: The number "-91283472332" is out of the range of a 32-bit signed integer. Thefore INT_MIN (−2^31) is returned.
    #[test]
    pub fn example_5() {
        test_all_solutions("-91283472332", -2147483648)
    }

    #[test]
    pub fn negative_non_overflow_edgecase() {
        test_all_solutions("-2147483648", -2147483648);
    }

    #[test]
    pub fn empty() {
        test_all_solutions("", 0);
    }

    #[test]
    pub fn whitespace_only() {
        test_all_solutions(" ", 0);
    }

    #[test]
    pub fn whitespaces_only() {
        test_all_solutions("   ", 0);
    }

    #[test]
    pub fn prefix_only() {
        test_all_solutions("+", 0);
    }

    #[test]
    pub fn prefixs_only() {
        test_all_solutions("-+", 0);
    }

    #[test]
    pub fn explicit_plus() {
        test_all_solutions("+1", 1);
    }

    /// This _should_ fail but could easily trick some implementations
    #[test]
    pub fn mixed_plus_and_minus() {
        test_all_solutions("+-1", 0);
    }

    #[test]
    pub fn infix_minus() {
        test_all_solutions("0-1", 0);
    }

    #[test]
    pub fn infix_minus_with_value() {
        test_all_solutions("10-1", 10);
    }

    #[test]
    pub fn infix_plus() {
        test_all_solutions("0+1", 0);
    }

    #[test]
    pub fn infix_plus_with_value() {
        test_all_solutions("10+1", 10);
    }
}
