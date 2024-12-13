/// 
/// Problem: 3Sum Closest
/// 
/// Given an integer array nums of length n and an integer target, find three integers in nums such that the sum is closest to target.
/// 
/// Return the sum of the three integers.
/// 
/// You may assume that each input would have exactly one solution.
/// 
/// Example 1:
/// 
/// Input: nums = [-1,2,1,-4], target = 1
/// Output: 2
/// Explanation: The sum that is closest to the target is 2. (-1 + 2 + 1 = 2).
/// 
/// # Solution 1:
/// Time complexity: 0(n^2)
/// Space complexity: 0(1)

impl Solution {
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        let mut nums = nums;
        nums.sort();
        let mut closest = i32::MAX;
        for i in 0..nums.len() {
            let mut l = i + 1;
            let mut r = nums.len() - 1;
            while l < r {
                let sum = nums[i] + nums[l] + nums[r];
                if (sum - target).abs() < (closest - target).abs() {
                    closest = sum;
                }
                if sum < target {
                    l += 1;
                } else if sum > target {
                    r -= 1;
                } else {
                    return sum;
                }
            }
        }
        closest
    }
}