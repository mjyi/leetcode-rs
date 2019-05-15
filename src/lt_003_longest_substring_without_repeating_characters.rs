/*
Given a string, find the length of the longest substring without repeating characters.

Example 1:

Input: "abcabcbb"
Output: 3
Explanation: The answer is "abc", with the length of 3.

Example 2:

Input: "bbbbb"
Output: 1
Explanation: The answer is "b", with the length of 1.

Example 3:

Input: "pwwkew"
Output: 3
Explanation: The answer is "wke", with the length of 3.
             Note that the answer must be a substring, "pwke" is a subsequence and not a substring.

*/

pub struct Solution;

use std::cmp;
use std::collections::HashMap;

// use HashMap
// Runtime 4 ms Memory 2.5M
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut max_length = 0;
        let mut head = 0;
        let mut map: HashMap<char, i32> = HashMap::new();

        for (i, c) in s.chars().enumerate() {
            let tail = i as i32;

            if let Some(v) = map.get(&c) {
                head = cmp::max(*v + 1, head);
            }
            max_length = cmp::max(tail + 1 - head, max_length);

            map.insert(c, tail);
        }

        max_length
    }
}

// use Vec
// Runtime 0 ms Memory 2.6M
mod OtherSolution {

    use std::cmp;

    struct Solution;

    impl Solution {
        pub fn length_of_longest_substring(s: String) -> i32 {
            let mut max_len: i32 = 0;
            let mut head: i32 = 0;
            let mut index: Vec<i32> = vec![0; 128];

            for (j, c) in s.chars().enumerate() {
                let tail = j as i32;
                let i = (c as u32) as usize;
                head = cmp::max(index[i], head);
                max_len = cmp::max(max_len, tail + 1 - head);
                index[i] = (tail + 1) as i32;

                println!("- {} {} {}", head, tail, max_len);
            }
            max_len
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string() {
        assert_eq!(
            Solution::length_of_longest_substring("abcabcbb".to_string()),
            3
        );
    }

}
