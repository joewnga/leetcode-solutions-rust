/// 
/// # Problem
/// 
/// The string "PAYPALISHIRING" is written in a zigzag pattern on a given number of rows like this:
/// (you may want to display this pattern in a fixed font for better legibility)
/// 
/// P   A   H   N
/// 
/// A P L S I I G
/// 
/// Y   I   R
/// 
/// And then read line by line: "PAHNAPLSIIGYIR"
/// 
/// Write the code that will take a string and make this conversion given a number of rows:
/// 
/// string convert(string s, int numRows);

/// # Solution 1:
/// Time complexity: O(n)
/// Space complexity: O(n)
impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        let num_rows = num_rows as usize;
        if num_rows == 1 {
            return s;
        }

        (0..num_rows)
            .chain((1..num_rows - 1).rev())
            .cycle()
            .zip(s.chars())
            .fold(vec![Vec::new(); num_rows], |mut diags, (row, c)| {
                diags[row].push(c);
                diags
            })
            .into_iter()
            .flat_map(|row| row.into_iter())
            .collect()
        
    }
}
