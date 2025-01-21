/// 
/// Problem: Trapping Rain Water
/// 
/// Given an array `height` representing the elevation map where the width of each bar is `1`, 
/// compute how much water can be trapped **after raining**.
///
/// **Example 1:**
/// ```plaintext
/// Input: height = [0,1,0,2,1,0,1,3,2,1,2,1]
/// Output: 6
/// ```
///
/// **Example 2:**
/// ```plaintext
/// Input: height = [4,2,0,3,2,5]
/// Output: 9
/// ```
///
/// **Constraints:**
/// - `1 <= height.length <= 10^5`
/// - `0 <= height[i] <= 10^4`
///
/// # Solution:
///
/// **Time Complexity:** `O(n)`
/// **Space Complexity:** `O(1)`
impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        if height.is_empty() {
            return 0;
        }

        let (mut left, mut right) = (0, height.len() - 1);
        let (mut left_max, mut right_max) = (0, 0);
        let mut water = 0;

        while left < right {
            if height[left] < height[right] {
                if height[left] >= left_max {
                    left_max = height[left];
                } else {
                    water += left_max - height[left];
                }
                left += 1;
            } else {
                if height[right] >= right_max {
                    right_max = height[right];
                } else {
                    water += right_max - height[right];
                }
                right -= 1;
            }
        }

        water
        
    }
}