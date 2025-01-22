/// 
/// Problem: Rotate Image
/// 
/// Given an `n x n` 2D matrix representing an image, rotate the image **90 degrees clockwise** in-place.
///
/// **Example 1:**
/// ```plaintext
/// Input:
/// [
///   [1,2,3],
///   [4,5,6],
///   [7,8,9]
/// ]
/// Output:
/// [
///   [7,4,1],
///   [8,5,2],
///   [9,6,3]
/// ]
/// ```
///
/// **Example 2:**
/// ```plaintext
/// Input:
/// [
///   [5,1,9,11],
///   [2,4,8,10],
///   [13,3,6,7],
///   [15,14,12,16]
/// ]
/// Output:
/// [
///   [15,13,2,5],
///   [14,3,4,1],
///   [12,6,8,9],
///   [16,7,10,11]
/// ]
/// ```
///
/// **Constraints:**
/// - `n == matrix.length == matrix[i].length`
/// - `1 <= n <= 20`
/// - `-1000 <= matrix[i][j] <= 1000`
///
/// # Solution:
///
/// - **Time Complexity:** `O(n²)`
/// - **Space Complexity:** `O(1)`
impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let n = matrix.len();

        for i in 0..n {
            for j in i + 1..n {
                let temp = matrix[i][j];
                matrix[i][j] = matrix[j][i];
                matrix[j][i] = temp;
            }
        }

        for row in matrix.iter_mut() {
            row.reverse();
        }
        
    }
}