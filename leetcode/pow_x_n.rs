/// 
/// Problem: Pow(x, n)
/// 
/// Implement `pow(x, n)`, which calculates `x` raised to the power `n` (**xⁿ**).
///
/// **Example 1:**
/// ```plaintext
/// Input: x = 2.0, n = 10
/// Output: 1024.0
/// ```
///
/// **Example 2:**
/// ```plaintext
/// Input: x = 2.1, n = 3
/// Output: 9.261
/// ```
///
/// **Example 3:**
/// ```plaintext
/// Input: x = 2.0, n = -2
/// Output: 0.25
/// Explanation: 2⁻² = 1 / (2²) = 1 / 4 = 0.25
/// ```
///
/// **Constraints:**
/// - `-100.0 < x < 100.0`
/// - `-2³¹ <= n <= 2³¹ - 1`
/// - `n` is an integer.
/// - The answer is guaranteed to fit in a **double-precision floating-point** number.
///
/// # Solution:
/// - **Time Complexity:** `O(log n)`
/// - **Space Complexity:** `O(1)`
impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        let mut x = x;
        let mut n = n as i64; 
        if n < 0 {
            x = 1.0 / x;
            n = -n;
        }

        let mut result = 1.0;
        while n > 0 {
            if n % 2 == 1 {
                result *= x;
            }
            x *= x;
            n /= 2;
        }

        result
        
    }
}