/// 
/// Problem: Minimum Window Substring
/// 
/// Given two strings `s` and `t`, return the **smallest substring in `s`** that contains **all characters of `t`** in any order.
/// 
/// If such a substring does not exist, return an **empty string**.
///
/// **Example 1:**
/// ```plaintext
/// Input: s = "ADOBECODEBANC", t = "ABC"
/// Output: "BANC"
/// ```
///
/// **Example 2:**
/// ```plaintext
/// Input: s = "a", t = "a"
/// Output: "a"
/// ```
///
/// **Example 3:**
/// ```plaintext
/// Input: s = "a", t = "aa"
/// Output: ""
/// Explanation: `t` requires two 'a's, but `s` only has one.
/// ```
///
/// **Constraints:**
/// - `1 <= s.length, t.length <= 10^5`
/// - `s` and `t` consist of **uppercase and lowercase English letters**.
///
/// # Solution:
/// - **Time Complexity:** `O(n)`
/// - **Space Complexity:** `O(1)`
use std::collections::HashMap;

impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        let (s_bytes, t_bytes) = (s.as_bytes(), t.as_bytes());
        let mut target_count = HashMap::new();
        let mut window_count = HashMap::new();

        for &c in t_bytes {
            *target_count.entry(c).or_insert(0) += 1;
        }

        let (mut left, mut right) = (0, 0);
        let (mut min_len, mut min_start) = (usize::MAX, 0);
        let mut required = target_count.len();
        let mut formed = 0;

        while right < s_bytes.len() {
            let c = s_bytes[right];
            *window_count.entry(c).or_insert(0) += 1;

            if target_count.contains_key(&c) && window_count[&c] == target_count[&c] {
                formed += 1;
            }

            while formed == required {
                let start_char = s_bytes[left];

                if right - left + 1 < min_len {
                    min_len = right - left + 1;
                    min_start = left;
                }

                *window_count.get_mut(&start_char).unwrap() -= 1;
                if target_count.contains_key(&start_char) && window_count[&start_char] < target_count[&start_char] {
                    formed -= 1;
                }
                left += 1;
            }
            right += 1;
        }

        if min_len == usize::MAX {
            "".to_string()
        } else {
            s[min_start..min_start + min_len].to_string()
        }
    }
}