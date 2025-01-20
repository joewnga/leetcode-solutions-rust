/// 
/// Problem: Search Insert Position
/// 
/// Given a sorted array of distinct integers `nums` and a target value, return the **index** where the target is found.
/// If the target is not found, return the **index where it would be inserted** to maintain order.
///
/// You must write an algorithm with `O(log n)` runtime complexity.
///
/// Example 1:
/// Input: nums = [1,3,5,6], target = 5
/// Output: 2
///
/// Example 2:
/// Input: nums = [1,3,5,6], target = 2
/// Output: 1
///
/// Example 3:
/// Input: nums = [1,3,5,6], target = 7
/// Output: 4
///
/// Constraints:
/// - `1 <= nums.length <= 10^4`
/// - `-10^4 <= nums[i] <= 10^4`
/// - `nums` contains **distinct** values sorted in ascending order.
/// - `-10^4 <= target <= 10^4`
///
/// Time Complexity: O(log n)
/// Space Complexity: O(1)
impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let (mut left, mut right) = (0, nums.len() as isize - 1);

        while left <= right {
            let mid = left + (right - left) / 2;
            
            if nums[mid as usize] == target {
                return mid as i32;
            } else if nums[mid as usize] < target {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }

        left as i32
    }
}