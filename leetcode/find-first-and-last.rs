/// 
/// Problem: Find First and Last Position of Element in Sorted Array
/// 
/// Given an array of integers `nums` sorted in non-decreasing order, find the starting and ending position of a given `target` value.
/// If `target` is not found, return `[-1, -1]`.
///
/// You must write an algorithm with `O(log n)` runtime complexity.
///
/// Example 1:
/// Input: nums = [5,7,7,8,8,10], target = 8
/// Output: [3,4]
///
/// Example 2:
/// Input: nums = [5,7,7,8,8,10], target = 6
/// Output: [-1,-1]
///
/// Example 3:
/// Input: nums = [], target = 0
/// Output: [-1,-1]
///
/// Constraints:
/// - `0 <= nums.length <= 10^5`
/// - `-10^9 <= nums[i] <= 10^9`
/// - `nums` is sorted in **non-decreasing order**.
///
/// Time Complexity: O(log n)
/// Space Complexity: O(1)
impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        fn binary_search(nums: &Vec<i32>, target: i32, find_first: bool) -> i32 {
            let (mut left, mut right) = (0, nums.len() as isize - 1);
            let mut result = -1;

            while left <= right {
                let mid = left + (right - left) / 2;

                if nums[mid as usize] == target {
                    result = mid as i32;
                    if find_first {
                        right = mid - 1; 
                    } else {
                        left = mid + 1;  
                    }
                } else if nums[mid as usize] < target {
                    left = mid + 1;
                } else {
                    right = mid - 1;
                }
            }

            result
        }

        let first = binary_search(&nums, target, true);
        let last = binary_search(&nums, target, false);

        vec![first, last]
    }
}