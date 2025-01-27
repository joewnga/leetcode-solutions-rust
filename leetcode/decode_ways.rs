/// 
/// Problem: Decode Ways
/// 
/// A message containing letters from **A-Z** is encoded using the following mapping:
/// ```plaintext
/// 'A' -> "1"
/// 'B' -> "2"
/// ...
/// 'Z' -> "26"
/// ```
/// Given a string `s`, return the **number of ways** to decode it.
///
/// **Example 1:**
/// ```plaintext
/// Input: s = "226"
/// Output: 3
/// Explanation: "226" can be decoded as "BZ", "VF", or "BBF".
/// ```
///
/// **Example 2:**
/// ```plaintext
/// Input: s = "06"
/// Output: 0
/// Explanation: "06" is invalid because "0" cannot map to any letter.
/// ```
///
/// **Constraints:**
/// - `1 <= s.length <= 100`
/// - `s` contains only digits and does not contain leading zeros except for "0".
///
/// # Solution:
/// - **Time Complexity:** `O(n)`
/// - **Space Complexity:** `O(n)`
impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        if s.is_empty() || s.starts_with('0') {
            return 0;
        }

        let chars: Vec<char> = s.chars().collect();
        let n = chars.len();
        let mut dp = vec![0; n + 1];

        dp[0] = 1; // Empty string has one valid decoding
        dp[1] = if chars[0] != '0' { 1 } else { 0 };

        for i in 2..=n {
            let one_digit = chars[i - 1].to_digit(10).unwrap();
            let two_digit = chars[i - 2].to_digit(10).unwrap() * 10 + one_digit;

            if one_digit > 0 {
                dp[i] += dp[i - 1];
            }
            if two_digit >= 10 && two_digit <= 26 {
                dp[i] += dp[i - 2];
            }
        }

        dp[n]
    }
}