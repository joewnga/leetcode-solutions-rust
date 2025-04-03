/// 
/// Problem: Surrounded Regions
/// 
/// Given an m x n matrix board containing 'X' and 'O', capture all regions that are 4-directionally 
/// surrounded by 'X'.
/// 
/// A region is captured by flipping all 'O's into 'X's in that surrounded region.
/// 
/// Example 1:
/// Input: board = [["X","X","X","X"],["X","O","O","X"],["X","X","O","X"],["X","O","X","X"]]
/// Output: [["X","X","X","X"],["X","X","X","X"],["X","X","X","X"],["X","O","X","X"]]
/// Explanation: Notice that an 'O' should not be flipped if:
/// - It is on the border, or
/// - It is adjacent to an 'O' that should not be flipped.
/// The bottom 'O' is on the border, so it is not flipped.
/// The other three 'O' form a surrounded region, so they are flipped.
/// 
/// Example 2:
/// Input: board = [["X"]]
/// Output: [["X"]]
/// 
/// Constraints:
/// m == board.length
/// n == board[i].length
/// 1 <= m, n <= 200
/// board[i][j] is 'X' or 'O'.
/// 

// # Solution
// Time complexity: O(m*n) 
// Space complexity: O(m*n)

impl Solution {
    pub fn solve(board: &mut Vec<Vec<char>>) {
        if board.is_empty() || board[0].is_empty() {
           return;
       }
       
       let rows = board.len();
       let cols = board[0].len();
       
       
       for col in 0..cols {
           Self::dfs(board, 0, col as i32, rows as i32, cols as i32);
           Self::dfs(board, (rows - 1) as i32, col as i32, rows as i32, cols as i32);
       }
       
       for row in 0..rows {
           Self::dfs(board, row as i32, 0, rows as i32, cols as i32);
           Self::dfs(board, row as i32, (cols - 1) as i32, rows as i32, cols as i32);
       }
       
       for row in 0..rows {
           for col in 0..cols {
               if board[row][col] == 'O' {
                   board[row][col] = 'X'; 
               } else if board[row][col] == 'E' {
                   board[row][col] = 'O'; 
               }
           }
       }
   }
   
   fn dfs(board: &mut Vec<Vec<char>>, row: i32, col: i32, rows: i32, cols: i32) {
       if row < 0 || row >= rows || col < 0 || col >= cols || board[row as usize][col as usize] != 'O' {
           return;
       }
       
       board[row as usize][col as usize] = 'E';
       
       Self::dfs(board, row + 1, col, rows, cols);
       Self::dfs(board, row - 1, col, rows, cols);
       Self::dfs(board, row, col + 1, rows, cols);
       Self::dfs(board, row, col - 1, rows, cols);
    }
}