#![allow(dead_code)]

struct Solution;

impl Solution {
    /// Implement [strStr()](http://www.cplusplus.com/reference/cstring/strstr/).
    ///
    /// Return the index of the first occurrence of needle in haystack, or -1 if needle is not part
    /// of haystack.
    ///
    /// **Clarification:**
    ///
    /// What should we return when needle is an empty string? This is a great question to ask during
    /// an interview.
    ///
    /// For the purpose of this problem, we will return 0 when needle is an empty string. This is consistent
    /// to C's strstr() and Java's indexOf().
    pub fn str_str_brute(haystack: String, needle: String) -> i32 {
        // This is a brute force implementation with no major optimisations
        if needle.is_empty() { return 0; }

        let haystack: Vec<char> = haystack.chars().collect();
        let needle: Vec<char> = needle.chars().collect();

        haystack
            .windows(needle.len())
            .position(|haystack_slice| {
                haystack_slice == &needle[..]
            })
            .map(|i| i as i32)
            .unwrap_or_else(|| -1)
    }

    /// Another basic implementation but implemented with less of  Rust's nice iterator tools to make
    /// it closer to something you might see in Java or C++
    pub fn str_str_brute_no_iterators(haystack: String, needle: String) -> i32 {
        // This is a brute force implementation with no major optimisations
        if needle.is_empty() { return 0; }

        let haystack: Vec<char> = haystack.chars().collect();
        let needle: Vec<char> = needle.chars().collect();

        'outer: for haystack_index in 0..haystack.len() {
            for needle_index in 0..needle.len() {
                if haystack_index + needle_index >= haystack.len() {
                    break 'outer;
                }

                if haystack[haystack_index + needle_index] != needle[needle_index] {
                    break;
                }

                if needle_index == needle.len() - 1 {
                    return haystack_index as i32;
                }
            }
        }

        -1
    }

    /// This is an implementation of `str_str` optimised for a `needle` that is uncommon
    /// within the `haystack`. It's a simplified variant of the Boyer-Moore algorithm.
    ///
    /// The idea is we can skip the entire length of the needle and
    /// we only need to backtrack by the length of the needle if the character we land on
    /// is contained within the needle.
    ///
    /// If the haystack mostly contains characters that aren't within the needle then we have
    /// good odds of skipping large parts of the haystack.
    ///
    /// This is particularly useful for parsing binary formats delimited with text such as the HTTP multipart
    /// file requests where large blobs of binary are surrounded by relatively uncommon boundary strings.
    pub fn str_str_uncommon_needle(haystack: String, needle: String) -> i32 {
        use std::collections::HashSet;
        use std::iter::FromIterator;

        if needle.is_empty() { return 0; }

        let haystack: Vec<char> = haystack.chars().collect();
        let needle: Vec<char> = needle.chars().collect();
        let needle_chars: HashSet<char> = HashSet::from_iter(needle.iter().cloned());

        let mut haystack_index = 0;
        while haystack_index < haystack.len() {
            // If `haystack_index + needle.len()` doesn't have _any_ of the characters
            // in the needle then we can skip the whole chunk since the character sequence
            // can't possibly result in the needle.
            while haystack_index + needle.len() - 1 < haystack.len() {
                if needle_chars.contains(&haystack[haystack_index + needle.len() - 1]) {
                    break;
                }

                haystack_index += needle.len();
            }

            // Brute force search. In the case where `haystack_index` is aligned with
            // the start of the needle this will find the result. If we're misaligned
            // and `haystack_index` is earlier then the needle we will essentially
            // stutter forward until we're aligned or no match is found.
            let haystack_needle = haystack.iter().skip(haystack_index).take(needle.len());
            if haystack_needle.eq(needle.iter()) {
                return haystack_index as i32;
            } else {
                haystack_index += 1;
            }
        }

        -1
    }

    pub fn str_str_kmp(_haystack: String, _needle: String) -> i32 {
        panic!("str_str_kmp not implemented yet");
    }

    pub fn str_str_boyer_moore(_haystack: String, _needle: String) -> i32 {
        panic!("str_str_boyer_moore not implemented yet");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_solutions(haystack: &str, needle: &str, expected: i32) {
        assert_eq!(Solution::str_str_brute(haystack.to_string(), needle.to_string()), expected, "str_str_brute failed");
        assert_eq!(Solution::str_str_brute_no_iterators(haystack.to_string(), needle.to_string()), expected, "str_str_brute_no_iterators failed");
        assert_eq!(Solution::str_str_uncommon_needle(haystack.to_string(), needle.to_string()), expected, "str_str_brute_uncommon_needle failed");
    }

    /// Input: haystack = "hello", needle = "ll"
    /// Output: 2
    #[test]
    pub fn example_1() {
        test_solutions("hello", "ll", 2);
    }

    /// Input: haystack = "aaaaa", needle = "bba"
    /// Output: -1
    #[test]
    pub fn example_2() {
        test_solutions("aaaaa", "bba", -1);
    }

    #[test]
    pub fn longer_needle_then_input() {
        test_solutions("aaa", "aaaa", -1);
    }

    #[test]
    pub fn empty_needle() {
        test_solutions("abc", "", 0);
    }

    #[test]
    pub fn submatch() {
        test_solutions("ababc", "abc", 2);
    }

    #[test]
    pub fn match_at_end() {
        test_solutions("mississippi", "issipi", -1);
    }
}
