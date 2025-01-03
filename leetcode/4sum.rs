/// 
/// Problem: 4Sum
/// 
/// Given an array `nums` of `n` integers and an integer `target`, return all the unique quadruplets [nums[a], nums[b], nums[c], nums[d]] such that:
/// - 0 <= a, b, c, d < n
/// - a, b, c, and d are distinct indices.
/// - nums[a] + nums[b] + nums[c] + nums[d] == target
/// 
/// You may return the answer in any order.
/// 
/// Example 1:
/// Input: nums = [1,0,-1,0,-2,2], target = 0
/// Output: [[-2,-1,1,2],[-2,0,0,2],[-1,0,0,1]]
/// 
/// Example 2:
/// Input: nums = [2,2,2,2,2], target = 8
/// Output: [[2,2,2,2]]
/// 
/// # Solution:
///  
/// Time complexity: O(n^3)
/// Space complexity: O(1) 

impl Solution {
    pub fn four_sum(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        nums.sort(); 
        let mut result = Vec::new();
        let n = nums.len();

        for i in 0..n {
            if i > 0 && nums[i] == nums[i - 1] {
                continue; 
            }

            for j in i + 1..n {
                if j > i + 1 && nums[j] == nums[j - 1] {
                    continue; 
                }

                let mut left = j + 1; 
                let mut right = n - 1; 

                while left < right {
                    let sum = nums[i] + nums[j] + nums[left] + nums[right];

                    if sum == target {
                        result.push(vec![nums[i], nums[j], nums[left], nums[right]]);

                        
                        while left < right && nums[left] == nums[left + 1] {
                            left += 1;
                        }

                        
                        while left < right && nums[right] == nums[right - 1] {
                            right -= 1;
                        }

                        left += 1;
                        right -= 1;
                    } else if sum < target {
                        left += 1;
                    } else {
                        right -= 1; 
                    }
                }
            }
        }

        result
    }
}
