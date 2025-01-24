/// 
/// Problem: Climbing Stairs
/// 
/// You are climbing a staircase. It takes `n` steps to reach the top.
/// Each time you can either **climb 1 step or 2 steps**.
/// 
/// Return **the number of distinct ways** you can climb to the top.
///
/// **Example 1:**
/// ```plaintext
/// Input: n = 2
/// Output: 2
/// Explanation: (1 step + 1 step) OR (2 steps at once)
/// ```
///
/// **Example 2:**
/// ```plaintext
/// Input: n = 3
/// Output: 3
/// Explanation: (1+1+1), (1+2), (2+1)
/// ```
///
/// **Constraints:**
/// - `1 <= n <= 45`
///
/// # Solution:
/// /// - **Time Complexity:** `O(n)`
/// - **Space Complexity:** `O(1)`
impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        if n == 1 {
            return 1;
        }
        
        let (mut prev2, mut prev1) = (1, 2);
        for _ in 3..=n {
            let current = prev1 + prev2;
            prev2 = prev1;
            prev1 = current;
        }

        prev1
    }
}