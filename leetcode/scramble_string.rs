/// 
/// Problem: Scramble String
/// 
/// We define a scramble of a string `s` using the following rules:
/// - If a string consists of one character, it is a scramble of itself.
/// - If a string can be divided into two non-empty substrings, they can be swapped.
/// - You can swap the substrings recursively to form another string.
///
/// Given two strings `s1` and `s2` of the same length, return `true` if `s2` is a scramble of `s1`.
///
/// **Example 1:**
/// ```plaintext
/// Input: s1 = "great", s2 = "rgeat"
/// Output: true
/// Explanation: You can swap "gr" and "eat" to match.
/// ```
///
/// **Example 2:**
/// ```plaintext
/// Input: s1 = "abcde", s2 = "caebd"
/// Output: false
/// ```
///
/// **Constraints:**
/// - `1 <= s1.length <= 30`
/// - `s1` and `s2` consist of lowercase English letters.
///
/// # Solution:
/// - **Time Complexity:** `O(n^4)`
/// - **Space Complexity:** `O(n^2)`
use std::collections::HashMap;

impl Solution {
    pub fn is_scramble(s1: String, s2: String) -> bool {
        let mut memo = HashMap::new();
    Self::scramble(&s1, &s2, &mut memo)
}

fn scramble(s1: &str, s2: &str, memo: &mut HashMap<(String, String), bool>) -> bool {
    if s1 == s2 {
        return true;
    }
    if s1.len() != s2.len() {
        return false;
    }
    if let Some(&result) = memo.get(&(s1.to_string(), s2.to_string())) {
        return result;
    }

    let n = s1.len();
    let mut count = [0; 26];

    for i in 0..n {
        count[(s1.as_bytes()[i] - b'a') as usize] += 1;
        count[(s2.as_bytes()[i] - b'a') as usize] -= 1;
    }

    for i in 0..26 {
        if count[i] != 0 {
            memo.insert((s1.to_string(), s2.to_string()), false);
            return false;
        }
    }

    for i in 1..n {
        if Self::scramble(&s1[0..i], &s2[0..i], memo) && Self::scramble(&s1[i..], &s2[i..], memo) {
            memo.insert((s1.to_string(), s2.to_string()), true);
            return true;
        }
        if Self::scramble(&s1[0..i], &s2[n - i..], memo) && Self::scramble(&s1[i..], &s2[0..n - i], memo) {
            memo.insert((s1.to_string(), s2.to_string()), true);
            return true;
        }
    }

    memo.insert((s1.to_string(), s2.to_string()), false);
    false
    }
}