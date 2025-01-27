/// 
/// Problem: Largest Rectangle in Histogram
/// 
/// Given an array `heights` representing the heights of bars in a histogram, 
/// return the area of the **largest rectangle** that can be formed.
///
/// **Example 1:**
/// ```plaintext
/// Input: heights = [2,1,5,6,2,3]
/// Output: 10
/// Explanation: The largest rectangle is formed by bars `[5,6]` (area = `5*2=10`).
/// ```
///
/// **Example 2:**
/// ```plaintext
/// Input: heights = [2,4]
/// Output: 4
/// ```
///
/// **Constraints:**
/// - `1 <= heights.length <= 10^5`
/// - `0 <= heights[i] <= 10^4`
///
/// # Solution:
/// - **Time Complexity:** `O(n)`
/// - **Space Complexity:** `O(n)`
impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let mut stack: Vec<usize> = Vec::new();
        let mut max_area = 0;
        let mut heights = heights;
        heights.push(0); 

        for i in 0..heights.len() {
            while let Some(&top) = stack.last() {
                if heights[i] >= heights[top] {
                    break;
                }
                stack.pop();
                let height = heights[top];
                let width = if stack.is_empty() { i } else { i - stack.last().unwrap() - 1 };
                max_area = max_area.max(height * width as i32);
            }
            stack.push(i);
        }

        max_area
    }
}