/// 
/// Problem: Sqrt(x)
/// 
/// Given a **non-negative integer `x`**, compute and return **the square root of `x`**, **rounded down** to the nearest integer.
/// The returned integer **should be non-negative**.
///
/// **You may not use built-in exponent functions (`pow` or `sqrt`).**
///
/// **Example 1:**
/// ```plaintext
/// Input: x = 4
/// Output: 2
/// ```
///
/// **Example 2:**
/// ```plaintext
/// Input: x = 8
/// Output: 2
/// Explanation: The square root of `8` is `2.8284`, so we round down to `2`.
/// ```
///
/// **Constraints:**
/// - `0 <= x <= 2^31 - 1`
///
/// # Solution:
/// - **Time Complexity:** `O(log x)`
/// - **Space Complexity:** `O(1)`
impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        if x < 2 {
            return x;
        }

        let (mut left, mut right) = (1, x / 2);
        while left <= right {
            let mid = left + (right - left) / 2;
            let squared = mid as i64 * mid as i64; 
            
            if squared == x as i64 {
                return mid;
            } else if squared < x as i64 {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }

        right
    }
}