/// 
/// Problem: Pascal's Triangle
/// 
/// Given an integer numRows, return the first numRows of Pascal's triangle.
/// 
/// In Pascal's triangle, each number is the sum of the two numbers directly above it.
/// 
/// Example 1:
/// Input: numRows = 5
/// Output: [[1],[1,1],[1,2,1],[1,3,3,1],[1,4,6,4,1]]
/// 
/// Example 2:
/// Input: numRows = 1
/// Output: [[1]]
/// 
/// Constraints:
/// 1 <= numRows <= 30
/// 

// # Solution:
// Time complexity: O(numRows²) 
// Space complexity: O(numRows²) 

impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let num_rows = num_rows as usize;
       let mut result = Vec::with_capacity(num_rows);
       
       
       result.push(vec![1]);
       
       
       for i in 1..num_rows {
           let prev_row = &result[i - 1];
           let mut current_row = Vec::with_capacity(i + 1);
           
           
           current_row.push(1);
           
           
           for j in 1..i {
               current_row.push(prev_row[j - 1] + prev_row[j]);
           }
           
           
           current_row.push(1);
           
           result.push(current_row);
       }
       
       result
    }
}