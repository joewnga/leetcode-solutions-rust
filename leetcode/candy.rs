/// 
/// Problem: Candy
/// 
/// There are n children standing in a line. Each child is assigned a rating value given in the 
/// integer array ratings.
/// 
/// You are giving candies to these children subjected to the following requirements:
/// - Each child must have at least one candy.
/// - Children with a higher rating get more candies than their neighbors.
/// 
/// Return the minimum number of candies you need to have to distribute the candies to the children.
/// 
/// Example 1:
/// Input: ratings = [1,0,2]
/// Output: 5
/// Explanation: You can allocate to the first, second and third child with 2, 1, 2 candies respectively.
/// 
/// Example 2:
/// Input: ratings = [1,2,2]
/// Output: 4
/// Explanation: You can allocate to the first, second and third child with 1, 2, 1 candies respectively.
/// The third child gets 1 candy because it satisfies the above two conditions.
/// 
/// Constraints:
/// n == ratings.length
/// 1 <= n <= 2 * 10^4
/// 0 <= ratings[i] <= 2 * 10^4
/// 

// # Solution 
// Time complexity: O(n) 
// Space complexity: O(1)

impl Solution {
    pub fn candy(ratings: Vec<i32>) -> i32 {
        let n = ratings.len();
       
       if n <= 1 {
           return n as i32;
       }
       
       let mut candies = 0;
       let mut up = 0;
       let mut down = 0;
       let mut old_slope = 0;
       
       for i in 1..n {
           // Calculate the current slope
           let new_slope = if ratings[i] > ratings[i - 1] {
               1
           } else if ratings[i] < ratings[i - 1] {
               -1
           } else {
               0
           };
           
           // If we were going up and now we're going down (peak)
           if (old_slope > 0 && new_slope == 0) || (old_slope < 0 && new_slope >= 0) {
               
               candies += Self::count(up) + Self::count(down) + std::cmp::max(up, down);
               up = 0;
               down = 0;
           }
           
           // Update up, down counters
           if new_slope > 0 {
               up += 1;
           } else if new_slope < 0 {
               down += 1;
           } else {
               
               candies += 1;
           }
           
           old_slope = new_slope;
       }
       
       
       candies += Self::count(up) + Self::count(down) + std::cmp::max(up, down) + 1;
       
       candies
   }
   
   
   fn count(n: i32) -> i32 {
       (n * (n + 1)) / 2
    }
}