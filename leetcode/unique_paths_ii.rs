/// 
/// Problem: Unique Paths II
/// 
/// You are given an `m x n` grid where:
/// - `0` represents an **empty cell**.
/// - `1` represents an **obstacle**.
/// 
/// The robot can only move **right or down** at any point in time.
/// The robot starts at `(0,0)` and tries to reach the **bottom-right corner** `(m-1, n-1)`.
///
/// Find the number of unique paths that the robot can take.
///
/// **Example 1:**
/// ```plaintext
/// Input: obstacle_grid = [
///  [0,0,0],
///  [0,1,0],
///  [0,0,0]
/// ]
/// Output: 2
/// Explanation: The two paths are:
/// 1. Right → Right → Down → Down
/// 2. Down → Down → Right → Right
/// ```
///
/// **Example 2:**
/// ```plaintext
/// Input: obstacle_grid = [
///  [0,1],
///  [0,0]
/// ]
/// Output: 1
/// ```
///
/// **Constraints:**
/// - `1 <= m, n <= 100`
/// - `obstacle_grid[i][j]` is `0` (empty) or `1` (obstacle).
///
/// # Solution:
/// - **Time Complexity:** `O(m * n)`
/// - **Space Complexity:** `O(n)`
impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        let m = obstacle_grid.len();
        let n = obstacle_grid[0].len();
        let mut dp = vec![0; n];

        if obstacle_grid[0][0] == 1 {
            return 0;
        }

        dp[0] = 1;

        for i in 0..m {
            for j in 0..n {
                if obstacle_grid[i][j] == 1 {
                    dp[j] = 0;
                } else if j > 0 {
                    dp[j] += dp[j - 1];
                }
            }
        }

        dp[n - 1]
    }
}