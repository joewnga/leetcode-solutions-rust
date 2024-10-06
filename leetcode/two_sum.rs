
///
///     Problem:
/// 
///     Given an array of integers `nums` and an integer `target`, return indices of the two numbers such that they add up to `target`.
/// 
///     You may assume that each input would have exactly one solution, and you may not use the same element twice.
/// 
///     You can return the answer in any order.
/// 
/// 
///     Example 1:
///     
///     Input: nums = [2,7,11,15], target = 9
///     Output: [0,1]
///     Explanation: Because nums[0] + nums[1] == 9, we return [0, 1].
/// 
/// 
/// 

/// Brute Force Approach:
/// Time complexity: O(n^2)
/// Space complexity: O(1)
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {

        for i in 0..nums.len(){
            for j in i + 1..nums.len(){
                if nums[i] + nums[j] == target{
                    return vec![i as i32, j as i32]
                }
            }
        }
        vec![]
        
    }
}


/// Optimal Solution:
/// Time complexity: O(n)
/// Space complexity: O(n)
use std::collections::HashMap;

impl Solution {
    
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {

        let mut map = HashMap::new();

       for (i,&num) in nums.iter().enumerate(){
        let complement = target - num;
        if let Some(&j) = map.get(&complement){
            return vec![i as i32, j as i32];
        }
        map.insert(num, i);
       }
        vec![]
        
    }
}


