/// 
/// Problem: Valid Sudoku
/// 
/// Determine if a `9x9` Sudoku board is **valid**.
/// Only filled cells need to be validated.
///
/// A valid Sudoku board:
/// - Each **row** contains digits `1-9` without repetition.
/// - Each **column** contains digits `1-9` without repetition.
/// - Each of the **9 sub-boxes** (`3x3` grids) contains digits `1-9` without repetition.
///
/// Note:
/// - The Sudoku board may be partially filled.
/// - The board does not need to be solvable, only valid.
///
/// Example 1:
/// Input:
/// ```plaintext
/// [
///   ["5","3",".",".","7",".",".",".","."],
///   ["6",".",".","1","9","5",".",".","."],
///   [".","9","8",".",".",".",".","6","."],
///   ["8",".",".",".","6",".",".",".","3"],
///   ["4",".",".","8",".","3",".",".","1"],
///   ["7",".",".",".","2",".",".",".","6"],
///   [".","6",".",".",".",".","2","8","."],
///   [".",".",".","4","1","9",".",".","5"],
///   [".",".",".",".","8",".",".","7","9"]
/// ]
/// ```
/// Output: `true`
///
/// Constraints:
/// - `board` is always `9x9`.
/// - Each cell is `'.'` (empty) or a digit `'1'-'9'`.
///
/// # Solution:
///
/// **Time Complexity:** `O(1)`
/// **Space Complexity:** `O(1)`

use std::collections::HashSet;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
         let mut rows = vec![HashSet::new(); 9];
        let mut cols = vec![HashSet::new(); 9];
        let mut boxes = vec![HashSet::new(); 9];

        for row in 0..9 {
            for col in 0..9 {
                let cell = board[row][col];
                if cell == '.' {
                    continue;
                }

                let box_index = (row / 3) * 3 + col / 3;

                if rows[row].contains(&cell)
                    || cols[col].contains(&cell)
                    || boxes[box_index].contains(&cell)
                {
                    return false;
                }

                rows[row].insert(cell);
                cols[col].insert(cell);
                boxes[box_index].insert(cell);
            }
        }

        true
        
    }
}