/// 
/// Problem: Triangle
/// 
/// Given a triangle array, return the minimum path sum from top to bottom.
/// 
/// For each step, you may move to an adjacent number of the row below. More formally, 
/// if you are on index i on the current row, you may move to either index i or index i + 1 
/// on the next row.
/// 
/// Example 1:
/// Input: triangle = [[2],[3,4],[6,5,7],[4,1,8,3]]
/// Output: 11
/// Explanation: The triangle looks like:
///    2
///   3 4
///  6 5 7
/// 4 1 8 3
/// The minimum path sum from top to bottom is 2 + 3 + 5 + 1 = 11.
/// 
/// Example 2:
/// Input: triangle = [[-10]]
/// Output: -10
/// 
/// Constraints:
/// 1 <= triangle.length <= 200
/// triangle[0].length == 1
/// triangle[i].length == triangle[i - 1].length + 1
/// -10^4 <= triangle[i][j] <= 10^4
/// 
/// Follow up: Could you do this using only O(n) extra space, where n is the total number
/// of rows in the triangle?
/// 

// # Solution 1
// Time complexity: O(n²) 
// Space complexity: O(n²) 


impl Solution {
    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        let n = triangle.len();
       
       let mut dp = vec![vec![0; n]; n];
       
       for j in 0..n {
           dp[n - 1][j] = triangle[n - 1][j];
       }
       
       for i in (0..n-1).rev() {
           for j in 0..=i {
            
               dp[i][j] = triangle[i][j] + std::cmp::min(dp[i + 1][j], dp[i + 1][j + 1]);
           }
       }
       
       dp[0][0]
    }
}


// # Solution 2:
// Time complexity: O(n²) 
// Space complexity: O(n)

impl Solution {
    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
       let n = triangle.len();
       
       let mut dp = triangle[n - 1].clone();
       
       for i in (0..n-1).rev() {
           for j in 0..=i {
               
               dp[j] = triangle[i][j] + std::cmp::min(dp[j], dp[j + 1]);
           }
       }
       
       dp[0]
    }
}