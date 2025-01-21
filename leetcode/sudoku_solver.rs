/// 
/// Problem: Sudoku Solver
/// 
/// Write a program to solve a `9x9` Sudoku puzzle by filling the empty cells (`'.'`).
///
/// The solution must **follow Sudoku rules**:
/// - Each row must contain the digits `1-9` without repetition.
/// - Each column must contain the digits `1-9` without repetition.
/// - Each `3x3` sub-box must contain the digits `1-9` without repetition.
///
/// Example:
/// ```plaintext
/// Input:
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
///
/// **Output (solved Sudoku board):**
/// ```plaintext
/// [
///   ["5","3","4","6","7","8","9","1","2"],
///   ["6","7","2","1","9","5","3","4","8"],
///   ["1","9","8","3","4","2","5","6","7"],
///   ["8","5","9","7","6","1","4","2","3"],
///   ["4","2","6","8","5","3","7","9","1"],
///   ["7","1","3","9","2","4","8","5","6"],
///   ["9","6","1","5","3","7","2","8","4"],
///   ["2","8","7","4","1","9","6","3","5"],
///   ["3","4","5","2","8","6","1","7","9"]
/// ]
/// ```
///
/// Constraints:
/// - `board` is always `9x9`.
/// - The input board will be **partially filled**.
/// - There is **exactly one solution**.
///
/// # Solution:
///
/// - **Time Complexity:** `O(9^(n))`
/// - **Space Complexity:** `O(1)`

impl Solution {
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        
        Self::backtrack(board);
    }

    fn backtrack(board: &mut Vec<Vec<char>>) -> bool {
        for row in 0..9 {
            for col in 0..9 {
                if board[row][col] == '.' {
                    for num in '1'..='9' {
                        if Self::is_valid(board, row, col, num) {
                            board[row][col] = num;
                            if Self::backtrack(board) {
                                return true;
                            }
                            board[row][col] = '.'; 
                        }
                    }
                    return false; 
                }
            }
        }
        true 
    }

    fn is_valid(board: &Vec<Vec<char>>, row: usize, col: usize, num: char) -> bool {
        let box_index = (row / 3) * 3 + col / 3;

        for i in 0..9 {
            if board[row][i] == num || board[i][col] == num || 
               board[(box_index / 3) * 3 + i / 3][(box_index % 3) * 3 + i % 3] == num {
                return false;
            }
        }

        true
    }
}