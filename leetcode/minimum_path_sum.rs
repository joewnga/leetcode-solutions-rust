/// 
/// Problem: Minimum Path Sum
/// 
/// Given an `m x n` grid filled with **non-negative** numbers, find a path from the **top-left** to the **bottom-right**
/// that minimizes the sum of all numbers along the path.
/// 
/// You can only move **right or down** at any point in time.
///
/// **Example 1:**
/// ```plaintext
/// Input: grid = [
///   [1,3,1],
///   [1,5,1],
///   [4,2,1]
/// ]
/// Output: 7
/// Explanation: The path `1 → 3 → 1 → 1 → 1` minimizes the sum to `7`.
/// ```
///
/// **Example 2:**
/// ```plaintext
/// Input: grid = [
///   [1,2,3],
///   [4,5,6]
/// ]
/// Output: 12
/// ```
///
/// **Constraints:**
/// - `1 <= m, n <= 200`
/// - `0 <= grid[i][j] <= 100`
///
/// # Solution:
/// - **Time Complexity:** `O(m * n)`
/// - **Space Complexity:** `O(n)`

impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut dp = vec![i32::MAX; n];

        dp[0] = 0;

        for row in grid.iter() {
            for j in 0..n {
                if j == 0 {
                    dp[j] += row[j];
                } else {
                    dp[j] = dp[j].min(dp[j - 1]) + row[j];
                }
            }
        }

        dp[n - 1]
    }
}