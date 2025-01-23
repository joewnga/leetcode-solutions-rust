/// 
/// Problem: N-Queens II
/// 
/// The N-Queens II problem asks for the number of distinct ways to place `n` queens on an `n x n` chessboard **such that no two queens attack each other**.
///
/// **Example 1:**
/// ```plaintext
/// Input: n = 4
/// Output: 2
/// Explanation: There are two distinct solutions.
/// ```
///
/// **Example 2:**
/// ```plaintext
/// Input: n = 1
/// Output: 1
/// ```
///
/// **Constraints:**
/// - `1 <= n <= 9`
///
/// # Solution:
/// - **Time Complexity:** `O(n!)`
/// - **Space Complexity:** `O(n)`
impl Solution {
    pub fn total_n_queens(n: i32) -> i32 {
        let mut count = 0;
        Self::backtrack(n as usize, 0, 0, 0, 0, &mut count);
        count
    }

    fn backtrack(n: usize, row: usize, cols: i32, diag1: i32, diag2: i32, count: &mut i32) {
        if row == n {
            *count += 1;
            return;
        }

        let mut available_positions = ((1 << n) - 1) & !(cols | diag1 | diag2);

        while available_positions != 0 {
            let position = available_positions & -available_positions;
            available_positions &= available_positions - 1;
            Self::backtrack(n, row + 1, cols | position, (diag1 | position) << 1, (diag2 | position) >> 1, count);
        }
    }
}