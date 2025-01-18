/// 
/// Problem: Search in Rotated Sorted Array
/// 
/// Suppose an array sorted in ascending order is rotated at some pivot unknown to you beforehand.
/// Given the rotated sorted array `nums` and an integer `target`, return the index of `target` if it is in `nums`, otherwise return `-1`.
///
/// You must write an algorithm with O(log n) runtime complexity.
///
/// Example 1:
/// Input: nums = [4,5,6,7,0,1,2], target = 0
/// Output: 4
///
/// Example 2:
/// Input: nums = [4,5,6,7,0,1,2], target = 3
/// Output: -1
///
/// Example 3:
/// Input: nums = [1], target = 0
/// Output: -1
///
/// Constraints:
/// - `1 <= nums.length <= 5000`
/// - `-10^4 <= nums[i] <= 10^4`
/// - All values are **unique**.
/// - `nums` is **sorted** in ascending order and rotated.
/// - `-10^4 <= target <= 10^4`
///
/// 
/// Time Complexity: O(log n)
/// Space Complexity: O(1)
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let (mut left, mut right) = (0, nums.len() as isize - 1);

        while left <= right {
            let mid = left + (right - left) / 2;

            if nums[mid as usize] == target {
                return mid as i32;
            }

            
            if nums[left as usize] <= nums[mid as usize] { 
                if nums[left as usize] <= target && target < nums[mid as usize] {
                    right = mid - 1; 
                } else {
                    left = mid + 1;  
                }
            } else { 
                if nums[mid as usize] < target && target <= nums[right as usize] {
                    left = mid + 1;  
                } else {
                    right = mid - 1; 
                }
            }
        }

        -1
    }
}