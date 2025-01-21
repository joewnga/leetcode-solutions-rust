/// 
/// Problem: First Missing Positive
/// 
/// Given an unsorted integer array `nums`, return the **smallest missing positive integer**.
///
/// **Example 1:**
/// ```plaintext
/// Input: nums = [1,2,0]
/// Output: 3
/// ```
///
/// **Example 2:**
/// ```plaintext
/// Input: nums = [3,4,-1,1]
/// Output: 2
/// ```
///
/// **Example 3:**
/// ```plaintext
/// Input: nums = [7,8,9,11,12]
/// Output: 1
/// ```
///
/// **Constraints:**
/// - `1 <= nums.length <= 10^5`
/// - `-2^31 <= nums[i] <= 2^31 - 1`
///
/// # Solution:
///
/// **Time Complexity:** `O(n)`
/// **Space Complexity:** `O(1)`
impl Solution {
    pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        let n = nums.len();

        for i in 0..n {
            while nums[i] > 0 && nums[i] <= n as i32 && nums[i] != nums[nums[i] as usize - 1] {
                let target_index = nums[i] as usize - 1;
                nums.swap(i, target_index);
            }
        }

        for i in 0..n {
            if nums[i] != (i + 1) as i32 {
                return (i + 1) as i32;
            }
        }

        (n + 1) as i32
        
    }
}