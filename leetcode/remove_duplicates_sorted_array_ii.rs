/// 
/// Problem: Remove Duplicates from Sorted Array II
/// 
/// Given an integer array `nums` sorted in **non-decreasing order**, remove some duplicates in-place.
/// 
/// Each element **must appear at most twice**.
/// 
/// Return the new length of `nums` after modifying it in-place.
///
/// **Example 1:**
/// ```plaintext
/// Input: nums = [1,1,1,2,2,3]
/// Output: 5, nums = [1,1,2,2,3,_]
/// Explanation: Keep at most 2 occurrences of each element.
/// ```
///
/// **Example 2:**
/// ```plaintext
/// Input: nums = [0,0,1,1,1,1,2,3,3]
/// Output: 7, nums = [0,0,1,1,2,3,3,_]
/// ```
///
/// **Constraints:**
/// - `1 <= nums.length <= 10^5`
/// - `-10^4 <= nums[i] <= 10^4`
/// - `nums` is sorted in **non-decreasing order**.
///
/// # Solution:
/// - **Time Complexity:** `O(n)`
/// - **Space Complexity:** `O(1)`
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.len() <= 2 {
            return nums.len() as i32;
        }

        let mut insert_pos = 2; 

        for i in 2..nums.len() {
            if nums[i] != nums[insert_pos - 2] {
                nums[insert_pos] = nums[i];
                insert_pos += 1;
            }
        }

        insert_pos as i32
    }
}