/// 
/// Problem: Spiral Matrix II
/// 
/// Given an integer `n`, generate an `n x n` matrix filled with elements from `1` to `n²` in **spiral order**.
///
/// **Example 1:**
/// ```plaintext
/// Input: n = 3
/// Output:
/// [
///  [1, 2, 3],
///  [8, 9, 4],
///  [7, 6, 5]
/// ]
/// ```
///
/// **Example 2:**
/// ```plaintext
/// Input: n = 1
/// Output: [[1]]
/// ```
///
/// **Constraints:**
/// - `1 <= n <= 20`
///
/// # Solution:
/// - **Time Complexity:** `O(n²)`
/// - **Space Complexity:** `O(1)`

impl Solution {
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        let mut matrix = vec![vec![0; n as usize]; n as usize];
        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        let (mut row, mut col, mut dir_index) = (0, 0, 0);
        let mut num = 1;

        for _ in 0..n * n {
            matrix[row][col] = num;
            num += 1;

            let next_row = row as isize + directions[dir_index].0;
            let next_col = col as isize + directions[dir_index].1;

            if next_row < 0 || next_row >= n as isize || next_col < 0 || next_col >= n as isize || matrix[next_row as usize][next_col as usize] != 0 {
                dir_index = (dir_index + 1) % 4;
            }

            row = (row as isize + directions[dir_index].0) as usize;
            col = (col as isize + directions[dir_index].1) as usize;
        }

        matrix
        
    }
}