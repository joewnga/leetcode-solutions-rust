/// 
/// Problem: Longest Consecutive Sequence
/// 
/// Given an unsorted array of integers nums, return the length of the longest consecutive 
/// elements sequence.
/// 
/// You must write an algorithm that runs in O(n) time.
/// 
/// Example 1:
/// Input: nums = [100,4,200,1,3,2]
/// Output: 4
/// Explanation: The longest consecutive elements sequence is [1, 2, 3, 4]. Therefore its length is 4.
/// 
/// Example 2:
/// Input: nums = [0,3,7,2,5,8,4,6,0,1]
/// Output: 9
/// 
/// Constraints:
/// 0 <= nums.length <= 10^5
/// -10^9 <= nums[i] <= 10^9
/// 

// # Solution
// Time complexity: O(n) 
// Space complexity: O(n) 

use std::collections::HashSet;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
           return 0;
       }
       
       let num_set: HashSet<i32> = nums.into_iter().collect();
       
       let mut max_length = 0;
       
       for &num in &num_set {
           if !num_set.contains(&(num - 1)) {
               let mut current_num = num;
               let mut current_length = 1;
               
               while num_set.contains(&(current_num + 1)) {
                   current_num += 1;
                   current_length += 1;
               }
               
               max_length = max_length.max(current_length);
           }
       }
       
       max_length
    }
}