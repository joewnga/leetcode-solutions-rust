/// 
/// Problem: Unique Binary Search Trees
/// 
/// Given an integer `n`, return the number of **structurally unique BSTs** that store values `1` to `n`.
/// 
/// **Example 1:**
/// ```plaintext
/// Input: n = 3
/// Output: 5
/// Explanation: There are exactly 5 unique BSTs for `n = 3`.
/// ```
///
/// **Example 2:**
/// ```plaintext
/// Input: n = 1
/// Output: 1
/// ```
///
/// **Constraints:**
/// - `1 <= n <= 19`
///
/// # Solution:
/// - **Time Complexity:** `O(n^2)`
/// - **Space Complexity:** `O(n)`

impl Solution {
    pub fn num_trees(n: i32) -> i32 {
        let n = n as usize;
        let mut dp = vec![0; n + 1];
        dp[0] = 1; 
        dp[1] = 1; 

        for i in 2..=n {
            for j in 1..=i {
                dp[i] += dp[j - 1] * dp[i - j];
            }
        }

        dp[n]
    }
}

