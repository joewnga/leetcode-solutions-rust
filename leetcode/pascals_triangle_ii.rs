/// 
/// Problem: Pascal's Triangle II
/// 
/// Given an integer rowIndex, return the rowIndex^th (0-indexed) row of Pascal's triangle.
/// 
/// In Pascal's triangle, each number is the sum of the two numbers directly above it.
/// 
/// Example 1:
/// Input: rowIndex = 3
/// Output: [1,3,3,1]
/// 
/// Example 2:
/// Input: rowIndex = 0
/// Output: [1]
/// 
/// Example 3:
/// Input: rowIndex = 1
/// Output: [1,1]
/// 
/// Constraints:
/// 0 <= rowIndex <= 33
/// 
/// Follow up: Could you optimize your algorithm to use only O(rowIndex) extra space?
/// 

// # Solution:
// Time complexity: O(rowIndex²) 
// Space complexity: O(rowIndex)

impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        let row_index = row_index as usize;
       let mut row = vec![1];
    
       for i in 0..row_index {
           let mut next_row = vec![1];
           
           
           for j in 0..row.len() - 1 {
               next_row.push(row[j] + row[j + 1]);
           }
           
           next_row.push(1);
           
           row = next_row;
       }
       
       row
    }
}