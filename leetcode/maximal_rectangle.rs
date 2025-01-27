/// 
/// Problem: Maximal Rectangle
/// 
/// Given a `rows x cols` binary matrix filled with `0s` and `1s`, find the area of the **largest rectangle** containing only `1s`.
///
/// **Example 1:**
/// ```plaintext
/// Input:
/// matrix =
/// [
///   ['1','0','1','0','0'],
///   ['1','0','1','1','1'],
///   ['1','1','1','1','1'],
///   ['1','0','0','1','0']
/// ]
/// Output: 6
/// Explanation: The largest rectangle is formed using `[1,1,1]` in row 2 and `[1,1,1]` in row 3 (area = `3*2=6`).
/// ```
///
/// **Example 2:**
/// ```plaintext
/// Input:
/// matrix = [
///   ['0']
/// ]
/// Output: 0
/// ```
///
/// **Constraints:**
/// - `1 <= rows, cols <= 200`
/// - `matrix[i][j]` is either `'0'` or `'1'`.
///
/// # Solution:
/// - **Time Complexity:** `O(rows * cols)`
/// - **Space Complexity:** `O(cols)`
impl Solution {
    pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
        if matrix.is_empty() || matrix[0].is_empty() {
            return 0;
        }

        let rows = matrix.len();
        let cols = matrix[0].len();
        let mut heights = vec![0; cols];
        let mut max_area = 0;

        for row in 0..rows {
            for col in 0..cols {
                if matrix[row][col] == '1' {
                    heights[col] += 1;
                } else {
                    heights[col] = 0;
                }
            }
            max_area = max_area.max(Self::largest_histogram(&heights));
        }

        max_area
    }

    fn largest_histogram(heights: &Vec<i32>) -> i32 {
        let mut stack: Vec<usize> = Vec::new();
        let mut max_area = 0;
        let mut heights = heights.clone();
        heights.push(0); 

        for i in 0..heights.len() {
            while let Some(&top) = stack.last() {
                if heights[i] >= heights[top] {
                    break;
                }
                stack.pop();
                let height = heights[top];
                let width = if stack.is_empty() { i } else { i - stack.last().unwrap() - 1 };
                max_area = max_area.max(height * width as i32);
            }
            stack.push(i);
        }

        max_area
    }
}