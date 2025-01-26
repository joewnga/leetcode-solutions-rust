/// 
/// Problem: Search a 2D Matrix
/// 
/// You are given an `m x n` matrix where:
/// - Each row is **sorted in ascending order**.
/// - The first integer of each row is **greater than the last integer of the previous row**.
///
/// Given an integer `target`, return **`true` if `target` exists in the matrix, otherwise return `false`.**
///
/// **Example 1:**
/// ```plaintext
/// Input: matrix =
/// [
///   [1, 3, 5, 7],
///   [10, 11, 16, 20],
///   [23, 30, 34, 60]
/// ]
/// target = 3
/// Output: true
/// ```
///
/// **Example 2:**
/// ```plaintext
/// Input: matrix =
/// [
///   [1, 3, 5, 7],
///   [10, 11, 16, 20],
///   [23, 30, 34, 60]
/// ]
/// target = 13
/// Output: false
/// ```
///
/// **Constraints:**
/// - `m == matrix.length`
/// - `n == matrix[i].length`
/// - `1 <= m, n <= 100`
/// - `-10^4 <= matrix[i][j], target <= 10^4`
///
/// # Solution:
/// - **Time Complexity:** `O(m * n)`
/// - **Space Complexity:** `O(1)`
impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        if matrix.is_empty() || matrix[0].is_empty() {
            return false;
        }

        let (m, n) = (matrix.len(), matrix[0].len());
        let (mut left, mut right) = (0, m * n - 1);

        while left <= right {
            let mid = left + (right - left) / 2;
            let row = mid / n;
            let col = mid % n;

            let mid_value = matrix[row][col];

            if mid_value == target {
                return true;
            } else if mid_value < target {
                left = mid + 1; 
            } else {
                if mid == 0 { break; } 
                right = mid - 1; 
            }
        }

        false
    }
}