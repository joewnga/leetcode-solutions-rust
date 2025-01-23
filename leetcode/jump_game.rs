/// 
/// Problem: Jump Game
/// 
/// Given an array of integers `nums`, where `nums[i]` represents the **maximum jump length** at that position, 
/// determine whether you can reach the **last index** starting from index `0`.
///
/// **Example 1:**
/// ```plaintext
/// Input: nums = [2,3,1,1,4]
/// Output: true
/// Explanation: Jump from index 0 to index 1, then to the last index.
/// ```
///
/// **Example 2:**
/// ```plaintext
/// Input: nums = [3,2,1,0,4]
/// Output: false
/// Explanation: No matter what, index `4` cannot be reached.
/// ```
///
/// **Constraints:**
/// - `1 <= nums.length <= 10^4`
/// - `0 <= nums[i] <= 10^5`
///
/// # Solution:
/// - **Time Complexity:** `O(n)`
/// - **Space Complexity:** `O(1)`
impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut max_reach = 0;
        
        for i in 0..nums.len() {
            if i > max_reach {
                return false;
            }
            max_reach = max_reach.max(i + nums[i] as usize);
            if max_reach >= nums.len() - 1 {
                return true;
            }
        }
        
        true
    }
}
