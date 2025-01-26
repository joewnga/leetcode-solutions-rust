/// 
/// Problem: Sort Colors
/// 
/// Given an array `nums` with `n` objects colored **red (0), white (1), or blue (2)**, sort them **in-place** so that objects of the same color are adjacent, with the colors in the order:
/// - `0` (Red)
/// - `1` (White)
/// - `2` (Blue)
///
/// You must **solve this problem without using the built-in `sort()` function.**
///
/// **Example 1:**
/// ```plaintext
/// Input: nums = [2,0,2,1,1,0]
/// Output: [0,0,1,1,2,2]
/// ```
///
/// **Example 2:**
/// ```plaintext
/// Input: nums = [2,0,1]
/// Output: [0,1,2]
/// ```
///
/// **Constraints:**
/// - `n == nums.length`
/// - `1 <= n <= 300`
/// - `nums[i]` is `0`, `1`, or `2`.
///
/// # Solution:
/// - **Time Complexity:** `O(n)`
/// - **Space Complexity:** `O(1)`
impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let (mut left, mut right, mut i) = (0, nums.len() - 1, 0);

        while i <= right {
            match nums[i] {
                0 => {
                    nums.swap(i, left);
                    left += 1;
                    i += 1;
                }
                2 => {
                    nums.swap(i, right);
                    if right == 0 { break; } 
                    right -= 1;
                }
                _ => i += 1,
            }
        }
    }
}