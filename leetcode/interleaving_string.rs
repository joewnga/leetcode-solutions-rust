/// 
/// Problem: Interleaving String
/// 
/// Given strings `s1`, `s2`, and `s3`, return `true` if `s3` is formed by **interleaving** `s1` and `s2`.
/// 
/// An interleaving means:
/// - **Characters from `s1` and `s2` must appear in order.**
/// - **No reordering of `s1` or `s2` characters is allowed.**
///
/// **Example 1:**
/// ```plaintext
/// Input: s1 = "aab", s2 = "axy", s3 = "aaxaby"
/// Output: true
/// Explanation: "aaxaby" interleaves "aab" and "axy".
/// ```
///
/// **Example 2:**
/// ```plaintext
/// Input: s1 = "abc", s2 = "def", s3 = "abdecf"
/// Output: false
/// Explanation: "def" appears in the wrong order.
/// ```
///
/// **Constraints:**
/// - `1 <= s1.length, s2.length <= 100`
/// - `s1`, `s2`, and `s3` contain only lowercase letters.
///
/// # Solution:
/// - **Time Complexity:** `O(m * n)`
/// - **Space Complexity:** `O(n)`

impl Solution {
    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        let (m, n, l) = (s1.len(), s2.len(), s3.len());
        if m + n != l {
            return false;
        }

        let s1 = s1.as_bytes();
        let s2 = s2.as_bytes();
        let s3 = s3.as_bytes();

        let mut dp = vec![false; n + 1];
        dp[0] = true;

        for j in 1..=n {
            dp[j] = dp[j - 1] && s2[j - 1] == s3[j - 1];
        }

        for i in 1..=m {
            dp[0] = dp[0] && s1[i - 1] == s3[i - 1];
            for j in 1..=n {
                dp[j] = (dp[j] && s1[i - 1] == s3[i + j - 1]) ||
                        (dp[j - 1] && s2[j - 1] == s3[i + j - 1]);
            }
        }

        dp[n]
    }
}