/// 
/// Problem: Spiral Matrix
/// 
/// Given an `m x n` matrix, return all elements of the matrix in **spiral order**.
///
/// **Example 1:**
/// ```plaintext
/// Input: matrix = [[1,2,3],[4,5,6],[7,8,9]]
/// Output: [1,2,3,6,9,8,7,4,5]
/// ```
///
/// **Example 2:**
/// ```plaintext
/// Input: matrix = [[1,2,3,4],[5,6,7,8],[9,10,11,12]]
/// Output: [1,2,3,4,8,12,11,10,9,5,6,7]
/// ```
///
/// **Constraints:**
/// - `m == matrix.length`
/// - `n == matrix[i].length`
/// - `1 <= m, n <= 10`
/// - `-100 <= matrix[i][j] <= 100`
///
/// # Solution:
/// - **Time Complexity:** `O(m * n)`
/// - **Space Complexity:** `O(1)`
impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let mut result = Vec::new();
        if matrix.is_empty() {
            return result;
        }

        let (mut top, mut bottom) = (0, matrix.len() as isize - 1);
        let (mut left, mut right) = (0, matrix[0].len() as isize - 1);

        while top <= bottom && left <= right {
            for i in left..=right {
                result.push(matrix[top as usize][i as usize]);
            }
            top += 1;

            for i in top..=bottom {
                result.push(matrix[i as usize][right as usize]);
            }
            right -= 1;

            if top <= bottom {
                for i in (left..=right).rev() {
                    result.push(matrix[bottom as usize][i as usize]);
                }
                bottom -= 1;
            }

            if left <= right {
                for i in (top..=bottom).rev() {
                    result.push(matrix[i as usize][left as usize]);
                }
                left += 1;
            }
        }

        result
    }
}