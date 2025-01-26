/// 
/// Problem: Word Search
/// 
/// Given an `m x n` grid of characters (`board`) and a string `word`, return **`true`** if `word` exists in the grid.
/// 
/// The word can be constructed **from letters of sequentially adjacent cells**, where:
/// - Cells are adjacent **horizontally or vertically**.
/// - The same letter cell **cannot be used more than once per search**.
///
/// **Example 1:**
/// ```plaintext
/// Input:
/// board =
/// [
///   ['A','B','C','E'],
///   ['S','F','C','S'],
///   ['A','D','E','E']
/// ]
/// word = "ABCCED"
/// Output: true
/// ```
///
/// **Example 2:**
/// ```plaintext
/// Input:
/// board =
/// [
///   ['A','B','C','E'],
///   ['S','F','C','S'],
///   ['A','D','E','E']
/// ]
/// word = "SEE"
/// Output: true
/// ```
///
/// **Example 3:**
/// ```plaintext
/// Input:
/// board =
/// [
///   ['A','B','C','E'],
///   ['S','F','C','S'],
///   ['A','D','E','E']
/// ]
/// word = "ABCB"
/// Output: false
/// ```
///
/// **Constraints:**
/// - `1 <= board.length, board[i].length <= 6`
/// - `1 <= word.length <= 15`
/// - `board` and `word` consist only of lowercase and uppercase English letters.
///
/// # Solution:
/// - **Time Complexity:** ``O(m * n * 4^L)`
/// - **Space Complexity:** `O(L)`, Recursion depth is L, the word length)
impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let (rows, cols) = (board.len(), board[0].len());
        let word_chars: Vec<char> = word.chars().collect();
        let mut board = board;

        for i in 0..rows {
            for j in 0..cols {
                if Self::dfs(&mut board, &word_chars, 0, i as isize, j as isize) {
                    return true;
                }
            }
        }

        false
    }

    fn dfs(board: &mut Vec<Vec<char>>, word: &[char], index: usize, row: isize, col: isize) -> bool {
        if index == word.len() {
            return true;
        }

        if row < 0 || row >= board.len() as isize || col < 0 || col >= board[0].len() as isize {
            return false;
        }

        if board[row as usize][col as usize] != word[index] {
            return false;
        }

        let temp = board[row as usize][col as usize];
        board[row as usize][col as usize] = '#'; 

        let found = Self::dfs(board, word, index + 1, row + 1, col)
            || Self::dfs(board, word, index + 1, row - 1, col)
            || Self::dfs(board, word, index + 1, row, col + 1)
            || Self::dfs(board, word, index + 1, row, col - 1);

        board[row as usize][col as usize] = temp; 

        found
    }
}