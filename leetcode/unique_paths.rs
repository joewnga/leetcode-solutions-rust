/// 
/// Problem: Unique Paths
/// 
/// A robot is located at the **top-left corner** of an `m x n` grid.
/// The robot can only **move right or down** at any point in time.
/// The robot is trying to reach the **bottom-right corner** of the grid.
///
/// How many possible unique paths are there?
///
/// **Example 1:**
/// ```plaintext
/// Input: m = 3, n = 7
/// Output: 28
/// ```
///
/// **Example 2:**
/// ```plaintext
/// Input: m = 3, n = 2
/// Output: 3
/// Explanation: The possible paths are: "Right → Down → Down", "Down → Down → Right", "Down → Right → Down".
/// ```
///
/// **Constraints:**
/// - `1 <= m, n <= 100`
///
/// # Solution:
/// - **Time Complexity:** `O(1)`
/// - **Space Complexity:** `O(1)`
impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let (m, n) = (m as u64, n as u64);
        let mut res = 1;
        for i in 1..n {
            res = res * (m + i - 1) / i;
        }
        res as i32
    }
}