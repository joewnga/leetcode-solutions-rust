/// 
/// Problem: Maximum Subarray
/// 
/// Given an integer array `nums`, find the **contiguous subarray** (containing at least one number) 
/// which has the **largest sum** and return its sum.
///
/// **Example 1:**
/// ```plaintext
/// Input: nums = [-2,1,-3,4,-1,2,1,-5,4]
/// Output: 6
/// Explanation: [4,-1,2,1] has the largest sum.
/// ```
///
/// **Example 2:**
/// ```plaintext
/// Input: nums = [1]
/// Output: 1
/// ```
///
/// **Example 3:**
/// ```plaintext
/// Input: nums = [5,4,-1,7,8]
/// Output: 23
/// ```
///
/// **Constraints:**
/// - `1 <= nums.length <= 10^5`
/// - `-10^4 <= nums[i] <= 10^4`
///
/// # Solution:
/// - **Time Complexity:** `O(n)`
/// - **Space Complexity:** `O(1)`
impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut max_sum = nums[0];
        let mut current_sum = nums[0];

        for &num in nums.iter().skip(1) {
            current_sum = num.max(num + current_sum);
            max_sum = max_sum.max(current_sum);
        }

        max_sum
    }
}