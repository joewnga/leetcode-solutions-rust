/// 
/// Problem: Set Matrix Zeroes
/// 
/// Given an `m x n` integer matrix, if an element is `0`, set its **entire row and column** to `0`.
/// 
/// You must do it **in place** with `O(1)` extra space.
///
/// **Example 1:**
/// ```plaintext
/// Input: matrix =
/// [
///   [1,1,1],
///   [1,0,1],
///   [1,1,1]
/// ]
/// 
/// Output:
/// [
///   [1,0,1],
///   [0,0,0],
///   [1,0,1]
/// ]
/// ```
///
/// **Example 2:**
/// ```plaintext
/// Input: matrix =
/// [
///   [0,1,2,0],
///   [3,4,5,2],
///   [1,3,1,5]
/// ]
/// 
/// Output:
/// [
///   [0,0,0,0],
///   [0,4,5,0],
///   [0,3,1,0]
/// ]
/// ```
///
/// **Constraints:**
/// - `m == matrix.length`
/// - `n == matrix[i].length`
/// - `1 <= m, n <= 200`
/// - `-2^31 <= matrix[i][j] <= 2^31 - 1`
///
/// # Solution:
/// - **Time Complexity:** `O(m * n)`
/// - **Space Complexity:** `O(1)`
impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let (m, n) = (matrix.len(), matrix[0].len());
        let mut first_row_has_zero = false;
        let mut first_col_has_zero = false;

        
        for i in 0..m {
            if matrix[i][0] == 0 {
                first_col_has_zero = true;
                break;
            }
        }
        for j in 0..n {
            if matrix[0][j] == 0 {
                first_row_has_zero = true;
                break;
            }
        }

        
        for i in 1..m {
            for j in 1..n {
                if matrix[i][j] == 0 {
                    matrix[i][0] = 0;
                    matrix[0][j] = 0;
                }
            }
        }

        
        for i in 1..m {
            for j in 1..n {
                if matrix[i][0] == 0 || matrix[0][j] == 0 {
                    matrix[i][j] = 0;
                }
            }
        }

        if first_col_has_zero {
            for i in 0..m {
                matrix[i][0] = 0;
            }
        }
        if first_row_has_zero {
            for j in 0..n {
                matrix[0][j] = 0;
            }
        }
    }
}