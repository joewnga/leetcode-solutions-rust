/// 
/// Problem: Search in Rotated Sorted Array II
/// 
/// You are given an integer array `nums` sorted in **ascending order** and rotated at some pivot index.
/// 
/// Given a target value `target`, return **`true` if `target` exists in `nums`, otherwise return `false`.**
///
/// **Unlike `Search in Rotated Sorted Array I`, this array may contain **duplicates**.**
///
/// **Example 1:**
/// ```plaintext
/// Input: nums = [2,5,6,0,0,1,2], target = 0
/// Output: true
/// ```
///
/// **Example 2:**
/// ```plaintext
/// Input: nums = [2,5,6,0,0,1,2], target = 3
/// Output: false
/// ```
///
/// **Constraints:**
/// - `1 <= nums.length <= 5000`
/// - `-10^4 <= nums[i] <= 10^4`
/// - `nums` is sorted and rotated at an unknown pivot.`
///
/// # Solution:
/// - **Time Complexity:** `O(log n)`
/// - **Space Complexity:** `O(1)`
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> bool {
        if nums.is_empty() {
            return false;
        }

        let (mut left, mut right) = (0, nums.len() as isize - 1);

        while left <= right {
            let mid = left + (right - left) / 2;

            if nums[mid as usize] == target {
                return true;
            }

            
            if nums[left as usize] == nums[mid as usize] && nums[mid as usize] == nums[right as usize] {
                left += 1;
                right -= 1;
                continue;
            }

            
            if nums[left as usize] <= nums[mid as usize] {
                if nums[left as usize] <= target && target < nums[mid as usize] {
                    right = mid - 1;
                } else {
                    left = mid + 1;
                }
            } 
            
            else {
                if nums[mid as usize] < target && target <= nums[right as usize] {
                    left = mid + 1;
                } else {
                    right = mid - 1;
                }
            }
        }

        false
    }
}