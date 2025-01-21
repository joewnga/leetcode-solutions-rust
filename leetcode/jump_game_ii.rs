/// 
/// Problem: Jump Game II
/// 
/// Given an array `nums` where `nums[i]` represents the **maximum jump length** at that position, 
/// return the **minimum number of jumps** required to reach the last index.
///
/// **Example 1:**
/// ```plaintext
/// Input: nums = [2,3,1,1,4]
/// Output: 2
/// Explanation: Jump from index 0 to index 1, then to the last index.
/// ```
///
/// **Example 2:**
/// ```plaintext
/// Input: nums = [2,3,0,1,4]
/// Output: 2
/// ```
///
/// **Constraints:**
/// - `1 <= nums.length <= 10^4`
/// - `0 <= nums[i] <= 1000`
///
/// # Solution:
///
/// - **Time Complexity:** `O(n)`
/// - **Space Complexity:** `O(1)`
impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let (mut jumps, mut current_end, mut farthest) = (0, 0, 0);

        for i in 0..nums.len() - 1 {
            farthest = farthest.max(i + nums[i] as usize);

            if i == current_end {
                jumps += 1;
                current_end = farthest;

                if current_end >= nums.len() - 1 {
                    break;
                }
            }
        }

        jumps
    }
}