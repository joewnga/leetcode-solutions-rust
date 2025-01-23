/// 
/// Problem: N-Queens
/// 
/// The N-Queens problem involves placing `n` queens on an `n x n` chessboard such that:
/// - No two queens threaten each other.
/// - Queens cannot share the same row, column, or diagonal.
///
/// **Example 1:**
/// ```plaintext
/// Input: n = 4
/// Output:
/// [[".Q..",  // Solution 1
///   "...Q",
///   "Q...",
///   "..Q."],
///
///  ["..Q.",  // Solution 2
///   "Q...",
///   "...Q",
///   ".Q.."]]
/// ```
///
/// **Example 2:**
/// ```plaintext
/// Input: n = 1
/// Output: [["Q"]]
/// ```
///
/// **Constraints:**
/// - `1 <= n <= 9`
///
/// # Solution:
/// - **Time Complexity:** `O(n!)`
/// - **Space Complexity:** `O(n)`
use std::collections::HashSet;

impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        let mut result = Vec::new();
        let mut board = vec![vec!['.'; n as usize]; n as usize];
        let mut cols = HashSet::new();
        let mut diag1 = HashSet::new();
        let mut diag2 = HashSet::new();

        Self::backtrack(0, n as usize, &mut board, &mut cols, &mut diag1, &mut diag2, &mut result);
        result
    }

    fn backtrack(
        row: usize,
        n: usize,
        board: &mut Vec<Vec<char>>,
        cols: &mut HashSet<usize>,
        diag1: &mut HashSet<i32>,
        diag2: &mut HashSet<i32>,
        result: &mut Vec<Vec<String>>,
    ) {
        if row == n {
            result.push(board.iter().map(|r| r.iter().collect()).collect());
            return;
        }

        for col in 0..n {
            let d1 = row as i32 - col as i32;
            let d2 = row as i32 + col as i32;
            if cols.contains(&col) || diag1.contains(&d1) || diag2.contains(&d2) {
                continue;
            }

            board[row][col] = 'Q';
            cols.insert(col);
            diag1.insert(d1);
            diag2.insert(d2);

            Self::backtrack(row + 1, n, board, cols, diag1, diag2, result);

            board[row][col] = '.';
            cols.remove(&col);
            diag1.remove(&d1);
            diag2.remove(&d2);
        }
    }
}